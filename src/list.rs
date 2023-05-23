use std::fs::{
    read_dir,
    ReadDir,
};
use term_grid::{
    Grid,
    GridOptions,
    Direction,
    Filling,
    Cell,
};

use crate::Args;

pub fn list_dir_contents(path_index: isize, args: &Args) {
    let path;
    if path_index < 0 {
        path = ".";
    } else {
        path = &args.paths[path_index as usize];
    }
    let rd = read_dir(path).unwrap();

    let mut items = Vec::new();
    for de in rd {
        let item = de.unwrap().file_name().into_string().unwrap();

        if !args.all && &item[0..1] == "." {
            continue;
        }

        items.push(item);
    }

    if args.oneline {
        list_one_per_line(items);
    } else {
        list_in_grid(items);
    }
} 

fn list_in_grid(items: Vec<String>) {
    println!("list_in_grid");
}

fn list_one_per_line(items: Vec<String>) {
    for item in items {
        println!("{}", item);
    }
}
