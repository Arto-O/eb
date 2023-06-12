use std::fs::metadata;

use crate::{ Args, list, print };

pub fn handle_path(path_index: isize, args: &Args) {
    // list current directory if no path was provided
    let path = if path_index < 0 {
        "."
    } else {
        &args.paths[path_index as usize]
    };

    // get file metadata
    let md = match metadata(path) {
        Ok(x) => x,
        Err(e) => panic!("Failed to retrieve metadata: {}", e),
    };

    if md.is_dir() {
        list::list_dir_contents(path, args);
    } else {
        print::print_file(path, args);
    }
}

pub fn handle_paths(args: &Args) {
    for i in 0..args.paths.len() {
        if i > 0 {
            println!();
        }

        println!("{}:", args.paths[i]);
        handle_path(i as isize, args);
    }
}
