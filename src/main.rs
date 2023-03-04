use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = match args.get(2) {
        Some(file) => file,
        None => "README.md",
    };
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("{} がありません", filename);
            return;
        }
    };
    let reader = BufReader::new(file);
    let mut tasks: Vec<(String, String, String)> = Vec::new();
    let mut current_task: Option<(String, String, String)> = None;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("###") {
            if let Some(task) = current_task {
                tasks.push(task);
            }
            let task_name = line[4..].trim().to_owned();
            current_task = Some((task_name, String::new(), String::new()));
        } else if let Some(task) = current_task.as_mut() {
            if line.starts_with("```") {
                let code_block = task.2.clone();
                task.1.push_str(&format!("{}\t{}\n", task.0, code_block));
                task.2.clear();
            } else {
                task.2.push_str(&format!("{}\n", line));
            }
        }
    }
    if let Some(task) = current_task {
        tasks.push(task);
    }
    if let Some(task_name) = args.get(1) {
        if let Some(task) = tasks.iter().find(|(name, _, _)| name == task_name) {
            let (_, _, code_block) = task;
            println!("{}", code_block.trim());
        } else {
            eprintln!("{} というタスクはありません", task_name);
            return;
        }
    } else {
        for (name, help, _) in tasks {
            println!("{}\t{}", name, help.trim());
        }
    }
}