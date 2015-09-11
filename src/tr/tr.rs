extern crate getopts;

use std::env;
use getopts::{Options,Matches};

fn main() {
    match parse_options() {
        None => { return; },
        Some(matches) => tr(&matches)
    }
}

fn parse_options() -> Option<Matches> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();

    opts.optflag("c", "complement", "use the complement of SET1");
    opts.optflag("C", "", "same as -c");
    opts.optflag("d", "delete", "delete characters in SET1, do not translate");
    opts.optflag("s", "squeeze", "replace each input sequence of a repeated character that is listed in SET1 with  a single occurrence of that character");
    opts.optflag("t", "truncate-set1", "first truncate SET1 to length of SET2");


    let matches = match opts.parse(&args[1..]) {
        Ok(x) => x,
        Err(e) => {
            println!("{}", e);
            println!("");
            print_usage(&opts);
            return None;
        }
    };

    let delete_flag: bool = matches.opt_present("d");
    let squeeze_flag: bool = matches.opt_present("s");

    if matches.free.is_empty() {
        println!("SET1 is required");
        println!("");
        print_usage(&opts);
        return None;
    }

    if matches.opt_present("t") {
        if delete_flag || matches.free.len() != 2 {
            println!("-t may only be used when translating.");
            println!("");
            print_usage(&opts);
            return None;
        }
    }

    if !squeeze_flag && !delete_flag && matches.free.len() != 2 {
        println!("SET1 and SET2 are required for translation.");
        println!("");
        print_usage(&opts);
        return None;
    }

    if squeeze_flag {
        if delete_flag && matches.free.len() == 1 {
            println!("Two strings must be given when both deleting and squeezing repeats.");
            println!("");
            print_usage(&opts);
            return None;
        }
    }

    Some(matches)
}

fn print_usage(opts: &Options) {
    let brief = "Usage: tr - [OPTION]... SET1 [SET2]";
    print!("{}", opts.usage(&brief));
}

fn tr(matches: &Matches) {
    let delete_flag: bool = matches.opt_present("d");
    let complement_flag: bool = matches.opts_present(&["c".to_string(), "C".to_string()]);
    let truncate_flag: bool = matches.opt_present("t");
    let translating: bool = !matches.opt_present("d") && matches.free.len() == 2;
    let deleting: bool = matches.opt_present("d");
    let squeezing: bool = matches.opt_present("s");

    let mut set1 = matches.free[0].clone();
    let mut set2 = matches.free[1].clone();

    if translating && truncate_flag && set1.len() > set2.len() {
        set1 = set1[0..set2.len() -1].to_string();
    }

    print!("tr yo");
}
