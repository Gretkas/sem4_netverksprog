#![feature(proc_macro_hygiene, decl_macro)]
extern crate inotify;
extern crate rocket_cors;
extern crate serde;
#[macro_use]
extern crate rocket;

pub mod compilers;

use compilers::{LangMap, Language};
use inotify::{EventMask, Inotify, WatchMask};
use rocket_contrib::json::Json;
use serde::Deserialize;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

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
    current_language: Language,
}

impl CodeExecutionContainer {
    pub fn new(code_request: CodeRequest) -> CodeExecutionContainer {
        let name = String::from("test");
        let result = String::from("null");
        let request = code_request;
        let lang_map = LangMap::new();
        let map_lang = lang_map.languages.get(&request.language).unwrap();

        let current_language = Language {
            language: map_lang.language.to_owned(),
            compile_args: map_lang.compile_args.clone(),
        };

        return CodeExecutionContainer {
            name,
            request,
            result,
            current_language,
        };
    }
    pub fn init(&self) {
        fs::create_dir("test").expect("File exists");
        fs::create_dir("test/result").expect("File exists");
        fs::copy("run.sh", "test/run.sh").unwrap();

        println!("{}", &self.current_language.compile_args[1]);
        let mut file = File::create(format!("test/{}", self.current_language.compile_args[1]))
            .expect("unable to create file");
        file.write_all(self.request.code.as_bytes()).unwrap();
    }

    pub fn run(&self) {
        let current_dir = env::current_dir().unwrap();

        let volume = String::from(format!("{}/test:/code", current_dir.to_str().unwrap()));

        let mut docker_run = Command::new("docker");

        docker_run
            .args(&[
                "run",
                "-v",
                &volume,
                "sigmundgranaas/code_execution_engine:latest",
                "bash",
                "./run.sh",
                &self.current_language.compile_args[0],
                &self.current_language.compile_args[1],
            ])
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
                                // I had to sleep before to let the compile finish, no longer needed, but kept in case I still need to wait
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
