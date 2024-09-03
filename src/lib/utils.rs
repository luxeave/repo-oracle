use std::env;
use std::process;

pub fn parse_path_and_ext(args: &[String]) -> (String, String) {
    let mut path_flag_index = None;
    let mut ext_flag_index = None;
    for (i, arg) in args.iter().enumerate() {
        if arg == "--path" {
            path_flag_index = Some(i);
        } else if arg == "--ext" {
            ext_flag_index = Some(i);
        }
    }

    let root_dir_path = if let Some(index) = path_flag_index {
        if index + 1 < args.len() {
            args[index + 1].clone()
        } else {
            println!("Missing value for --path flag.");
            process::exit(1);
        }
    } else {
        env::current_dir().unwrap().to_str().unwrap().to_string()
    };

    let file_extension = if let Some(index) = ext_flag_index {
        if index + 1 < args.len() {
            args[index + 1].clone()
        } else {
            println!("Missing value for --ext flag.");
            process::exit(1);
        }
    } else {
        "all".to_string()
    };

    (root_dir_path, file_extension)
}

pub fn parse_path(args: &[String]) -> String {
    let mut path_flag_index = None;
    for (i, arg) in args.iter().enumerate() {
        if arg == "--path" {
            path_flag_index = Some(i);
            break;
        }
    }

    if let Some(index) = path_flag_index {
        if index + 1 < args.len() {
            args[index + 1].clone()
        } else {
            println!("Missing value for --path flag.");
            process::exit(1);
        }
    } else {
        env::current_dir().unwrap().to_str().unwrap().to_string()
    }
}
