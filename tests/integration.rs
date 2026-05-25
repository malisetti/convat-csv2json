use std::io::Write;
use std::process::{Command, Stdio};

const SAMPLE_CSV: &str = "name,age,city\nAlice,30,NYC\nBob,25,LA\n";

#[test]
fn stdin_csv_with_header_writes_json_lines_to_stdout() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_convat-csv2json"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn convat-csv2json");

    {
        let stdin = child.stdin.as_mut().expect("stdin");
        stdin.write_all(SAMPLE_CSV.as_bytes()).expect("write csv");
    }

    let output = child.wait_with_output().expect("wait");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], r#"{"age":"30","city":"NYC","name":"Alice"}"#);
    assert_eq!(lines[1], r#"{"age":"25","city":"LA","name":"Bob"}"#);
}
