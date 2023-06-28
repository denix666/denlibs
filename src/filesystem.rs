/// Returns the full path of the application, from where it was executed.
/// 
/// # Example
/// ```rust
/// use denlibs::filesystem::executable_path;
/// 
/// println!("Application was executed from: {}", executable_path().unwrap());
/// ```
pub fn executable_path() -> Result<String, Box<dyn std::error::Error>> {
    if let Some(dir) = std::env::current_exe().unwrap().parent().unwrap().to_str() {
        return Ok(dir.to_string());
    } else {
        return Err("Unable to get the path.".into())
    }
}

#[cfg(target_family = "unix")]
/// Returns true if the binary is in Executable and Linkable Format (ELF) or PIE
/// 
/// # Example
/// ```rust
/// use denlibs::filesystem::is_elf;
/// 
/// let path_buf = std::path::PathBuf::from("/bin/bash");
/// let path = path_buf.as_os_str();
/// 
/// if is_elf(path) {
///     println!("File is ELF: {:?}", path);
/// }
/// ```
pub fn is_elf(path: &std::ffi::OsStr) -> bool {
    const ELF: &[u8; 4] = &[0x7f, 0x45, 0x4c, 0x46];
    let mut ident = [0; 4];

    match std::fs::File::open(path) {
        Ok(mut f) => {
            match std::io::Read::read(&mut f, &mut ident) {
                Ok(_) => {
                    if &ident == ELF {
                        return true
                    }
                },
                Err(e) => {
                    println!("{}", e);
                    return false
                },
            }
        },
        Err(e) => {
            println!("{}", e);
            return false
        }
    }
    return false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn executable_path_test() {
        assert!(executable_path().is_ok());
    }

    #[cfg(target_family = "unix")]
    #[test]
    fn is_elf_test() {
        let path_buf = std::path::PathBuf::from("/bin/bash");
        let path = path_buf.as_os_str();
        assert!(is_elf(path));
    }
}
