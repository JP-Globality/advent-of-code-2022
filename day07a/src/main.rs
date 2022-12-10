use std::collections::{HashMap, HashSet};
use std::str::Chars;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn generate_directories_from_current_dir(current_dir: &String) -> Vec<String> {
    // Generates list of directories ( as well as current directory )
    // Example path = "/my/directory/path"
    // Generates: "/my", "/my/directory" "/my/directory/path"

    let dirs = current_dir.split("/").collect::<Vec<&str>>();
    let mut last_path = "".to_string();
    let mut paths: Vec<String> = Vec::new();
    for (index, dir) in dirs.into_iter().enumerate() {
        if index == 0 {
            // ignore this case (we could use a range to get round this)
            continue;
        }
        last_path.push_str("/");
        last_path.push_str(dir);
        paths.push(last_path.to_owned());
    }
    return paths;
}

fn main() {
    // Stores the current directory sizes
    let mut directory_sizes: HashMap<String, i32> = HashMap::new();
    // Example directories to store as keys:
    // "a"
    // "a/e"
    // "a/b/c"

    let mut current_dir = "".to_string();
    for line in lines_from_file("./input.txt").iter_mut() {
        // First need to work out if we are changing directories
        // or if it's a list command

        // is it a command or is a command response
        if line.contains("$ cd") {
            // we know we're changing directories now

            // let mut mutable_line = line.to_owned();
            let cmd_arg = line.replace("$ cd ", "");
            match cmd_arg.as_str() {
                // back one directory
                ".." => {
                    // Example path = "/a/b/c"
                    // We need to go back to "/a/b"
                    let val = current_dir.split("/").last().unwrap();
                    current_dir = current_dir.replace(val, "");
                    // Then remove the slash from the end
                    current_dir.remove(current_dir.len() - 1);
                }
                // back to the start
                "/" => {
                    current_dir = "".to_string();
                }
                // Forward one directory
                dir_name => {
                    current_dir.push_str("/");
                    current_dir.push_str(dir_name);
                }
            }
        } else if line.contains("$ ls") {
            // We ignore this case
            continue;
        } else {
            // We must be listing out files
            if line.contains("dir") {
                // we have a new directory
                // we can ignore this case
                continue;
            } else {
                // We have a size and a file name
                let split = line.split(" ").collect::<Vec<&str>>();
                let file_size = split.get(0).unwrap().parse::<i32>().unwrap();

                // Need to generate a list of directories to add the file size to...
                let paths = generate_directories_from_current_dir(&current_dir);
                println!("{:?}", paths);
                for dir_path in paths.into_iter() {
                    // Get directory
                    let mut directory_size = directory_sizes.get(&dir_path);
                    if directory_size.is_some() {
                        // then we can add to the directory
                        let new_directory_size = directory_size.unwrap() + file_size;
                        directory_sizes.insert(dir_path, new_directory_size);
                    } else {
                        // we need to insert a new value
                        directory_sizes.insert(dir_path, file_size);
                    }
                }
            }
        }
    }
    let mut score = 0;
    for (_, dir_size) in directory_sizes.iter() {
        if dir_size.to_owned() < 100000 {
            score += dir_size;
        }
    }

    println!("Score: {}", score)
}
