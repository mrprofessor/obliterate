// TODO
// Change variable names while refactoring
pub mod file_io {
    use std::path::Path;
    use std::fs::metadata;
    use std::fs::remove_file;

    pub fn path_exists(path: &std::path::PathBuf) -> bool {
        let exists = Path::new(path).exists();
        return exists;
    }

    pub fn is_a_file(path: &std::path::PathBuf) -> bool {
        let path_metadata = metadata(path).unwrap();
        return path_metadata.is_file();
    }

   pub fn delete_file(path: &std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        remove_file(path)?;
        Ok(())
   }
}
