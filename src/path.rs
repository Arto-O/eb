use std::fs::metadata;

use crate::{
    Args,
    list,
    print,
};

pub fn handle_path(path_index: usize, args: &Args) {
    // get file metadata
    let md = match metadata(&args.paths[path_index]) {
        Ok(x) => x,
        Err(e) => {
            println!("{}", e);
            return;
        },
    };

    if md.is_dir() {
        list::list_dir_contents(path_index as isize, args);
    } else {
        print::print_file(path_index, args);
    }
}

pub fn handle_paths(args: &Args) {
    for i in 0..args.paths.len() {
        if i > 0 {
            println!();
        }

        println!("{}:", args.paths[i]);
        handle_path(i, args);
    }
}
