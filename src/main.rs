use std::io::prelude::*;
use std::io;
use std::env;



pub mod hash_tools;




fn main() {
   let tool_chain_args: Vec<String> = env::args().collect();
   println!("{:#?}", tool_chain_args);
   input_handler(&tool_chain_args);
}


fn input_handler (args: &[String]) {
    if args.len() < 2 {
        panic!("Invalid argument length, please follow the guidelines to use this functionality.")
    }

    match &args[1][..] {
        "--hash" => println!("[password] = {}\n[hash] = {}", &args[2], hash_tools::hash_password(&args[2], &args[3])),
        "--search" => println!("[Collision found] = {} ", hash_tools::find_matching_hash(&args[2]).expect("Couldnt find a matching hash").replace("\n","")),
        "--create-hash-file" => hash_tools::create_hashed_file(&args[2], &args[3]),
        _ => println!("What did you want??"),
    }
}
