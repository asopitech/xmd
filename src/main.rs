use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file_name = "README.md".to_string();
    let mut task_name = "".to_string();
    
    // 引数の解析
    for (i, arg) in args.iter().skip(1).enumerate() {
        if arg == "-f" && i + 1 < args.len() {
            file_name = args[i + 1].clone();
            break;
        } else if task_name.is_empty() {
            task_name = arg.clone();
        }
    }

    let file_content = match fs::read_to_string(&file_name) {
        Ok(content) => content,
        Err(_) => {
            println!("{} がありません", file_name);
            return;
        }
    };

    let mut tasks: Vec<(String, String, String)> = vec![];
    let mut in_task_list = false;
    let mut current_task_name = String::new();
    let mut current_help_text = String::new();
    let mut current_code_block = String::new();
    let mut in_code_block = false;

    for line in file_content.lines() {
        if line.starts_with("## タスクリスト") {
            in_task_list = true;
            continue;
        }

        if !in_task_list {
            continue;
        }

        if line.starts_with("```") {
            in_code_block = !in_code_block;
            if !in_code_block {
                tasks.push((current_task_name.clone(), current_help_text.clone(), current_code_block.clone()));
                current_task_name.clear();
                current_help_text.clear();
                current_code_block.clear();
            }
            continue;
        }

        if in_code_block {
            current_code_block.push_str(line);
            current_code_block.push('\n');
        } else if line.starts_with("### ") {
            current_task_name = line[4..].to_string();
        } else if !line.is_empty() {
            current_help_text.push_str(line.trim());
            current_help_text.push('\n');
        }
    }

    if task_name.is_empty() {
        for (task_name, help_text, _) in &tasks {
            println!("{}{}", task_name, help_text);
        }
    } else {
        for (name, _, code_block) in tasks {
            if name == task_name {
                let output = Command::new("sh")
                    .arg("-c")
                    .arg(&code_block)
                    .output()
                    .expect("シェルスクリプトの実行に失敗しました");
                println!("{}", String::from_utf8_lossy(&output.stdout));
                break;
            }
        }
    }
}