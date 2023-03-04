#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::path::Path;
    use std::fs;

    #[test]
    fn test_readme_tasks() {
        let file_name = "README.md";
        let expected_output = "README.md がありません\n";
        let readme_tasks = "### Task 1\nTask 1 Description\n```echo \"Task 1 Execution\"```\n\n### Task 2\nTask 2 Description\n```echo \"Task 2 Execution\"```\n";

        // Prepare test directory with README.md file
        let dir = tempfile::tempdir().unwrap();
        let readme_path = Path::new(dir.path()).join(file_name);
        fs::write(readme_path.clone(), readme_tasks).unwrap();

        // Test case 1: README.md file does not exist
        fs::remove_file(readme_path.clone()).unwrap();
        let output = Command::new("cargo")
            .args(&["run", "--"])
            .output()
            .unwrap();
        assert_eq!(String::from_utf8_lossy(&output.stdout), expected_output);

        // Test case 2: Task name is not provided
        let expected_output = "Task 1\tTask 1 Description\nTask 2\tTask 2 Description\n";
        let output = Command::new("cargo")
            .args(&["run", "--", readme_path.to_str().unwrap()])
            .output()
            .unwrap();
        assert_eq!(String::from_utf8_lossy(&output.stdout), expected_output);

        // Test case 3: Task name is provided and exists
        let expected_output = "Task 2 Execution\n";
        let output = Command::new("cargo")
            .args(&["run", "--", readme_path.to_str().unwrap(), "Task 2"])
            .output()
            .unwrap();
        assert_eq!(String::from_utf8_lossy(&output.stdout), expected_output);

        // Test case 4: Task name is provided but does not exist
        let expected_output = "Task 3 does not exist\n";
        let output = Command::new("cargo")
            .args(&["run", "--", readme_path.to_str().unwrap(), "Task 3"])
            .output()
            .unwrap();
        assert_eq!(String::from_utf8_lossy(&output.stdout), expected_output);
    }
}
