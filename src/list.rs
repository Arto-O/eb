use std::{
    fs::{ DirEntry, read_dir },
    os::{ unix::fs::{ PermissionsExt, MetadataExt } },
    collections::HashMap,
    time::{ SystemTime, Duration },
};
use term_grid::{ Alignment, Grid, GridOptions, Direction, Filling, Cell };
use term_size;
use bitvec::prelude::*;
use chrono::{ offset::Local, DateTime};

use crate::{ Args, sort };

// Chars for permissions in long form listing
const PERM_CHARS: [char; 3] = ['r', 'w', 'x'];
const NO_PERM: char = '-';
const PREFIXES: [char; 6] = ['k', 'M', 'G', 'T', 'P', 'E'];
const BINARY_PREFIX: char = 'i';
const DIRECTORY_SIZE: &str = "-";
const TIME_FORMAT: &str = "%d %b %H:%M";

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
        print!("{}", display);
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
    let mut widths = HashMap::new();

    // also cache file sizes and times
    let mut sizes = HashMap::with_capacity(files.len());
    let mut modifieds = HashMap::with_capacity(files.len());
    let mut changeds = HashMap::with_capacity(files.len());
    let mut createds = HashMap::with_capacity(files.len());
    let mut accesseds = HashMap::with_capacity(files.len());
    for file in &files {
        let md = match file.metadata() {
            Ok(x) => x,
            Err(e) => panic!("Failed to retrieve metadata: {}", e),
        };

        if args.inode {
            update_width(&mut widths, "inode", md.ino().to_string().len());
        }

        if args.links {
            update_width(&mut widths, "links", md.nlink().to_string().len());
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

            update_width(&mut widths, "size", size_str.len());

            sizes.insert(file.file_name().into_string().unwrap(), size_str);
        }

        if args.blocks {
            update_width(&mut widths, "blocks", md.blocks().to_string().len());
        }

        if !args.no_user {
            update_width(&mut widths, "user", md.uid().to_string().len());
        }

        if args.group {
            update_width(&mut widths, "group", md.gid().to_string().len());
        }

        if !args.no_time {
            if args.modified {
                let dt: DateTime<Local> = match md.modified() {
                    Ok(st) => st.into(),
                    Err(e) => panic!("Failed to retrieve file timestamp: {}", e),
                };
                let time_str = dt.format(TIME_FORMAT).to_string();

                update_width(&mut widths, "modified", time_str.len());

                modifieds.insert(file.file_name().into_string().unwrap(), time_str);
            }
            
            if args.changed {
                let st = SystemTime::UNIX_EPOCH + Duration::from_secs(md.ctime() as u64);
                let dt: DateTime<Local> = st.into();
                let time_str = dt.format(TIME_FORMAT).to_string();

                update_width(&mut widths, "changed", time_str.len());

                changeds.insert(file.file_name().into_string().unwrap(), time_str);
            }

            if args.created {
                let dt: DateTime<Local> = match md.created() {
                    Ok(st) => st.into(),
                    Err(e) => panic!("Failed to retrieve file timestamp: {}", e),
                };
                let time_str = dt.format(TIME_FORMAT).to_string();

                update_width(&mut widths, "created", time_str.len());

                createds.insert(file.file_name().into_string().unwrap(), time_str);
            }

            if args.accessed {
                let dt: DateTime<Local> = match md.accessed() {
                    Ok(st) => st.into(),
                    Err(e) => panic!("Failed to retrieve file timestamp: {}", e),
                };
                let time_str = dt.format(TIME_FORMAT).to_string();

                update_width(&mut widths, "accessed", time_str.len());

                accesseds.insert(file.file_name().into_string().unwrap(), time_str);
            }
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
            push_pad_str(&mut item_str, &md.ino().to_string(),
                *widths.get("inode").unwrap());
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
            push_pad_str(&mut item_str, &md.nlink().to_string(),
                *widths.get("links").unwrap());
        }

        if !args.no_filesize {
            push_pad_str(&mut item_str, &sizes.get(&name).unwrap(),
                *widths.get("size").unwrap());
        }

        if args.blocks {
            let blocks_str = match md.blocks() {
                0 => String::from("-"),
                x => x.to_string(),
            };

            push_pad_str(&mut item_str, &blocks_str, *widths.get("blocks").unwrap());
        }

        if !args.no_user {
            push_pad_str(&mut item_str, &md.uid().to_string(),
                *widths.get("user").unwrap());
        }

        if args.group {
            push_pad_str(&mut item_str, &md.gid().to_string(),
                *widths.get("group").unwrap());
        }

        if !args.no_time {
            if args.modified {
                push_pad_str(&mut item_str, &modifieds.get(&name).unwrap(),
                    *widths.get("modified").unwrap());
            }

            if args.changed {
                push_pad_str(&mut item_str, &changeds.get(&name).unwrap(),
                    *widths.get("changed").unwrap());
            }

            if args.created {
                push_pad_str(&mut item_str, &createds.get(&name).unwrap(),
                    *widths.get("created").unwrap());
            }

            if args.accessed {
                push_pad_str(&mut item_str, &accesseds.get(&name).unwrap(),
                    *widths.get("accessed").unwrap());
            }
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

fn update_width(map: &mut HashMap<String, usize>, key: &str, value: usize) {
    if !map.contains_key(key) || value > *map.get(key).unwrap() {
        map.insert(key.to_string(), value);
    }
}
