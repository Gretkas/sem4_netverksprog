pub struct Language {
    pub language: String,
    pub compile_args: Vec<String>,
}

pub struct LangArray {
    pub languages: Vec<Language>,
}

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}
impl LangArray {
    pub fn init() -> LangArray {
        let lang_vec: Vec<Language> = Vec::new();

        let mut langs = LangArray {
            languages: lang_vec,
        };

        let python = Language {
            language: "Python".to_owned(),
            compile_args: vec_of_strings!["python", "file.py", "", "Python", ""],
        };

        let nodejs = Language {
            language: "NodeJS".to_owned(),
            compile_args: vec_of_strings!["nodejs", "file.js", "", "Nodejs", ""],
        };

        let go = Language {
            language: "GO".to_owned(),
            compile_args: vec_of_strings!["\'go run\'", "file.go", "", "Go", ""],
        };

        langs.languages.push(python);
        langs.languages.push(nodejs);
        langs.languages.push(go);
        return langs;
    }
}

// [],
// 			 ["ruby","file.rb","","Ruby",""],
// 			 ["clojure","file.clj","","Clojure",""],
// 			 ["php","file.php","","Php",""],
// 			 ["nodejs","file.js","","Nodejs",""],
// 			 ["scala","file.scala","","Scala",""],
// 			 ["\'go run\'","file.go","","Go",""],
// 			 ["\'g++ -o /usercode/a.out\' ","file.cpp","/usercode/a.out","C/C++",""],
// 			 ["javac","file.java","\'./usercode/javaRunner.sh\'","Java",""],
// 			 ["\'vbnc -nologo -quiet\'","file.vb","\'mono /usercode/file.exe\'","VB.Net",""],
// 			 ["gmcs","file.cs","\'mono /usercode/file.exe\'","C#",""],
// 			 ["/bin/bash","file.sh"," ","Bash",""],
// 			 ["gcc ","file.m"," /usercode/a.out","Objective-C","\' -o /usercode/a.out -I/usr/include/GNUstep -L/usr/lib/GNUstep -lobjc -lgnustep-base -Wall -fconstant-string-class=NSConstantString\'"],
// 			 ["/usercode/sql_runner.sh","file.sql","","MYSQL",""],
// 			 ["perl","file.pl","","Perl",""],
// 			 ["\'env HOME=/opt/rust /opt/rust/.cargo/bin/rustc\'","file.rs","/usercode/a.out","Rust","\'-o /usercode/a.out\'"] ];
