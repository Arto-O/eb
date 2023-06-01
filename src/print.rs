use std::{
    fs::File,
    io::{
        BufReader,
        BufRead,
    },
};
use term_size;

use crate::Args;

const TAB_LENGTH: usize = 8;

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

    for i in 0..lines.len() {
        print!("{:line_num_length$}", i + 1);

        // max line length = terminal width - tab length
        let max_line_len = match term_size::dimensions() {
            Some((w, _)) => w - TAB_LENGTH,
            None => panic!("Couldn't determine terminal width."),
        };

        let mut line = lines[i].clone();

        // cut and print the line until it's shorter than max line length
        while line.len() > max_line_len {
            println!("\t{}", &line[0..max_line_len]);
            line = line[max_line_len..].to_string();
        }
        println!("\t{}", line);
    }
    println!();
}
