use std::borrow::Cow;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use colored::Colorize;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("错误:{}", err);
        process::exit(1);
    });
    println!("查询：{}, 路径：{}", config.query, config.path);
    let file = File::open(&config.path).unwrap_or_else(|err| {
        eprintln!("文件无法打开:{}", err);
        process::exit(1);
    });

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                // 匹配query
                let highlight_style = &config.query.red().to_string();
                let output = if line.contains(&config.query) {
                    Cow::Owned(line.replace(&config.query, highlight_style))
                } else {
                    Cow::Borrowed(&line)
                };
                println!("{}", output)
            }
            Err(err) => {
                eprintln!("读取行失败: {}", err);
                process::exit(1);
            }
        }
    }
}

struct Config {
    query: String,
    path: String,
}

impl Config {
    fn build(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = args.next().ok_or("缺少查询参数")?;
        let path = args.next().ok_or("缺少文件路径参数")?;
        Ok(Config { query, path })
    }
}
