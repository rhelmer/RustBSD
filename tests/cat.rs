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

    println!("{}", output.status);
    assert_eq!(output.output.as_slice(), "hello\n".as_bytes());
}

