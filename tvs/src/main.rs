use rustylib::{colored, filter_path_w_ext};
use std::collections::HashMap;
use std::env;
use std::fs::rename;
use std::io::{self, Write};
use std::process::exit;

fn main() {
    // Parse commandline arguments
    let cmd_args: Vec<String> = env::args().collect();

    // Init start index
    let mut start_idx: u32 = 1;
    match cmd_args.len() {
        1 => {
            println!("Start number is needed!!!");
            exit(1)
        }
        2 => match cmd_args[1].parse::<u32>() {
            Ok(i) => start_idx = i,
            Err(_) => {
                println!("Start index not a NUMBER!!!");
                exit(1)
            }
        },
        _ => {}
    }

    // Filter paths by type and extention
    let extentions: Vec<&str> = vec!["ts", "mkv", "mp4", "avi"];
    let str_prefix = "Серія";

    let mut output = filter_path_w_ext("./", &extentions);
    output.sort_by_key(|f| f.to_lowercase());

    let mut stash = HashMap::new();
    output.iter().for_each(|f| {
        let ext = f.rsplit_once(".").expect("Unexpected file name").1;
        let r_name = format!("{str_prefix} {:0>2}.{ext}", start_idx);
        println!(
            "{} {} {}",
            colored(96, f),
            colored(33, "=>"),
            colored(32, &r_name)
        );
        stash.insert(f, r_name);
        start_idx += 1
    });

    print!(
        "{}",
        colored(35, "Do you really want to rename? [N]/Y/y/yes ")
    );
    let _ = io::stdout().flush();
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failes to read from STDIN");
    let trimmed = user_input.trim();
    match trimmed.to_lowercase().as_str() {
        "y" | "yes" => {
            for (k, v) in stash.into_iter() {
                rename(k, v).expect("Error renaming file")
            }
        }
        _ => println!("{}", colored(31, "Abort!!!")),
    }
}
