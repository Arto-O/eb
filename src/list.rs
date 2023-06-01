use std::{
    fs::{
        DirEntry,
        read_dir,
    },
    os::{
        unix::fs::{
            PermissionsExt,
            MetadataExt,
        },
    }, collections::HashMap,
};
use term_grid::{
    Alignment,
    Grid,
    GridOptions,
    Direction,
    Filling,
    Cell,
};
use term_size;
use bitvec::prelude::*;

use crate::{
    Args,
    sort,
};

// Chars for permissions in long form listing
const PERM_CHARS: [char; 3] = ['r', 'w', 'x'];
const NO_PERM: char = '-';
const PREFIXES: [char; 6] = ['k', 'M', 'G', 'T', 'P', 'E'];
const BINARY_PREFIX: char = 'i';
const DIRECTORY_SIZE: &str = "-";

pub fn list_dir_contents(path_index: isize, args: &Args) {
    // list current directory if no path was provided
    let path = if path_index < 0 {
        "."
    } else {
        &args.paths[path_index as usize]
    };

    // get files
    let mut files = Vec::new();
    for r in read_dir(path).unwrap() {
        files.push(r.unwrap());
    }

    sort::sort_files(&mut files, args);

    if args.long {
        let items = get_long_form_items(files, args);

        if args.grid {
            list_in_grid(items, 4);
        } else {
            list_one_per_line(items);
        }
    } else {
        let items = get_short_form_items(files, args);

        if args.oneline {
            list_one_per_line(items);
        } else {
            list_in_grid(items, 2);
        }
    }
} 

fn list_in_grid(items: Vec<String>, margin: usize) {
    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(margin),
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

fn get_short_form_items(files: Vec<DirEntry>, args: &Args) -> Vec<String> {
    let mut items = Vec::new();

    for file in files {
        let item = file.file_name().into_string().unwrap();

        // check if hidden file
        if &item[0..1] == "." && !args.all {
            continue;
        }

        items.push(item);
    }

    items
}

fn get_long_form_items(files: Vec<DirEntry>, args: &Args) -> Vec<String> {
    let mut items = Vec::new();

    // get widths for some columns
    let mut inode_max_width = 0;
    let mut links_max_width = 0;
    let mut size_max_width = 0;

    // also cache file sizes
    let mut sizes = HashMap::with_capacity(files.len());
    for file in &files {
        let md = match file.metadata() {
            Ok(x) => x,
            Err(e) => panic!("Failed to retrieve metadata: {}", e),
        };

        if args.inode {
            let width = md.ino().to_string().len();
            if width > inode_max_width {
                inode_max_width = width;
            }
        }

        if args.links {
            let width = md.nlink().to_string().len();
            if width > links_max_width {
                links_max_width = width;
            }
        }

        // file sizes get cached because they need more processing here anyway
        if !args.no_filesize {
            let mut size_str;
            
            if md.is_file() {
                let size = md.size();

                if args.bytes {
                    size_str = format_with_sep(size);
                } else {
                    let mut size = size as f64;

                    let magnitude;
                    if args.binary {
                        magnitude = 1024.0;
                    } else {
                        magnitude = 1000.0;
                    }

                    // set order of magnitude
                    let mut order = 0;
                    while size > magnitude {
                        size /= magnitude;
                        order += 1;
                    }
                    

                    if order > 0 {
                        size_str = format_size(size);
                        
                        size_str.push(PREFIXES[order - 1]);
                        if args.binary {
                            size_str.push(BINARY_PREFIX);
                        }
                    } else {
                        size_str = format_with_sep(size as u64);
                    }
                }
            } else {
                size_str = DIRECTORY_SIZE.to_string();
            }

            let width = size_str.len();
            if width > size_max_width {
                size_max_width = width;
            }

            sizes.insert(file.file_name().into_string().unwrap(), size_str);
        }
    }

    for file in files {
        let name = file.file_name().into_string().unwrap();

        // check if hidden file
        if &name[0..1] == "." && !args.all {
            continue;
        }

        let md = match file.metadata() {
            Ok(x) => x,
            Err(e) => panic!("Failed to retrieve metadata: {}", e),
        };
        let mut item_str = String::new();

        if args.inode {
            push_pad_str(&mut item_str, &md.ino().to_string(), inode_max_width);
        }

        if !args.no_permissions {
            item_str.push(if md.is_dir() {
                'd'
            } else {
                '.'
            });

            // get file permissions as a bit slice
            let bit_arr = md.permissions().mode().into_bitarray::<Msb0>();
            let (_, perms) = bit_arr.split_at(32 - 9);

            // iterate over bit slice and permission letters
            for i in 0..perms.len() {
                item_str.push(if perms[i] {
                    PERM_CHARS[i % 3]
                } else {
                    NO_PERM
                });
            }

            item_str.push(' ');
        }

        if args.links {
            push_pad_str(&mut item_str, &md.nlink().to_string(), links_max_width);
        }

        if !args.no_filesize {
            push_pad_str(&mut item_str, &sizes.get(&name).unwrap(), size_max_width);
        }

        items.push(item_str + &name);
    }

    items
}

fn push_pad_str(a: &mut String, b: &str, pad: usize) {
    a.push_str(&format!("{:>pad$} ", b));
}

// formats integer with thousands separator
fn format_with_sep(num: u64) -> String {    
    let mut num = num;

    let mut result = String::new();
    loop {
        let modulus = num % 1000;

        if num > 1000 {
            result.insert_str(0, &format!(",{:0>3}", modulus));
        } else {
            result.insert_str(0, &modulus.to_string());
        }

        num /= 1000;
        if num == 0 {
            break;
        }
    }

    result
}

// formats size to 2 sig-figs or nearest integer with thousands separators
fn format_size(num: f64) -> String {
    if num >= 10.0 {
        format_with_sep(num.round() as u64)
    } else {
        let mut result = format_with_sep(num.floor() as u64);
        result.push_str(&format!(".{:.0}", (num.fract() * 10.0).round()));
        result
    }
}
