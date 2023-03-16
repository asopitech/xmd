use std::path::PathBuf;
use std::process::Command;

fn execute_xmd(task: Option<&str>, file: Option<&str>) -> (String, bool) {
    let mut cmd = Command::new("cargo");
    cmd.arg("run");
    cmd.arg("--");

    if let Some(t) = task {
        cmd.arg(t);
    }

    if let Some(f) = file {
        cmd.arg("-f");
        cmd.arg(f);
    }

    let output = cmd.output().expect("Failed to execute command");
    let output_str = String::from_utf8_lossy(&output.stdout).to_string();

    (output_str, output.status.success())
}

#[test]
fn test_no_readme() {
    let (output, success) = execute_xmd(None, Some("non_existing_file.md"));
    assert!(!success);
    assert_eq!(output.trim(), "README.md がありません");
}

#[test]
fn test_list_tasks() {
    let (output, success) = execute_xmd(None, None);
    assert!(success);
    assert!(output.contains("タスク名1\tヘルプ文1"));
    assert!(output.contains("タスク名2\tヘルプ文2"));
}

#[test]
fn test_execute_task() {
    let (output, success) = execute_xmd(Some("タスク名1"), None);
    assert!(success);
    assert_eq!(output.trim(), "タスク1の実行結果");
}

#[test]
fn test_invalid_task() {
    let (output, success) = execute_xmd(Some("無効なタスク"), None);
    assert!(!success);
    assert_eq!(output.trim(), "無効なタスクは存在しません");
}
