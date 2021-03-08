#![feature(proc_macro_hygiene, decl_macro)]
extern crate inotify;

use core::fmt::Error;
use inotify::{EventMask, Inotify, WatchMask};
use std::env;
use std::fs;
use std::fs::create_dir;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::{thread, time};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, execute_code])
        .launch();
}

#[get("/code")]
fn execute_code() -> String {
    let language = String::from("python");

    let code = String::from("print hello");

    let code_request = CodeRequest { language, code };
    let request = CodeExecutionContainer::new(code_request);
    request.init();
    request.run();

    let (dirname, full_dir_name) = get_dirname("test/result/output.txt");
    println!("{:?} {:?}", &dirname, &full_dir_name);
    let watch = request.watch(&dirname, &full_dir_name);

    let contents =
        fs::read_to_string(full_dir_name).expect("Something went wrong reading the file");

    request.cleanup();
    return String::from(&contents);
}

struct CodeRequest {
    language: String,
    code: String,
}

struct CodeExecutionContainer {
    name: String,
    request: CodeRequest,
    result: String,
}

impl CodeExecutionContainer {
    pub fn new(code_request: CodeRequest) -> CodeExecutionContainer {
        let name = String::from("test");
        let result = String::from("null");
        let request = code_request;
        return CodeExecutionContainer {
            name,
            request,
            result,
        };
    }
    pub fn init(&self) {
        fs::create_dir("test").expect("File exists");
        fs::create_dir("test/result").expect("File exists");
        fs::copy("engine/run.sh", "test/run.sh").unwrap();
        fs::copy("engine/temp/python.py", "test/python.py").unwrap();

        //let mut docker_build = Command::new("docker-compose");
        //let command = docker_build.args(&["build"]).current_dir("engine").spawn().expect("process failed to execute");
    }

    pub fn run(&self) {
        let current_dir = env::current_dir().unwrap();

        let volume = String::from(format!("{:?}/test:code", current_dir));

        let mut docker_run = Command::new("docker-compose");

        let command = docker_run
            .current_dir("engine")
            .args(&["up"])
            .spawn()
            .expect("process failed to execute");
    }

    pub fn watch(&self, dirname: &Path, full_dir_name: &Path) -> i32 {
        let mut ino = Inotify::init().unwrap();
        ino.add_watch(&dirname, WatchMask::DELETE_SELF | WatchMask::CREATE)
            .unwrap();

        if !full_dir_name.exists() {
            loop {
                let mut buffer = [0; 1024];
                let events = ino
                    .read_events_blocking(&mut buffer)
                    .expect("Error while reading events");
                for event in events {
                    match event.name {
                        Some(name) => {
                            if dirname.join(name) == full_dir_name {
                                let five_hunderd_millis = time::Duration::from_millis(500);
                                thread::sleep(five_hunderd_millis);
                                return 0;
                            }
                        }
                        None => {
                            if event.mask == EventMask::DELETE_SELF {
                                eprintln!("The watched directory has been deleted.");
                                return 1;
                            }
                        }
                    }
                }
            }
        }
        return 0;
    }

    pub fn cleanup(&self) {
        fs::remove_dir_all("test").unwrap();
    }
}

fn get_dirname(filename: &str) -> (PathBuf, PathBuf) {
    let path = Path::new(&filename);
    let maybe_dirname = match path.parent() {
        Some(d) if d.is_absolute() => Ok((d.to_path_buf(), path.to_path_buf())),
        Some(relative_path) => match env::current_dir() {
            Ok(cwd) => Ok((cwd.join(relative_path), cwd.join(path))),
            Err(_) => Err((String::from("Current working directory is invalid."), 3)),
        },
        None => Err((format!("Usage:  <filename>"), 1)),
    };
    return maybe_dirname.unwrap();
}
