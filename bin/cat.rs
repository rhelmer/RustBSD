extern crate getopts;
use getopts::{optflag,getopts,OptGroup};
use std::{io,os};

fn print_usage(program: &str, _opts: &[OptGroup]) {
    println!("Usage: {} [options]", program);
    println!("-h\tUsage");
}

fn main() {
    let args: Vec<String> = os::args();

    let program = args[0].clone();

    let opts = [
        optflag("h", "help", "print this help menu")
    ];

    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(program.as_slice(), opts);
        return;
    }

    for line in io::stdin().lines() {
        print!("{}", line.unwrap());
    }
}
