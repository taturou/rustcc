extern crate combine;

use std::env;
use combine::parser::char::{spaces, digit};
use combine::{many1, Parser};
use combine::stream::easy;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        std::process::exit(1);
    }

    let exp = &args[1];

    let mut integer = spaces()
                    .with(many1(digit()).map(|str: String| str.parse::<i32>().unwrap()));
    let result: Result<(i32, &str), easy::ParseError<&str>> = integer.easy_parse(exp);

    match result {
        Ok((value, "")) => {
            println!("define i32 @main(i32 %argc, i8** %argv) #0 {{");
            println!("  ret i32 {}", value);
            println!("}}");
        },
        Ok((value, remain)) => {
            eprintln!("\"{}\" の後の \"{}\" を認識できません。", value, remain);
        },
        Err(err) => {
            eprintln!("{}", err);
        }
    }
    return;
}
