
use std::{error::Error, fs::{File}, io::{BufReader, BufRead}};

/// rgrep
/// Usage: rgrep -find=Text/regexp-text -path=regexp-filename
/// 
/// 解析命令行参数 解析到结构体
/// 循环匹配文本
/// 匹配到了就输出

use clap::Parser;
use colored::Colorize;
use regex::Regex;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// 待查找的内容
    // #[clap(short, long)]
    find: String,

    /// 要查找的路径
    // #[clap(short, long)]
    glob: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    println!("args: {:?}", args);

    //获取文件列表
    let paths: Vec<_> = glob::glob(args.glob.as_str())?.collect();
    println!("paths len = {}", paths.len());
    let re = Regex::new(args.find.as_str())?;

    //循环依次匹配每个文件
    for path in paths {
        if let Err(e) = path {
            return Err(Box::new(e))
        }
        let path = path.unwrap();
        let file = File::open(path.as_path())?;
        let reader = BufReader::new(file);

        let mut path_printed = false;
        
        //循环依次匹配每一行
        for (lineno, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            if let Some(caps) = re.captures(line.as_str()) {
                if let Some(m) = caps.get(0) {
                    if !path_printed {
                        println!("{}:",path.display().to_string().green());
                        path_printed = true;
                    }

                    let start = m.start();
                    let end = m.end();
                    println!("{0: >6}:{1: <3} {2}{3}{4}",
                        (lineno+1).to_string().green(),
                        (start+1).to_string().green(),
                        &line[0..start],
                        &line[start..end].red(),
                        &line[end..],
                    )
                }
            }
        }

        //当打印完成后输出空行
        if path_printed {
            println!("");
        }
    }

    Ok(())
}