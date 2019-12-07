use std::path::PathBuf;

#[test]
fn test_path_exists() {
    // Create PathBuff
    let mut example_path = PathBuf::new();
    example_path.push("readme.md");
    
    // Path exists
    let example_result = obliterate::file_io::path_exists(&example_path);
    assert_eq!(example_result, true);

    let mut false_example_path = PathBuf::new();
    false_example_path.push("xkcd.md");
    
    // Path doesn't exist
    let false_example_result = obliterate::file_io::path_exists(&false_example_path);
    assert_eq!(false_example_result, false);
}


#[test]
fn test_is_a_file() {
    // Create PathBuff
    let mut example_path = PathBuf::new();
    example_path.push("readme.md");
    
    // It's a file
    let example_result = obliterate::file_io::is_a_file(&example_path);
    assert_eq!(example_result, true);

    let mut false_example_path = PathBuf::new();
    false_example_path.push("src");
    
    // It's a directory
    let false_example_result = obliterate::file_io::is_a_file(&false_example_path);
    assert_eq!(false_example_result, false);

}

