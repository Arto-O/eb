use std::{
    fs::File,
    io::{
        BufReader,
        BufRead,
    },
};

use crate::Args;

pub fn print_file(path_index: usize, args: &Args) {
    let file = File::open(&args.paths[path_index]).unwrap();

    // get all the lines and count them
    let mut line_count = 0;
    let mut lines = Vec::new();
    for line in BufReader::new(file).lines() {
        lines.push(line.unwrap());
        line_count += 1;
    }

    // count the amount to pad line numbers by
    let line_num_length = line_count.to_string().len();

    println!();
    for i in 0..lines.len() {
        // print padding for line number
        for _ in (i + 1).to_string().len()..line_num_length {
            print!(" ");
        }

        println!("{}\t{}", i + 1, lines[i]);
    }
    println!();
}
