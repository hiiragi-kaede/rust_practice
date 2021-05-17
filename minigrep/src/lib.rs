use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();//プログラム名はスキップする

        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        //varは環境変数が存在しているかのResultを返すが、今回はエラーハンドリングは必要ないため、is_errでbool値に変換するだけでよい。
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config{query, filename, case_sensitive})
    }
}

///引数で受け取ったconfigに合わせてcontentsの中からqueryが存在するかを調べる。
/// case_sensitiveがtrueなら大文字小文字を区別して調べるが、
/// falseなら区別せずに調べる。
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive{
        search(&config.query, &contents)
    }else{
        search_case_insensitive(&config.query, &contents)
    };
    
    for line in results{
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    // let query = query.to_lowercase();//全て小文字に変換。返り値は文字列スライスではなくString
    // let mut results = Vec::new();

    // for line in contents.lines(){//イテレータを返す
    //     //containsは文字列スライスを取るように定義されているので&stringを渡すようにしている
    //     if line.to_lowercase().contains(&query){
    //         //現在の行がクエリ文字列を含むかを確認する。
    //         //行自体も小文字に変換しているので大文字小文字を区別しなくなっている。
    //         results.push(line);
    //     }
    // }

    // results
    contents.lines()
        .filter(|line|{
            line.to_lowercase().contains(&query.to_lowercase())
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        // Rustは
        // 安全で速く生産性も高い。
        // 3つ選んで。
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:","Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}