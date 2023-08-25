use clap::{arg, Command};
use std::fs::OpenOptions;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::process::Command as Cmd;

fn main() {
    let matches = Command::new("bookmark-paths")
        .version("1.0")
        .author("Stefanos Kouroupis")
        .about("Bookmark paths and Navigates through them")
        .subcommand(Command::new("add").about("Add a new path"))
        .subcommand(
            Command::new("remove")
                .about("Remove a new path")
                .arg(arg!(<INDEX>).required(true)),
        )
        .subcommand(Command::new("list").about("List all paths"))
        .subcommand(
            Command::new("go")
                .about("Go to a path")
                .arg(arg!(<INDEX>).required(true)),
        )
        .get_matches();

    let file = format!(
        "{}{}",
        dirs::home_dir().unwrap().display(),
        "/.config/.bookmark-paths"
    );

    if let Some(_matches) = matches.subcommand_matches("add") {
        let mut ffw = get_file_db(false, &file);
        let out = Cmd::new("pwd")
            .output()
            .expect("pwd command failed to start");

        let result = String::from_utf8_lossy(&out.stdout);
        let reader = BufReader::new(&ffw);

        let mut exists = false;
        for line in reader.lines() {
            if line.unwrap() == result {
                exists = true;
            }
        }

        if !exists {
            ffw.write_all(result.as_bytes()).unwrap();
        }
    }

    if let Some(matches) = matches.subcommand_matches("remove") {
        let ffw = get_file_db(false, &file);
        let reader = BufReader::new(&ffw);

        let mut keep: Vec<String> = vec![];
        for (i, line) in reader.lines().enumerate() {
            if *matches.get_one::<String>("INDEX").unwrap() != (i + 1).to_string() {
                keep.push(line.unwrap());
            }
        }

        let mut fft = get_file_db(true, &file);
        for line in keep {
            fft.write_all(line.as_bytes()).unwrap();
        }
    }

    if let Some(_matches) = matches.subcommand_matches("list") {
        let ffw = get_file_db(false, &file);
        let reader = BufReader::new(&ffw);

        for (i, line) in reader.lines().enumerate() {
            println!("{}: {}", i + 1, line.unwrap());
        }
    }

    if let Some(matches) = matches.subcommand_matches("go") {
        let ffw = get_file_db(false, &file);
        let reader = BufReader::new(&ffw);
        for (i, line) in reader.lines().enumerate() {
            if *matches.get_one::<String>("INDEX").unwrap() == (i + 1).to_string() {
                println!("{}", line.unwrap());
            }
        }
    }
}

fn get_file_db(rem: bool, file: &str) -> std::fs::File {
    if !rem {
        if Path::new(&file).exists() {
            return OpenOptions::new()
                .read(true)
                .write(true)
                .append(true)
                .open(file)
                .unwrap();
        } else {
            return OpenOptions::new()
                .create_new(true)
                .read(true)
                .write(true)
                .append(true)
                .open(file)
                .unwrap();
        }
    } else {
        return OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .open(file)
            .unwrap();
    };
}
