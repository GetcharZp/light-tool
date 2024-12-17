use std::{fs, io};
use std::path::Path;
use std::io::Error;
use std::io::ErrorKind::NotFound;

/// FileCopy: 拷贝文件
///
/// # 参数:
/// - `src_file`: 源文件的绝对路径
/// - `dst_file`: 目标文件的绝对路径
///
/// # Example
/// ```txt
/// use light_tool::file;
/// file::copy("/opt/light-tool/tt.txt", "/opt/light-tool/tt/1/tt.txt").expect("copy error.")
/// ```
pub fn copy(src_file: &str, dst_file: &str) -> io::Result<()> {
    if !Path::new(src_file).exists() {
        return Err(Error::new(NotFound, format!("源文件不存在: {}", src_file)));
    }
    create_parent_dir(dst_file)?;
    fs::copy(src_file, dst_file)?;
    Ok(())
}

/// FileMove: 移动文件
///
/// # 参数:
/// - `src_file`: 源文件的绝对路径
/// - `dst_file`: 目标文件的绝对路径
///
/// # Example
/// ```txt
/// use light_tool::file;
/// file::rename("/opt/light-tool/tt.txt", "/opt/light-tool/tt/1/tt.txt").expect("file move error.")
/// ```
pub fn rename(src_file: &str, dst_file: &str) -> io::Result<()> {
    if !Path::new(src_file).exists() {
        return Err(Error::new(NotFound, format!("源文件不存在: {}", src_file)));
    }
    create_parent_dir(dst_file)?;
    fs::rename(src_file, dst_file)?;
    Ok(())
}

fn create_parent_dir(dst_file: &str) -> io::Result<()> {
    if let Some(parent_dir) = Path::new(dst_file).parent() {
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir)?;
        }
    } else {
        return Err(Error::new(NotFound, format!("目标文件路径不合法: {}", dst_file)));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_copy() {
        if let Err(e) = copy("/opt/light-tool/tt.txt", "/opt/light-tool/tt/1/tt.txt") {
            println!("file copy error: {}", e);
        }
    }

    #[test]
    fn test_file_move() {
        if let Err(e) = rename("/opt/light-tool/tt.txt", "/opt/light-tool/tt/1/tt.txt") {
            println!("file move error: {}", e);
        }
    }
}
