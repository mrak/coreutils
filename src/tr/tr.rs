extern crate getopts;

use std::env;
use getopts::{Options};
use std::io::{stdin,stdout,Read,Write};

enum Mode {
    Translating([u8; 256]),
    Deleting([bool; 256]),
    Noop,
}

struct Arguments {
    mode: Mode,
    squeezing: [bool; 256],
}

fn main() {
    match parse_arguments() {
        None => { return; },
        Some(arguments) => tr(&arguments)
    }
}

fn parse_arguments() -> Option<Arguments> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();

    opts.optflag("c", "complement", "use the complement of SET1");
    opts.optflag("C", "", "same as -c");
    opts.optflag("d", "delete", "delete characters in SET1, do not translate");
    opts.optflag("s", "squeeze", "replace each input sequence of a repeated character that is listed in SET1 with  a single occurrence of that character");
    opts.optflag("t", "truncate-set1", "first truncate SET1 to length of SET2");
    opts.optflag("h", "help", "display this help and exit");


    let matches = match opts.parse(&args[1..]) {
        Ok(x) => x,
        Err(e) => {
            println!("{}", e);
            println!("");
            print_usage(&opts);
            return None;
        }
    };

    if matches.opt_present("h") {
        print_usage(&opts);
        return None;
    }

    let delete_flag: bool = matches.opt_present("d");
    let squeeze_flag: bool = matches.opt_present("s");

    let mut set1 = if matches.free.is_empty() {
        print_usage_error(&opts, "SET1 is required");
        return None;
    } else {
        matches.free[0].clone()
    };

    let mut set2 = if matches.free.len() > 1 {
        Some(matches.free[1].clone())
    } else {
        None
    };

    if matches.opt_present("t") {
        if delete_flag || matches.free.len() < 2 {
            print_usage_error(&opts, "-t may only be used when translating.");
            return None;
        }

        match set2 {
            None => (),
            Some(ref s) => {
                if s.len() < set1.len() {
                    set1.truncate(s.len());
                }
            },
        };
    }

    set2 = set2.and_then(|mut s| set1.chars().last().and_then(|c| {
        while s.len() < set1.len() {
            s.push(c);
        }

        Some(s)
    }));

    if !squeeze_flag && !delete_flag && matches.free.len() < 2 {
        print_usage_error(&opts, "SET1 and SET2 are required for translation.");
        return None;
    }

    if squeeze_flag {
        if delete_flag && matches.free.len() < 2 {
            print_usage_error(&opts, "Two strings must be given when both deleting and squeezing repeats.");
            return None;
        }
    }

    let mode = if delete_flag {
        let value = matches.opt_present("c") || matches.opt_present ("C");
        let mut deletions = [value; 256];

        for b in set1.as_bytes() {
            deletions[*b as usize] = !deletions[*b as usize];
        }

        Mode::Deleting(deletions)
    } else if set2.is_some() {
        let mut translations = [0; 256];
        let set2bytes = set2.as_ref().unwrap().as_bytes();

        for b in 0..255 {
            translations[b as usize] = b;
        }

        for (i, b) in set1.as_bytes().into_iter().enumerate() {
            translations[*b as usize] = set2bytes[i];
        }

        Mode::Translating(translations)
    } else {
        Mode::Noop
    };

    let squeezing = [false; 256];

    Some(Arguments {
        mode: mode,
        squeezing: squeezing,
    })
}

fn print_usage(opts: &Options) {
    let brief = "Usage: tr - [OPTION]... SET1 [SET2]";
    print!("{}", opts.usage(&brief));
}

fn print_usage_error(opts: &Options, error: &str) {
    println!("{}", error);
    println!("");
    print_usage(opts);
}

fn tr(arguments: &Arguments) {
    match arguments.mode {
        Mode::Deleting(ds) => delete(&ds),
        Mode::Translating(ts) => translate(&ts),
        Mode::Noop => (),
    };
}

fn delete(deletions: &[bool; 256]) {
    let out_unlocked = stdout();
    let mut out = out_unlocked.lock();
    let input_unlocked = stdin();
    let input = input_unlocked.lock();

    for byte in input.bytes() {
        match byte {
            Err(_) => return,
            Ok(b) => {
                if !deletions[b as usize] {
                    out.write(&[b]).unwrap();
                }
            },
        };
    };
}

fn translate(translations: &[u8; 256]) {
    let out_unlocked = stdout();
    let mut out = out_unlocked.lock();
    let input_unlocked = stdin();
    let input = input_unlocked.lock();

    for byte in input.bytes() {
        match byte {
            Err(_) => return,
            Ok(b) => {
                out.write(&[translations[b as usize]]).unwrap();
            },
        };
    };
}
