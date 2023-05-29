use std::fs::DirEntry;

use crate::Args;

pub fn sort_files(vec: &mut Vec<DirEntry>, _args: &Args) {
    vec.sort_by(|a, b|a.file_name().to_ascii_lowercase()
        .cmp(&b.file_name().to_ascii_lowercase()));
}