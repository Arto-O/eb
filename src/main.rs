use std::ops::Range;
use clap::{
    Parser,
    builder::{
        PossibleValuesParser,
        TypedValueParser,
        NonEmptyStringValueParser,
    },
};

fn main() {
    let _args = Args::parse();
}

// help message headings
const DIR_LIST_FORMAT_HEADING: &str = "Directory List Formatting Options";
const DIR_LIST_FILT_SORT_HEADING: &str = "Directory List Filtering and Sorting Options";
const DIR_LIST_LONG_VIEW_HEADING: &str = "Directory List Long View Options";
const FILE_PRINT_HEADING: &str = "File Printing Options";

/// eb = exa + bat
/// 
/// Intuitively list directory contents or concatenate files.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Args {
    /// Directory(s) and/or File(s) to list / print / concatenate. No arguments lists out current directory
    #[arg(trailing_var_arg = true, num_args = 1.., value_name = "FILES", allow_hyphen_values = true)]
    paths: Vec<String>,

    // File print options
    
    /// Show non-printable characters
    #[arg(short = 'A', long, help_heading = FILE_PRINT_HEADING)]
    show_all: bool,

    /// Specify name to display for the file
    #[arg(short = 'F', long, default_value = "", value_name = "NAME", hide_default_value = true,
        help_heading = FILE_PRINT_HEADING)]
    file_name: String,

    /// Show line numbers
    #[arg(short = 'N', long, help_heading = FILE_PRINT_HEADING)]
    numbers: bool,

    /// Specify when to use the pager
    #[arg(short = 'P', long, value_parser = PossibleValuesParser::new(["auto", "never", "always"]),
        default_value = "auto", value_name = "WHEN", hide_default_value = true,
        help_heading = FILE_PRINT_HEADING)]
    paging: String,

    /// Only print the lines from N to M
    #[arg(short = 'r', long, value_parser = RangeValueParser, default_value = "-1:-1",
        value_name = "N:M", hide_default_value = true, help_heading = FILE_PRINT_HEADING)]
    line_range: Range<isize>,

    /// Specify text wrapping mode
    #[arg(short, short_alias = 'W', long,
        value_parser = PossibleValuesParser::new(["auto", "never", "character"]),
        default_value = "auto", value_name = "MODE", hide_default_value = true,
        help_heading = FILE_PRINT_HEADING)]
    wrap: String,


    // Dir list formatting options
    
    /// Display one item per line
    #[arg(short = '1', long, help_heading = DIR_LIST_FORMAT_HEADING)]
    oneline: bool,

    /// Display items in a grid
    #[arg(short = 'G', long, overrides_with = "oneline", help_heading = DIR_LIST_FORMAT_HEADING)]
    grid: bool,

    /// Display extended file metadata as a table
    #[arg(short, long, help_heading = DIR_LIST_FORMAT_HEADING)]
    long: bool,

    /// Recurse into directories
    #[arg(short = 'R', long, help_heading = DIR_LIST_FORMAT_HEADING)]
    recurse: bool,

    /// Recurse into directories as a tree
    #[arg(short = 'T', long, help_heading = DIR_LIST_FORMAT_HEADING)]
    tree: bool,

    /// Sort the grid across
    #[arg(short = 'x', short_alias = 'X', long, help_heading = DIR_LIST_FORMAT_HEADING)]
    across: bool,


    // Dir list filtering options

    /// Show hidden files
    #[arg(short, long, help_heading = DIR_LIST_FILT_SORT_HEADING)]
    all: bool,

    /// List directories as files; don't list their contents
    #[arg(short = 'd', long, help_heading = DIR_LIST_FILT_SORT_HEADING)]
    list_dirs: bool,

    /// List directories only; don't list files
    #[arg(short = 'D', long, help_heading = DIR_LIST_FILT_SORT_HEADING)]
    only_dirs: bool,

    /// Set the level of recursion
    #[arg(short = 'L', long, default_value = "-1", value_name = "DEPTH", hide_default_value = true,
        help_heading = DIR_LIST_FILT_SORT_HEADING)]
    level: isize,

    /// List all directories before files
    #[arg(short = 'q', short_alias = 'Q', long, help_heading = DIR_LIST_FILT_SORT_HEADING)]
    group_directories_first: bool,


    // Dir long listing field options

    /// List file sizes with binary prefixes
    #[arg(short, long, help_heading = DIR_LIST_LONG_VIEW_HEADING)]
    binary: bool,

    /// List file sizes in bytes
    #[arg(short = 'B', long, overrides_with = "binary", help_heading = DIR_LIST_LONG_VIEW_HEADING)]
    bytes: bool,

    /// Use the changed timestamp field
    #[arg(short, short_alias = 'C', long, help_heading = DIR_LIST_LONG_VIEW_HEADING)]
    changed: bool,

    /// List each file's group
    #[arg(short, long, help_heading = DIR_LIST_LONG_VIEW_HEADING)]
    group: bool,

    /// Show a header for each column
    #[arg(short = 'H', long, help_heading = DIR_LIST_LONG_VIEW_HEADING)]
    header: bool,

    /// List each file's inode number
    #[arg(short, short_alias = 'I', long, help_heading = DIR_LIST_LONG_VIEW_HEADING)]
    inode: bool,

    /// List each file's number of hard links
    #[arg(short = 'k', short_alias = 'K', long, help_heading = DIR_LIST_LONG_VIEW_HEADING)]
    links: bool,

    /// Use the modified timestamp field
    #[arg(short, short_alias = 'M', long, help_heading = DIR_LIST_LONG_VIEW_HEADING)]
    modified: bool,

    /// List numeric user and group IDs
    #[arg(short, long, help_heading = DIR_LIST_LONG_VIEW_HEADING)]
    numeric: bool,

    /// List each file's number of file system blocks
    #[arg(short = 'S', long, help_heading = DIR_LIST_LONG_VIEW_HEADING)]
    blocks: bool,

    /// Use the accessed timestamp field
    #[arg(short = 'u', long, help_heading = DIR_LIST_LONG_VIEW_HEADING)]
    accessed: bool,

    /// Use the created timestamp field
    #[arg(short = 'U', long, help_heading = DIR_LIST_LONG_VIEW_HEADING)]
    created: bool,

    /// Hide the permissions field
    #[arg(short = 'o', short_alias = 'O', long, help_heading = DIR_LIST_LONG_VIEW_HEADING)]
    no_permissions: bool,

    /// Hide the filesize field
    #[arg(short = 'z', short_alias = 'Z', long, help_heading = DIR_LIST_LONG_VIEW_HEADING)]
    no_filesize: bool,

    /// Hide the user field
    #[arg(short = 'y', short_alias = 'Y', long, help_heading = DIR_LIST_LONG_VIEW_HEADING)]
    no_user: bool,

    /// Hide the time field
    #[arg(short = 't', long, help_heading = DIR_LIST_LONG_VIEW_HEADING)]
    no_time: bool,
}

#[derive(Clone)]
struct RangeValueParser;

impl TypedValueParser for RangeValueParser {
    type Value = Range<isize>;

    fn parse_ref(&self, cmd: &clap::Command, arg: Option<&clap::Arg>, value: &std::ffi::OsStr)
        -> Result<Self::Value, clap::Error> {
            let val_str = NonEmptyStringValueParser::new().parse_ref(cmd, arg, value)?;
            let vals: Vec<&str> = val_str.split(':').collect();

            if vals.len() != 2 {
                return Err(clap::Error::new(clap::error::ErrorKind::InvalidValue));
            }

            let start: isize = match vals[0].parse() {
                Ok(x) => x,
                Err(_) => return Err(clap::Error::new(clap::error::ErrorKind::InvalidValue)),
            };
            let end: isize = match vals[1].parse() {
                Ok(x) => x,
                Err(_) => return Err(clap::Error::new(clap::error::ErrorKind::InvalidValue)),
            };

            Ok(start..end)
        }
}
