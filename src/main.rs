use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        std::process::exit(1);
    }

    println!("define i32 @main(i32 %argc, i8** %argv) #0 {{");
    println!("  ret i32 {}", args[1]);
    println!("}}");
    return;
}
