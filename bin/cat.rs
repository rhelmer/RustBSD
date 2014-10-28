extern crate getopts;
use getopts::{optflag,getopts,OptGroup};
use std::{io,os};

fn print_usage(program: &str, _opts: &[OptGroup]) {
    println!("Usage: {} [options]", program);
    println!("-h\tUsage");
    println!("-n\tNumber the output lines, starting at 1");
}

fn main() {
    let args: Vec<String> = os::args();

    let program = args[0].clone();

    // FIXME help here is redundant with print_usage
    // FIXME would be nice to if disabling --long-opts was cleaner
    let opts = [
        optflag("h", "", "print this help menu"),
        optflag("n", "", "Number the output lines, starting at 1")
    ];

    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f.to_string()) }
    };

    let mut count_lines = false;

    if matches.opt_present("h") {
        print_usage(program.as_slice(), opts);
        return;
    } else if matches.opt_present("n") {
        count_lines = true;
    }

    let mut line_number: int = 0;

    for line in io::stdin().lines() {
        if count_lines {
            line_number += 1;
            print!("    {}  {}", line_number, line.unwrap());
        } else {
            print!("{}", line.unwrap());
        }
    }
}
