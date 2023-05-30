use std::fs::read_dir;
use term_grid::{
    Alignment,
    Grid,
    GridOptions,
    Direction,
    Filling,
    Cell,
};
use term_size;

use crate::{
    Args,
    sort,
};

pub fn list_dir_contents(path_index: isize, args: &Args) {
    // list current directory if no path was provided
    let path;
    if path_index < 0 {
        path = ".";
    } else {
        path = &args.paths[path_index as usize];
    }

    // get files
    let mut files = Vec::new();
    for r in read_dir(path).unwrap() {
        files.push(r.unwrap());
    }

    sort::sort_files(&mut files, args);

    // get file names
    let mut items = Vec::new();
    for file in files {
        let item = file.file_name().into_string().unwrap();

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
    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(2),
        direction: Direction::TopToBottom,
    });

    for item in &items {
        grid.add(Cell {
            width: item.len(),
            contents: item.to_string(),
            alignment: Alignment::Left,
        });
    }

    let term_width = match term_size::dimensions() {
        Some((w, _)) => w,
        None => panic!("Couldn't determine terminal width."),
    };

    if let Some(display) = grid.fit_into_width(term_width) {
        println!("{}", display);
    } else {
        list_one_per_line(items);
    }
}

fn list_one_per_line(items: Vec<String>) {
    for item in items {
        println!("{}", item);
    }
}
