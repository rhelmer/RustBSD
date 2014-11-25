extern crate getopts;
use getopts::{optflag,getopts,OptGroup};
use std::{io,os};
use std::io::{BufferedReader,File};

fn print_usage(program: &str, _opts: &[OptGroup]) {
    println!("Usage: {} [options]", program);
    println!("-h\tUsage");
    println!("-n\tNumber the output lines, starting at 1");
}

fn print_stdout<T:Buffer>(f: &mut T, count_lines: bool) {
    let mut line_number: int = 0;

    for line in f.lines() {
        if count_lines {
            line_number += 1;
            print!("    {}  {}", line_number, line.unwrap());
        } else {
            print!("{}", line.unwrap());
        }
    }
}

fn cat(files: Vec<String>, count_lines: bool) {
    // no filenames passed means to read from stdin only
    if files.is_empty() {
        print_stdout(&mut io::stdin(), count_lines);
    } else {
        for file in files.iter() {
            // the filename "-" means to read stdin
            if file.as_slice() == "-" {
                print_stdout(&mut io::stdin(), count_lines);
            } else {
                let path = Path::new(file.as_slice());
                let mut file = BufferedReader::new(File::open(&path));

                print_stdout(&mut file, count_lines);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = os::args();

    let program = args[0].clone();

    // FIXME help here is redundant with print_usage
    // TODO would be nice if disabling --long-opts was cleaner
    let opts = &[
        optflag("h", "", "print this help menu"),
        optflag("n", "", "Number the output lines, starting at 1")
    ];

    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let files = matches.free.clone();
    let mut count_lines = false;

    if matches.opt_present("h") {
        print_usage(program.as_slice(), opts);
        return;
    } else if matches.opt_present("n") {
        count_lines = true;
    }

    cat(files, count_lines);
}
