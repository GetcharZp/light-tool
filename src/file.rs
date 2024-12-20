use std::{fs, io};
use std::fs::OpenOptions;
use std::path::Path;
use std::io::Error;
use std::io::ErrorKind::NotFound;
use std::io::Write;

/// FileCopy: 拷贝文件
///
/// # 参数:
/// - `src_file`: 源文件的绝对路径
/// - `dst_file`: 目标文件的绝对路径
///
/// # Example
/// ```txt
/// use light_tool::file;
/// file::copy("/opt/light-tool/tt.txt", "/opt/light-tool/tt/1/tt.txt").unwrap()
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
/// file::rename("/opt/light-tool/tt.txt", "/opt/light-tool/tt/1/tt.txt").unwrap()
/// ```
pub fn rename(src_file: &str, dst_file: &str) -> io::Result<()> {
    if !Path::new(src_file).exists() {
        return Err(Error::new(NotFound, format!("源文件不存在: {}", src_file)));
    }
    create_parent_dir(dst_file)?;
    fs::rename(src_file, dst_file)?;
    Ok(())
}

/// FileAppend: 文件追加内容
///
/// # 参数:
/// - `file_path`: 文件绝对路径
/// - `content`: 要追加的内容
///
/// # Example
/// ```txt
/// use light_tool::file;
/// use light_tool::random;
/// // 可以通过 \n 换行：format!("\n{}", random::num(6)).as_str()
/// file::append("/opt/light-tool/tt.txt", random::num(6).as_str()).unwrap();
/// ```
pub fn append(file_path: &str, content: &str) -> io::Result<()> {
    create_parent_dir(file_path)?;
    let mut file = OpenOptions::new()
        .write(true).append(true).create(true).open(file_path)?;

    write!(file, "{}", content)?;

    Ok(())
}

/// CreateParentDir: 创建目标文件的父目录
///
/// # Example
/// ```txt
/// use light_tool::file;
/// file::create_parent_dir("/opt/light-tool/tt/1/tt.txt").unwrap()
/// ```
pub fn create_parent_dir(dst_file: &str) -> io::Result<()> {
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
    use crate::random;
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

    #[test]
    fn test_append() {
        if let Err(e) = append("/opt/light-tool/tt.txt", format!("\n{}", random::num(6)).as_str()) {
            println!("append error: {}", e);
        }
    }

    #[test]
    fn test_create_parent_dir() {
        if let Err(e) = create_parent_dir("/opt/light-tool/tt/1/tt.txt") {
            println!("create parent dir error: {}", e);
        }
    }
}
