use std::{env, io::Write};

use clap::{
    builder::{styling, Styles},
    Args, Parser,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;

#[derive(Parser, Debug)]
#[command(
    version,
    next_display_order = None,
    help_template = "\
{before-help}{name} {version}

{usage-heading} {usage}

{all-args}{after-help}",
    styles = Styles::styled()
        .header(styling::AnsiColor::Yellow.on_default())
        .usage(styling::AnsiColor::Yellow.on_default())
        .literal(styling::AnsiColor::Green.on_default())
)]
enum Command {
    #[command(name = "a")]
    /// Adds the todo item
    Add(AddOptions),
    #[command(name = "r")]
    /// Removes the todo item
    Remove(RemoveOptions),
    #[command(name = "d")]
    /// Marks the todo item as done
    Done(DoneOptions),
    #[command(name = "l")]
    /// Marks the todo item as done
    List,
}

#[derive(Args, Clone, Debug)]
struct AddOptions {
    /// Id of the todo item
    item: String,
}

#[derive(Args, Clone, Debug)]
struct RemoveOptions {
    /// Bucket to remove from (t for todo, d for done)
    bucket: String,
    /// Id of the todo item
    id: usize,
}

#[derive(Args, Clone, Debug)]
struct DoneOptions {
    /// Id of the todo item
    id: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    item: String,
    done: bool,
}

fn main() {
    let dir = env::current_dir().expect("Not able to access the current directory");
    let file_name = "t.json";
    let file_path = dir.to_str().unwrap().to_owned() + "/" + file_name;
    if fs::metadata(&file_path).is_err() {
        _ = File::create(&file_path);
    }

    let _ = match Command::parse() {
        Command::Add(options) => add(file_path, options.item),
        Command::Remove(options) => {
            remove(std::io::stdout(), file_path, options.bucket, options.id)
        }
        Command::Done(options) => done(std::io::stdout(), file_path, options.id),
        Command::List => list(std::io::stdout(), file_path),
    };
}

fn add(file_path: String, item: String) {
    let file = get_file(file_path.clone());
    let mut todo_list = get_todo_list(file_path);
    let todo = Todo { item, done: false };
    todo_list.push(todo);
    save_todo_list(file, todo_list);
}

fn list<W: std::io::Write>(buffer: W, file_path: String) {
    let todo_list = get_todo_list(file_path);
    print(buffer, todo_list);
}

fn remove<W: std::io::Write>(mut buffer: W, file_path: String, bucket: String, id: usize) {
    let is_done = match bucket.as_str() {
        "t" => Some(false),
        "d" => Some(true),
        _ => None,
    };
    if let Some(done) = is_done {
        let todo_list = get_todo_list(file_path.clone());
        let todo = todo_list.iter().filter(|item| item.done == done);
        if id > todo.clone().count() || id < 1 {
            writeln!(buffer, "Incorrect `id`").expect("Error writing to buffer");
        } else {
            let mut updated_todo_list: Vec<Todo> = Vec::new();
            let mut i = 1;
            for todo in todo_list {
                if todo.done == done {
                    if i != id {
                        updated_todo_list.push(todo);
                    }
                    i += 1;
                } else {
                    updated_todo_list.push(todo);
                }
            }
            let file = get_file(file_path);
            save_todo_list(file, updated_todo_list);
        }
    } else {
        writeln!(
            buffer,
            "Incorrect bucket. Use `t` for todo bucket, `d` for done bucket."
        )
        .expect("Error writing to buffer");
    }
}

fn done<W: std::io::Write>(mut buffer: W, file_path: String, id: usize) {
    let todo_list = get_todo_list(file_path.clone());
    let todo = todo_list.iter().filter(|item| !item.done);
    if id > todo.clone().count() || id < 1 {
        writeln!(buffer, "Incorrect `id`").expect("Error writing to buffer");
    } else {
        let mut updated_todo_list: Vec<Todo> = Vec::new();
        let mut i = 1;
        for todo in todo_list {
            if !todo.done {
                if i == id {
                    updated_todo_list.push(Todo {
                        item: todo.item,
                        done: true,
                    });
                } else {
                    updated_todo_list.push(todo);
                }
                i += 1;
            } else {
                updated_todo_list.push(todo);
            }
        }
        let file = get_file(file_path);
        save_todo_list(file, updated_todo_list);
    }
}

fn print<W: std::io::Write>(mut buffer: W, todo_list: Vec<Todo>) {
    let todo = todo_list.iter().filter(|item| !item.done);
    if todo.clone().count() > 0 {
        writeln!(buffer, "Todo:").expect("Error writing to buffer");
        writeln!(buffer, "-----").expect("Error writing to buffer");
        for (i, item) in todo.enumerate() {
            writeln!(buffer, "{0}. {1}", i + 1, item.item).expect("Error writing to buffer");
        }
        writeln!(buffer, "").expect("Error writing to buffer");
    }

    let done = todo_list.iter().filter(|item| item.done);
    if done.clone().count() > 0 {
        writeln!(buffer, "Done:").expect("Error writing to buffer");
        writeln!(buffer, "-----").expect("Error writing to buffer");
        for (i, item) in done.enumerate() {
            writeln!(buffer, "{0}. {1}", i + 1, item.item).expect("Error writing to buffer");
        }
    }
}

fn get_file(file_path: String) -> File {
    let file = File::options()
        .write(true)
        .open(file_path)
        .expect("Not able to open the file");
    file
}

fn get_todo_list(file_path: String) -> Vec<Todo> {
    let contents = fs::read_to_string(file_path).expect("Error reading contents of the file");
    if contents.is_empty() {
        return Vec::new();
    }
    let todo_list: Vec<Todo> =
        serde_json::from_str(contents.as_str()).expect("Error parsing the file content");
    todo_list
}

fn save_todo_list(mut file: File, todo_list: Vec<Todo>) {
    let todo_list_str = serde_json::to_string(&todo_list).expect("Error formatting the todo list");
    _ = file.set_len(0);
    _ = file.write_all(todo_list_str.as_bytes());
}
