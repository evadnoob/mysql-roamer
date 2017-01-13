extern crate readline;

use std::process;
use schemas;
use tables;

pub fn start() {
    
    let url = "mysql://david:david@172.17.0.2:3306/test";
    loop {
        match readline::readline(">>> ") {
            Ok(input) => {
                let input = input.replace("\n", "");
                if input.len() > 0 {
                    readline::add_history(input.as_ref());
                    println!("{:?}", input);
                    if "help" == input {
                      println!("help comes later....");
                    }
                    else if "\\dt" == input {
                        tables::all(url);
                    }
                    else if "\\dn" == input {
                        schemas::all(url);
                    }
                    else if "exit" == input || "quit" == input {
                        process::exit(0);
                    }
                }
            },
            Err(e) => {
                println!("{}", e);
                //panic!("{}", e);
            }
        }
        
    }
}

