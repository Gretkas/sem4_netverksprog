use std::collections::HashMap;

#[derive(Clone)]
pub struct Language {
    pub language: String,
    pub compile_args: Vec<String>,
}

pub struct LangMap {
    pub languages: HashMap<String, Language>,
}

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}
impl LangMap {
    pub fn new() -> LangMap {
        let lang_map: HashMap<String, Language> = HashMap::new();

        let mut langs = LangMap {
            languages: lang_map,
        };

        let python = Language {
            language: "Python".to_owned(),
            compile_args: vec_of_strings!["python", "file.py", "", "Python", ""],
        };

        let nodejs = Language {
            language: "Nodejs".to_owned(),
            compile_args: vec_of_strings!["nodejs", "file.js", "", "Nodejs", ""],
        };

        let go = Language {
            language: "Go".to_owned(),
            compile_args: vec_of_strings!["go run", "file.go", "", "Go", ""],
        };

        langs
            .languages
            .insert(python.language.clone(), python.clone());
        langs
            .languages
            .insert(nodejs.language.clone(), nodejs.clone());
        langs.languages.insert(go.language.clone(), go.clone());
        return langs;
    }
}
