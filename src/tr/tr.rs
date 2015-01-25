#![allow(unstable)]
extern crate getopts;

use std::os;
use std::io::stdio::{stdin, stdout_raw};

fn main() {
    let opts = &[
        getopts::optflag("c", "complement", "use the complement of SET1"),
        getopts::optflag("C", "", "same as -c"),
        getopts::optflag("d", "delete", "delete characters in SET1, do not translate"),
        getopts::optflag("s", "squeeze", "replace each input sequence of a repeated character that is listed in SET1 with  a single occurrence of that character"),
        getopts::optflag("t", "truncate-set1", "first truncate SET1 to length of SET2"),
    ];

    let usage = getopts::usage("tr - [OPTION]... SET1 [SET2]", opts);

    let matches = match getopts::getopts(os::args().tail(), opts) {
        Ok(x) => x,
        Err(e) => {
            println!("{}", e);
            println!("");
            print!("{}", usage);
            return;
        }
    };

    let delete_flag: bool = matches.opt_present("d");
    //let squeeze_flag: bool = matches.opt_present("s");
    //let truncate_flag: bool = matches.opt_present("t");
    let complement_flag: bool = matches.opts_present(&["c".to_string(), "C".to_string()]);

    let set1 = match matches.free.first() {
        Some(s) => s.clone(),
        None => {
            println!("SET1 is required");
            print!("{}", usage);
            return;
        }
    };
    //let set2 = match matches.free.get(1) {
        //Some(s) => Some(s.clone()),
        //None => None
    //};

    if delete_flag {
        delete(set1, complement_flag);
    }
}

fn write_char(c: char) {
    match stdout_raw().write_char(c) {
        Err(e) => panic!("{}", e),
        Ok(_) => return
    };
}

fn delete(set1: String, complement: bool) {
    let mut input = stdin();

    let in_set1 = |&: c: char| {
        set1.chars().any(|s| { s == c })
    };

    loop {
        match input.read_char() {
            Err(_) => break,
            Ok(c) => {
                if complement {
                    if in_set1(c) {
                        write_char(c);
                    }
                } else {
                    if !in_set1(c) {
                        write_char(c);
                    }
                }
            }
        };
    };
}
