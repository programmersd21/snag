use std::io::Write;
use std::process::{Command, Stdio};

/// Helper: run `snag` with a given target and stdin input, return (stdout, stderr, exit_code).
fn run_snag(target: &str, input: &[u8]) -> (String, String, i32) {
    let bin = env!("CARGO_BIN_EXE_snag");
    let mut child = Command::new(bin)
        .arg(target)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn snag");

    if let Some(ref mut stdin) = child.stdin {
        let _ = stdin.write_all(input);
    }
    drop(child.stdin.take());

    let output = child.wait_with_output().expect("failed to wait");
    (
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
        output.status.code().unwrap_or(-1),
    )
}

/// Helper: run `snag` with no arguments.
fn run_snag_no_args() -> (String, String, i32) {
    let bin = env!("CARGO_BIN_EXE_snag");
    let output = Command::new(bin)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("failed to spawn snag");
    (
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
        output.status.code().unwrap_or(-1),
    )
}

/// Helper: run `snag` with a flag only, no stdin.
fn run_snag_flag(flag: &str) -> (String, String, i32) {
    let bin = env!("CARGO_BIN_EXE_snag");
    let output = Command::new(bin)
        .arg(flag)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("failed to spawn snag");
    (
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
        output.status.code().unwrap_or(-1),
    )
}

#[test]
fn test_01_builtin_ip() {
    let (out, _, code) = run_snag("ip", b"addr 10.0.0.1 fake 999.999.999.999\n");
    assert_eq!(code, 0);
    assert_eq!(out.trim(), "10.0.0.1");
}

#[test]
fn test_01_builtin_url() {
    let (out, _, code) = run_snag("url", b"see https://example.com and ftp://other.com\n");
    assert_eq!(code, 0);
    assert_eq!(out.trim(), "https://example.com");
}

#[test]
fn test_01_builtin_num() {
    let (out, _, code) = run_snag("num", b"count 42 and 3.14\n");
    assert_eq!(code, 0);
    assert!(out.contains("42"));
}

#[test]
fn test_01_builtin_hash() {
    let (out, _, code) = run_snag("hash", b"commit abc1234 short abc12 nothex zzzzzzzz\n");
    assert_eq!(code, 0);
    assert!(out.contains("abc1234"));
    assert!(!out.contains("abc12\n"));
}

#[test]
fn test_01_builtin_path() {
    let (out, _, code) = run_snag("path", b"file /usr/bin/ls and relative foo/bar\n");
    assert_eq!(code, 0);
    assert!(out.contains("/usr/bin/ls"));
    assert!(!out.contains("foo/bar"));
}

#[test]
fn test_01_builtin_email() {
    let (out, _, code) = run_snag("email", b"contact user@example.com and user@\n");
    assert_eq!(code, 0);
    assert_eq!(out.trim(), "user@example.com");
}

#[test]
fn test_02_ip_multiple_matches() {
    let (out, _, code) = run_snag("ip", b"from 10.0.0.1 to 192.168.1.1\n");
    assert_eq!(code, 0);
    let lines: Vec<&str> = out.trim().lines().collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "10.0.0.1");
    assert_eq!(lines[1], "192.168.1.1");
}

#[test]
fn test_03_column_out_of_range() {
    let (out, _, code) = run_snag(":5", b"a:b:c\n");
    assert_eq!(code, 0);
    assert_eq!(out, "");
}

#[test]
fn test_04_column_no_delimiter() {
    let (out, _, code) = run_snag(":1", b"hello world\n");
    assert_eq!(code, 0);
    assert_eq!(out.trim(), "hello world");
}

#[test]
fn test_05_template_metachar() {
    let (out, _, code) = run_snag("port=8080.{}$", b"port=8080.data$ end\n");
    assert_eq!(code, 0);
    assert_eq!(out.trim(), "data");
}

#[test]
fn test_06_json_mixed_lines() {
    let input = br#"{"name":"alice"}
not json at all
{"age":30}
"#;
    let (out, _, code) = run_snag(".name", input);
    assert_eq!(code, 0);
    assert_eq!(out.trim(), "alice");
}

