use std::io::process::Command;

#[test]
fn test_cat_stdin_stdout() {
    let output = match Command::new("sh").arg("-c")
                            .arg("echo 'hello'")
                            .arg("|")
                            .arg("./target/cat")
                            .output() {
        Ok(p) => p,
        Err(e) => panic!("failed to execute process: {}", e),
    };

    assert_eq!(output.output.as_slice(), "hello\n".as_bytes());
}

#[test]
fn test_cat_file_stdout() {
    let output = match Command::new("./target/cat")
                            .arg("tests/fixtures/cat/ascii.txt")
                            .output() {
        Ok(p) => p,
        Err(e) => panic!("failed to execute process: {}", e),
    };

    assert_eq!(output.output.as_slice(), "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ\n".as_bytes());
}

#[test]
fn test_cat_count_file_stdout() {
    let output = match Command::new("./target/cat")
                            .arg("-n")
                            .arg("tests/fixtures/cat/multiline.txt")
                            .output() {
        Ok(p) => p,
        Err(e) => panic!("failed to execute process: {}", e),
    };

    assert_eq!(output.output.as_slice(), "    1  0123456789\n    2  abcdefghijklmnopqrstuvwxyz\n    3  ABCDEFGHIJKLMNOPQRSTUVWXYZ\n".as_bytes());
}
