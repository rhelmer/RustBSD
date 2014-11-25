#![feature(slicing_syntax)]
extern crate getopts;
use getopts::{optflag,getopts,OptGroup};
use std::{io,os};
use std::io::process::Command;

fn print_usage(program: &str, _opts: &[OptGroup]) {
    println!("Usage: {} [options]", program);
    println!("-h\tUsage");
}

fn shell(prompt: &str) {
    loop {
        print!("{} ", prompt);
        // TODO just read one line instead of for loop w/ immediate break
        for line in io::stdin().lines() {
            let cmd = line.unwrap();
            // TODO only ,in -v mode
            // println!("debug: {}", cmd.as_slice());

            // shell built-ins
            match cmd.as_slice().trim() {
                "exit" => return,
                _ => exec(cmd)
            }
            break;
        }
    }
}

fn exec(cmd: String) {
    // TODO split by space, call as cmd+args
    let args: Vec<&str> = cmd.as_slice().trim().split(' ').collect();
    let output = match Command::new(args[0]).args(args[1..].as_slice()).output() {
        Ok(p) => p,
        Err(e) => panic!("failed to execute process: {}", e),
    };
    print!("{}", String::from_utf8_lossy(output.output.as_slice()));
    print!("{}", String::from_utf8_lossy(output.error.as_slice()));
    println!("{}", output.status);
}

fn main() {
    let args: Vec<String> = os::args();

    let program = args[0].clone();

    // FIXME help here is redundant with print_usage
    // TODO would be nice if disabling --long-opts was cleaner
    let opts = [
        optflag("h", "", "print this help menu"),
    ];

    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(program.as_slice(), opts);
        return;
    }

    let prompt = "$";
    shell(prompt);
}