#[test]
fn test_07_json_nested_object() {
    let input = br#"{"data":{"x":1,"y":2}}"#;
    let (out, _, code) = run_snag(".data", input);
    assert_eq!(code, 0);
    let trimmed = out.trim();
    assert!(trimmed.contains("\"x\":1"));
    assert!(trimmed.contains("\"y\":2"));
}

#[test]
fn test_08_empty_stdin() {
    let (out, _, code) = run_snag("ip", b"");
    assert_eq!(code, 0);
    assert_eq!(out, "");
}

#[test]
fn test_09_invalid_regex() {
    let (out, err, code) = run_snag("(unbalanced", b"anything\n");
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert!(
        err.to_lowercase().contains("invalid"),
        "stderr should contain 'invalid': {}",
        err
    );
}

#[test]
fn test_10_regex_empty_capture() {
    let (out, _, code) = run_snag("a(x*)b", b"ab\n");
    assert_eq!(code, 0);
    assert_eq!(out, "\n");
}

#[test]
fn test_11_no_argument() {
    let (_, err, code) = run_snag_no_args();
    assert_eq!(code, 1);
    assert!(!err.is_empty(), "stderr should contain usage message");
}

#[test]
fn test_12_help_flag() {
    let (out, _, code) = run_snag_flag("--help");
    assert_eq!(code, 0);
    assert!(out.contains("snag"));
}

#[test]
fn test_12_short_help_flag() {
    let (out, _, code) = run_snag_flag("-h");
    assert_eq!(code, 0);
    assert!(out.contains("snag"));
}

#[test]
fn test_12_version_flag() {
    let (out, _, code) = run_snag_flag("--version");
    assert_eq!(code, 0);
    assert!(out.contains("0.1.0"));
}

#[test]
fn test_12_short_version_flag() {
    let (out, _, code) = run_snag_flag("-V");
    assert_eq!(code, 0);
    assert!(out.contains("0.1.0"));
}

#[test]
fn test_13_invalid_utf8() {
    let mut input: Vec<u8> = Vec::new();
    input.extend_from_slice(b"addr 10.0.0.1\n");
    input.extend_from_slice(b"bad \xFF\xFE bytes 192.168.0.1\n");
    input.extend_from_slice(b"addr 172.16.0.1\n");

    let (out, _, code) = run_snag("ip", &input);
    assert_eq!(code, 0);
    assert!(out.contains("10.0.0.1"));
    assert!(out.contains("172.16.0.1"));
    assert!(out.contains("192.168.0.1"));
}

#[test]
fn test_14_broken_pipe() {
    let bin = env!("CARGO_BIN_EXE_snag");

    // Generate many lines of input with numbers.
    let mut input = String::new();
    for i in 0..10000 {
        input.push_str(&format!("line {} here\n", i));
    }

    // Pipe snag's output into `head -n 1` to force a broken pipe.
    let mut snag_child = Command::new(bin)
        .arg("num")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn snag");

    let head_child = Command::new("head")
        .arg("-n")
        .arg("1")
        .stdin(snag_child.stdout.take().expect("no stdout"))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn head");

    // Write all input to snag's stdin.
    if let Some(ref mut stdin) = snag_child.stdin {
        // Ignore write errors — snag may have already exited.
        let _ = stdin.write_all(input.as_bytes());
    }
    drop(snag_child.stdin.take());

    let head_output = head_child.wait_with_output().expect("head failed");
    let snag_status = snag_child.wait().expect("snag failed to exit");

    // head should have captured the first line.
    let head_stdout = String::from_utf8_lossy(&head_output.stdout);
    assert!(
        head_stdout.contains("0"),
        "head should have captured first number"
    );

    // snag should exit cleanly (code 0), not crash.
    let snag_code = snag_status.code().unwrap_or(-1);
    assert!(
        snag_code == 0 || snag_code == 141, // 141 = 128+13 (SIGPIPE) is also acceptable
        "snag should exit cleanly on broken pipe, got code {}",
        snag_code
    );
}
