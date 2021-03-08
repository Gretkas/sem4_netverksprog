#![feature(proc_macro_hygiene, decl_macro)]
extern crate inotify;
extern crate rocket_cors;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate rocket;

pub mod compilers;

use compilers::{LangArray, Language};
use core::fmt::Error;
use inotify::{EventMask, Inotify, WatchMask};
use rocket::http::Method;
use rocket_contrib::json::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use serde::Deserialize;
use std::env;
use std::fs;
use std::fs::create_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::{thread, time};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let default = rocket_cors::CorsOptions::default();

    rocket::ignite()
        .mount("/", routes![index, execute_code, execute_code_posted_code])
        .attach(default.to_cors().unwrap())
        .launch();
}

#[get("/code")]
fn execute_code() -> String {
    let langs = LangArray::init();

    println!("{}", langs.languages[0].language);

    let language = String::from("python");

    let code =
        fs::read_to_string("engine/temp/python.py").expect("Something went wrong reading the file");

    let code_request = CodeRequest { language, code };
    let request = CodeExecutionContainer::new(code_request);
    request.init();

    request.run();

    let (dirname, full_dir_name) = get_dirname("test/result/output.txt");

    let watch = request.watch(&dirname, &full_dir_name);

    let contents =
        fs::read_to_string(full_dir_name).expect("Something went wrong reading the file");

    request.cleanup();
    return String::from(&contents);
}

#[post("/code", format = "application/json", data = "<code_request>")]
fn execute_code_posted_code(code_request: Json<CodeRequest>) -> String {
    let code = code_request.code.to_owned();
    let language = code_request.language.to_owned();
    let code_request = CodeRequest { language, code };

    let request = CodeExecutionContainer::new(code_request);
    request.init();

    request.run();

    let (dirname, full_dir_name) = get_dirname("test/result/output.txt");

    let watch = request.watch(&dirname, &full_dir_name);

    let contents =
        fs::read_to_string(full_dir_name).expect("Something went wrong reading the file");

    request.cleanup();
    return String::from(&contents);
}

#[derive(Deserialize)]
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
        //fs::copy("engine/temp/python.py", "test/python.py").unwrap();

        let mut file = File::create("test/code.py").expect("unable to create file");
        file.write_all(self.request.code.as_bytes()).unwrap();

        //let mut docker_build = Command::new("docker-compose");
        //let command = docker_build.args(&["build"]).current_dir("engine").spawn().expect("process failed to execute");
    }

    pub fn run(&self) {
        let current_dir = env::current_dir().unwrap();

        // let volume = String::from(format!("{:?}/test:code", current_dir));

        let mut docker_run = Command::new("docker-compose");

        docker_run
            .current_dir("engine")
            .args(&["up"])
            .spawn()
            .expect("process failed to execute")
            .wait()
            .expect("Dunno");
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
                                // I had to sleep before to let the compile finish
                                //let five_hunderd_millis = time::Duration::from_millis(500);
                                //thread::sleep(five_hunderd_millis);
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
