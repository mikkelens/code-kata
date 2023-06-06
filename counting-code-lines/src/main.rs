use std::env;

pub mod code_analysis;

fn main() {
    let dir = env::current_dir().unwrap();
    println!("DIR: {:?}", dir);

    let all_args: Vec<String> = env::args().collect();
    let arg_str = all_args.join(", ");
    println!("ARGS: {}", arg_str);

    let mut arg_iter = all_args.into_iter().skip(1); // skip executable path
    let Some(target_str) = arg_iter.next() else {
        panic!("Did not provide a target.")
    };
    println!("Target: {target_str}");
    // let targets = 
}