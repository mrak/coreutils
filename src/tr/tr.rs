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

    tr(&matches);
}

fn tr(matches: &Matches) {
    let delete_flag: bool = matches.opt_present("d");
    let squeeze_flag: bool = matches.opt_present("s");
    let complement_flag: bool = matches.opts_present(&["c".to_string(), "C".to_string()]);
    //let truncate_flag: bool = matches.opt_present("t");

    let set1 = Arc::new(match matches.free.first() {
        Some(s) => s.clone(),
        None => String::new()
    });
    let set2 = Arc::new(match matches.free.get(1) {
        Some(s) => s.clone(),
        None => String::new()
    });

    let delete_set = set1.clone();
    let squeeze_set = if delete_flag { set2.clone() } else { set1.clone() };

    let guard = taskpipe::input(|tx: Sender<char>| {
        let mut input = stdin();

        loop {
            match input.read_char() {
                Err(_) => break,
                Ok(c) => tx.send(c).unwrap()
            };
        };
    }).pipe(move |rx: Receiver<char>, tx: Sender<char> | {
        if delete_flag {
            for c in rx.iter() {
                if !complement_flag {
                    if delete_set.chars().any(|s| { s == c }) {
                        continue;
                    }
                } else {
                    if !delete_set.chars().any(|s| { s == c }) {
                        continue;
                    }
                }

                tx.send(c).unwrap();
            }
        } else {
            pump(&rx, &tx);
        }
    }).pipe(move |rx: Receiver<char>, tx: Sender<char> | {
        let mut itr = rx.iter();

        if squeeze_flag {
            let mut last = itr.next().unwrap();
            tx.send(last).unwrap();

            for c in itr {
                if c == last && squeeze_set.chars().any(|s| { s == c }) {
                    continue;
                }
                tx.send(c).unwrap();
                last = c;
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
