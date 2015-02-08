#![allow(unstable)]
extern crate getopts;
extern crate taskpipe;

use std::os::args;
use std::io::stdio::{stdin, stdout_raw};
use std::sync::mpsc::{Sender,Receiver};
use getopts::Matches;
use std::sync::Arc;

fn main() {
    let opts = &[
        getopts::optflag("c", "complement", "use the complement of SET1"),
        getopts::optflag("C", "", "same as -c"),
        getopts::optflag("d", "delete", "delete characters in SET1, do not translate"),
        getopts::optflag("s", "squeeze", "replace each input sequence of a repeated character that is listed in SET1 with  a single occurrence of that character"),
        getopts::optflag("t", "truncate-set1", "first truncate SET1 to length of SET2"),
    ];

    let usage = getopts::usage("tr - [OPTION]... SET1 [SET2]", opts);

    let matches = match getopts::getopts(args().tail(), opts) {
        Ok(x) => x,
        Err(e) => {
            println!("{}", e);
            println!("");
            print!("{}", usage);
            return;
        }
    };

    if matches.free.len() == 0 {
        println!("SET1 is required");
        print!("{}", usage);
    }

    if !matches.opt_present("s") && !matches.opt_present("d") && matches.free.len() != 2 {
        println!("SET1 and SET2 are required for translation.");
        println!("");
        print!("{}", usage);
        return;
    }

    if matches.opt_present("s") {
        if matches.opt_present("d") && matches.free.len() == 1 {
            println!("Two strings must be given when both deleting and squeezing repeats.");
            println!("");
            print!("{}", usage);
            return;
        }
    }

    tr(&matches);
}

fn tr(matches: &Matches) {
    let delete_flag: bool = matches.opt_present("d");
    let complement_flag: bool = matches.opts_present(&["c".to_string(), "C".to_string()]);
    let truncate_flag: bool = matches.opt_present("t");
    let translating: bool = !matches.opt_present("d") && matches.free.len() == 2;
    let deleting: bool = matches.opt_present("d");
    let squeezing: bool = matches.opt_present("s");

    let mut set1 = Arc::new(match matches.free.first() {
        Some(s) => s.clone(),
        None => String::new()
    });
    let set1_len = set1.as_slice().chars().count();
    let set2 = Arc::new(match matches.free.get(1) {
        Some(s) => s.clone(),
        None => String::new()
    });
    let set2_len = set2.as_slice().chars().count();

    //if translating && truncate_flag && set1_len > set2_len {
        //set1 = Arc::new(set1.as_slice().chars().take(set2_len).collect());
    //} else if set2_len > set1_len {
        //let last = set1.as_slice().chars().last();
        //set1 = Arc::new(set1.as_slice().chain(range(1, set2_len - set1_len).map(|x| last))
                            //.collect());
    //}

    let first_set = set1.clone();
    //let translation_set = set2.clone();
    let (squeeze_set, squeeze_complement) = if !delete_flag && matches.free.len() == 1 {
        (set1.clone(), complement_flag)
    } else {
        (set2.clone(), false)
    };

    let guard = taskpipe::input(|tx: Sender<char>| {
        let mut input = stdin();

        loop {
            match input.read_char() {
                Err(_) => break,
                Ok(c) => tx.send(c).unwrap()
            };
        };
    }).pipe(move |rx: Receiver<char>, tx: Sender<char> | {
        if deleting {
            // deleting
            if complement_flag {
                for c in rx.iter() {
                    if first_set.chars().any(|s| { s == c }) {
                        tx.send(c).unwrap();
                    }
                }
            } else {
                for c in rx.iter() {
                    if !first_set.chars().any(|s| { s == c }) {
                        tx.send(c).unwrap();
                    }
                }
            }
        } else {
            // translating
            pump(&rx, &tx);
        }
    }).pipe(move |rx: Receiver<char>, tx: Sender<char> | {
        let mut itr = rx.iter();

        if squeezing {
            // sqeezing
            let mut last = itr.next().unwrap();
            tx.send(last).unwrap();

            if squeeze_complement {
                for c in itr {
                    if c == last && !squeeze_set.chars().any(|s| { s == c }) {
                        continue;
                    } else {
                        tx.send(c).unwrap();
                    }
                    last = c;
                }
            } else {
                for c in itr {
                    if c == last && squeeze_set.chars().any(|s| { s == c }) {
                        continue;
                    } else {
                        tx.send(c).unwrap();
                    }
                    last = c;
                }
            }
        } else {
            pump(&rx, &tx);
        }
    }).end(|rx: Receiver<char>| {
        let mut out = stdout_raw();
        for c in rx.iter() {
            match out.write_char(c) {
                Err(e) => panic!("{}", e),
                Ok(_) => continue
            };
        }
    });

    match guard.join() {
        Ok(_) => return,
        Err(_) => return
    };
}

fn pump(rx: &Receiver<char>, tx: &Sender<char>) {
    for x in rx.iter() {
        tx.send(x).unwrap();
    }
}
