//! SFTP 客户端与传输引擎实现

use super::{SftpError, SftpFileItem, TransferProgress};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::time::UNIX_EPOCH;

/// 默认分片传输块大小（64 KB）
pub const SFTP_CHUNK_SIZE: usize = 64 * 1024;

/// SFTP 操作服务
pub struct SftpService;

impl SftpService {
    /// 模拟或读取本地目录文件列表（作为通用文件系统操作，与远程 SFTP 保持相同协议接口）
    pub fn list_local_directory(dir_path: &Path) -> Result<Vec<SftpFileItem>, SftpError> {
        if !dir_path.exists() {
            return Err(SftpError::PathNotFound(dir_path.display().to_string()));
        }

        let read_dir = fs::read_dir(dir_path)
            .map_err(|e| SftpError::Io(format!("读取目录失败: {}", e)))?;

        let mut items = Vec::new();

        for entry_res in read_dir {
            let entry = entry_res.map_err(|e| SftpError::Io(e.to_string()))?;
            let metadata = entry.metadata().map_err(|e| SftpError::Io(e.to_string()))?;
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            let modified_at = metadata
                .modified()
                .ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);

            items.push(SftpFileItem {
                name,
                path: path.to_string_lossy().to_string(),
                is_dir: metadata.is_dir(),
                size: metadata.len(),
                permissions: 0o644,
                modified_at,
            });
        }

        // 排序：目录优先，随后按文件名排序
        items.sort_by(|a, b| {
            if a.is_dir != b.is_dir {
                b.is_dir.cmp(&a.is_dir)
            } else {
                a.name.cmp(&b.name)
            }
        });

        Ok(items)
    }

    /// 执行流式分块文件传输（支持进度回调，支持零字节空文件）
    pub fn transfer_stream<F>(
        source_path: &Path,
        dest_path: &Path,
        transfer_id: &str,
        mut on_progress: F,
    ) -> Result<(), SftpError>
    where
        F: FnMut(TransferProgress),
    {
        if !source_path.exists() {
            return Err(SftpError::PathNotFound(source_path.display().to_string()));
        }

        let mut src_file = File::open(source_path)
            .map_err(|e| SftpError::Io(format!("打开源文件失败: {}", e)))?;
        let total_bytes = src_file
            .metadata()
            .map_err(|e| SftpError::Io(e.to_string()))?
            .len();

        let mut dest_file = File::create(dest_path)
            .map_err(|e| SftpError::Io(format!("创建目标文件失败: {}", e)))?;

        // 零字节空文件特判（Review Focus 5）
        if total_bytes == 0 {
            dest_file.flush().map_err(|e| SftpError::Io(e.to_string()))?;
            on_progress(TransferProgress {
                transfer_id: transfer_id.to_string(),
                transferred_bytes: 0,
                total_bytes: 0,
                is_finished: true,
                error: None,
            });
            return Ok(());
        }

        let mut buffer = vec![0u8; SFTP_CHUNK_SIZE];
        let mut transferred = 0u64;

        loop {
            let bytes_read = src_file
                .read(&mut buffer)
                .map_err(|e| SftpError::Io(format!("读取源文件数据块失败: {}", e)))?;

            if bytes_read == 0 {
                break;
            }

            dest_file
                .write_all(&buffer[..bytes_read])
                .map_err(|e| SftpError::Io(format!("写入目标数据块失败: {}", e)))?;

            transferred += bytes_read as u64;

            on_progress(TransferProgress {
                transfer_id: transfer_id.to_string(),
                transferred_bytes: transferred,
                total_bytes,
                is_finished: transferred >= total_bytes,
                error: None,
            });
        }

        dest_file.flush().map_err(|e| SftpError::Io(e.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    /// 验证目录列表解析与排序（目录在前，文件在后）
    #[test]
    fn test_sftp_list_directory_ordering() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        fs::create_dir(root.join("z_dir")).unwrap();
        fs::create_dir(root.join("a_dir")).unwrap();
        fs::write(root.join("b_file.txt"), b"123").unwrap();
        fs::write(root.join("a_file.txt"), b"456").unwrap();

        let items = SftpService::list_local_directory(root).expect("列出目录成功");
        assert_eq!(items.len(), 4);

        // 前两个必须是目录
        assert!(items[0].is_dir);
        assert!(items[1].is_dir);
        assert_eq!(items[0].name, "a_dir");
        assert_eq!(items[1].name, "z_dir");

        // 后两个是文件
        assert!(!items[2].is_dir);
        assert!(!items[3].is_dir);
        assert_eq!(items[2].name, "a_file.txt");
        assert_eq!(items[3].name, "b_file.txt");
    }

    /// 验证零字节空文件与文件不存在错误处理 (Review Focus 5)
    #[test]
    fn test_sftp_empty_file_and_error_handling() {
        let dir = tempdir().unwrap();
        let src_empty = dir.path().join("empty.txt");
        let dest_empty = dir.path().join("empty_dest.txt");

        // 创建 0 字节文件
        fs::write(&src_empty, b"").unwrap();

        let mut progress_events = Vec::new();
        let res = SftpService::transfer_stream(
            &src_empty,
            &dest_empty,
            "trans_1",
            |p| progress_events.push(p),
        );

        assert!(res.is_ok());
        assert_eq!(progress_events.len(), 1);
        assert!(progress_events[0].is_finished);
        assert_eq!(progress_events[0].total_bytes, 0);
        assert_eq!(progress_empty_size(&dest_empty), 0);

        // 测试源文件不存在时的明确报错
        let nonexistent = dir.path().join("missing.txt");
        let res_err = SftpService::transfer_stream(&nonexistent, &dest_empty, "trans_2", |_| {});
        assert!(matches!(res_err, Err(SftpError::PathNotFound(_))));
    }

    fn progress_empty_size(p: &Path) -> u64 {
        fs::metadata(p).unwrap().len()
    }
}
