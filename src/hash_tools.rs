use std::fs::*;
use std::io::{BufReader, BufWriter, ErrorKind};
use std::io::prelude::*;
use std::io;
use sha2::{Sha256, Sha512, Digest};
use hex::*;
use std::cmp::Ordering::Equal;




pub fn hash_password(password: &String, algorithm: &str) -> String {
    match algorithm {
        "SHA256" => {
            let mut hasher = Sha256::new();
            hasher.update(&password);
            let mut hash = hasher.finalize();
            let hashed_text = hex::encode(&hash);
            hashed_text
        },
        "SHA512" => {
            let mut hasher = Sha512::new();
            hasher.update(&password);
            let mut hash = hasher.finalize();
            let hashed_text = hex::encode(&hash);
            hashed_text
        },
        _ => "Something went wrong hashing the password, please try again.".to_string(),
    }
}

pub fn find_matching_hash(password: &String) -> Result<String, &'static str> {
    let hash_file = File::open("_hashed_passwords.txt");
    let mut line_count = 0;
    let mut query_result = match hash_file {
        Ok(file) => file,
        Err(error) => panic!("There was an error finding a reference file."),
    };

    let file_len = &query_result.metadata().unwrap().len();
    let mut reader = BufReader::new(&query_result);
    let mut reader_line = String::new();

    loop {
        let _ = reader.read_line(&mut reader_line);
        line_count += 1;

        if reader_line.contains(password){
            reader_line = reader_line + " on line: " + &line_count.to_string();
            return Ok(reader_line.replace("\n",""));
        }
        else if line_count >= 100000 {
            break Err("Searched many lines...")
        } else {
        reader_line.clear();
        }
    }
    
}


pub fn create_hashed_file (f: &String, algorithm: &str) {

    let mut line_count = 0;
    let file_result = File::open(f);
    let password_file = match file_result {
        Ok(file) => {
            let reader = BufReader::new(&file);
            let file_desc = "_hashed_".to_string();
            let new_file = String::from(file_desc + &f);
            let mut hash_content: String = String::new();

            let mut hash_file_creation  = File::options()
                .create(true)
                .append(true)
                .open(new_file)
                .expect("There was an error creating the hash file.");
            
           
            
            for line in reader.lines().flatten() {
                let hash_pass = hash_password(&line, algorithm);
                let hash_holder = String::from(line + " : " + &hash_pass);
                hash_content = hash_content + &hash_holder + "\n";
                line_count += 1;
            }
            
           match &hash_file_creation {
               AlreadyExists => {
                    let _ = hash_file_creation.write(hash_content.as_bytes());
                    println!("Successfully generated {:?} hashes", line_count);
                },
                
                NotFound => {
                    let _ = hash_file_creation.write(hash_content.as_bytes());
                    println!("Successfully generated {:?} hashes", line_count);
                },
                _ => println!(""),
           } 
        },
        Err(err) => panic!("There was an error locating the file: {:?}", err),
       };

       
    }
