mod action;

use self::action::*;

// use serde::{Deserialize, Serialize};
use crate::cmd::*;
use crate::kmn_serde::*;
use crate::*;

// use rand::Rng;
// use std::error::Error;
// use std::fmt::Write;
use std::io;
// use std::str::FromStr;

// read line of input from io::stdin()
pub fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line from input)");
    input
}

fn input_menu(assignments_data: &mut Option<Assignments>) {
    'input: loop {
        println!("\nDEFINE ASSIGNMENTS");
        println!("-> Input command (h for help): ");
        let cmd = read_line();
        let cmd = cmd.trim();
        match cmd {
            "h" => {
                println!(
                    "
        command action:
            kmn        input k,m,n parameters for deafult assignments
            mn         input parameters m,n and then either k or p for deafult assignments
            json       input one-line JSON assignments data
            quit       quit 'DEFINE ASSIGNMENTS' menu without defining assignments
            "
                );
            }
            "quit" => {
                break 'input;
            }
            "kmn" => {
                println!("{}: input: k m n (1 <= k <= m <= n)", cmd);
                let input = read_line();
                match split_and_parse_input::<usize>(&input, 3) {
                    Ok(args) => {
                        let (k, m, n) = (args[0], args[1], args[2]);
                        if !(1 <= k && k <= m && m <= n) {
                            println!(
                                "You have input (k,m,n)={:?}, that does not meet the condition: 1 <= k <= m <= n !!!",
                                (k, m, n)
                            );
                            continue 'input;
                        } else {
                            // Ok,  set the assignments !
                            let assignments = Assignments::new(k, m, n);
                            *assignments_data = Some(assignments);
                            println!("Default assignments for (k, m, n)=({},{},{}) set!", k, m, n);
                            break 'input; // go to the loop of commands
                        }
                    }
                    Err(err) => {
                        println!("{}", err);
                        continue 'input; // try again!
                    }
                }
            }
            "mn" => {
                println!("{}: input: m n and then either k or p", cmd);
                let input = read_line();
                match split_and_parse_input::<usize>(&input, 2) {
                    Ok(args) => {
                        let (m, n) = (args[0], args[1]);
                        if m <= n {
                            println!("{cmd}: input: k, (1 <= k <= {m})");
                        } else {
                            println!("{cmd}: input: p, (1 <= p <= {n})");
                        }

                        match split_and_parse_input::<usize>(&read_line(), 1) {
                            Err(err) => {
                                println!("{err}");
                                continue 'input;
                            }
                            Ok(k_or_p) => {
                                let k_or_p = k_or_p[0];
                                if k_or_p < 1 {
                                    println!("You have input {k_or_p} < 1 !!!");
                                    continue 'input;
                                } else if m <= n && m < k_or_p {
                                    println!("You have input {k_or_p} > m !!!");
                                    continue 'input;
                                } else if n < m && n < k_or_p {
                                    println!("You have input {k_or_p} > n !!!");
                                    continue 'input;
                                } else {
                                    // k_or_p is correct either k or p
                                    let (k, p): (Option<usize>, Option<usize>);
                                    if m <= n {
                                        (k, p) = (Some(k_or_p), None);
                                    } else {
                                        (k, p) = (None, Some(k_or_p));
                                    }
                                    // let _pairs = Pairs::kmnp_pairs(k, m, n, p);
                                    let assignments = Assignments::new_kmnp(k, m, n, p);
                                    *assignments_data = Some(assignments);
                                    println!(
                                        "Default assignments for (k, m, n, p)=({k:?},{m},{n},{p:?}) set!"
                                    );
                                    break 'input; // go to the loop of commands
                                }
                            }
                        }
                    }
                    Err(err) => {
                        println!("{}", err);
                        continue 'input; // try again!
                    }
                }
            }
            "json" => {
                println!("input one-line json: ");
                let input = read_line();
                let deserialized: Result<SerdeKmnAssignment, serde_json::Error> =
                    serde_json::from_str(&input);
                match deserialized {
                    Ok(deserialized) => {
                        let mut assignments = Assignments::from(&deserialized);
                        match assignments.test_assignments() {
                            Ok(()) => {
                                if let Err(err) = &assignments.test_forbidden() {
                                    println!("{}", err);
                                }
                                *assignments_data = Some(assignments);
                                println!("Assignments set!");
                                break 'input;
                            }
                            Err(err) => {
                                println!("{}", err);
                                continue 'input; // try again!
                            }
                        }
                    }
                    Err(err) => {
                        println!("{}", err);
                        continue 'input; // try again!
                    }
                }
            }
            _ => println!("Unknown command: {}", cmd),
        }
    }
}

// `kmn_pairs_menu` - define and update Assignments
pub fn kmn_pairs_menu(assignments_data: &mut Option<Assignments>) {
    let mut rng = rand::rng(); // random number generator
    if assignments_data.is_none() {
        input_menu(assignments_data);
    }
    if let Some(assignments) = assignments_data {
        'cmd: loop {
            // MENU - Actions on the assigments:
            loop {
                // let (_k, m, n) = assignments.get_kmn(); // get the assignments' parameters
                println!(
                    "\nEDIT ASSIGNMENTS\n{}\n{}\n{}",
                    assignments.assignments_header(),
                    assignments.forbidden_header(),
                    assignments.backup_header(),
                );
                println!("-> Input command (h for help): ");
                let cmd = read_line();
                let cmd = cmd.trim();
                match cmd {
                    "h" => {
                        println!(
                            "
        command action:
            p        print current state of assignments and forbidden
            pa       print only assigments
            palvrvj  print assigments for each left and for each right as one-line JSONs
            pf       print only fordidden
            pflvrvj  print fordidden for each left and for each right as one-line JSONs
            pfa      print only fordidden in assignments
            json     print one-line JSON assignments data
            gl       group assignments by left
            gr       group assignments by right
            fgl      group (sort) forbidden by left
            fgr      group (sort) forbidden by right
            rl       randomly permute left IDs
            rr       randomly permute right IDs
            rlr      randomly permute left and right IDs
            sl       swap left IDs of forbidden with random other left IDs
            sr       swap right IDs of forbidden with random other right IDs
            slr      swap left and right IDs of forbidden with random other left and right IDs
            bsr      back and swap right IDs of forbidden with random other right IDs
            bslr     back and swap left and right IDs of forbidden with random other left and right IDs
            sbrk     'skeleton breaking' (result may be not isomorphic)
            back     go back to the backup with minimal forbidden pairs in assigments
            af       add forbidden pairs
            arf      try to add some random forbidden pairs
            arfl     try to add some random forbidden pairs with given left id
            arfr     try to add some random forbidden pairs with given right id
            aflvrvj  add to forbidden all the pairs between left and right vectors from one-line JSONs
            df       delete forbidden pair (l, r) (prints deleted pairs)
            dfl      delete all forbidden pairs with left ID l (prints deleted pairs)
            dfr      delete all forbidden pairs with right ID r (prints deleted pairs)
            test     do some tests ...
            quit     quit the 'EDIT ASSIGNMENTS' menu (prints JSON assignments data)
"
                        );
                    }
                    "p" => {
                        println!("{}", assignments);
                    }
                    "pa" => {
                        println!(
                            "{}{}",
                            assignments.assignments_header(),
                            assignments.assignments_body()
                        );
                    }
                    "palvrvj" => {
                        palvrvj(assignments);
                    }
                    "pf" => {
                        println!(
                            "{}{}",
                            assignments.forbidden_header(),
                            assignments.forbidden_body()
                        );
                    }
                    "pfa" => {
                        println!("{}", assignments.assignments_in_forbidden());
                    }
                    "pflvrvj" => {
                        pflvrvj(assignments);
                    }
                    "json" => {
                        match serde_json::to_string(&SerdeKmnAssignment::from(&*assignments)) {
                            Ok(out) => {
                                println!("{}", out)
                            }
                            Err(err) => {
                                println!("{}", err)
                            }
                        }
                    }

                    "rl" => {
                        rl(assignments, &mut rng);
                    }
                    "rr" => {
                        rr(assignments, &mut rng);
                    }
                    "rlr" => {
                        rlr(assignments, &mut rng);
                    }
                    "sl" => {
                        sl(assignments, &mut rng);
                    }
                    "sr" => {
                        sr(assignments, &mut rng);
                    }
                    "slr" => {
                        slr(assignments, &mut rng);
                    }
                    "back" => {
                        back(assignments);
                    }
                    "bsr" => {
                        bsr(assignments, &mut rng);
                    }
                    "bslr" => {
                        bslr(assignments, &mut rng);
                    }
                    "sbrk" => {
                        sbrk(assignments);
                    }
                    "gl" => {
                        assignments.group_by_left();
                        println!("Grouped by left.");
                    }
                    "gr" => {
                        assignments.group_by_right();
                        println!("Grouped by right.");
                    }
                    "fgl" => {
                        assignments.forbidden.sort_by(|a, b| a.cmp(&b));
                        println!("Grouped by left.");
                    }
                    "fgr" => {
                        assignments.forbidden.sort_by(|a, b| {
                            let (la, ra) = a;
                            let (lb, rb) = b;
                            (ra, la).cmp(&(rb, lb))
                        });
                        println!("Grouped by right.");
                    }
                    "df" => {
                        df(assignments);
                    }
                    "dfl" => {
                        dfl(assignments);
                    }
                    "dfr" => {
                        dfr(assignments);
                    }
                    "af" => {
                        af(assignments);
                    }
                    "arf" => {
                        arf(assignments, &mut rng);
                    }
                    "arfl" => {
                        arfl(assignments, &mut rng);
                    }
                    "arfr" => {
                        arfr(assignments, &mut rng);
                    }
                    "aflvrvj" => {
                        aflvrvj(assignments);
                    }
                    "test" => {
                        println!("test_assignments ...");
                        if let Err(err) = assignments.test_assignments() {
                            println!("{}", err);
                        } else {
                            println!("Implemented tests passed.");
                        }
                        println!("test_forbidden ...");
                        if let Err(err) = assignments.test_forbidden() {
                            println!("{}", err);
                        } else {
                            println!("Implemented tests passed.");
                        }
                    }
                    "quit" => {
                        println!(
                            "ASSIGNMENTS JSON:\n{}\n",
                            match serde_json::to_string(&SerdeKmnAssignment::from(&*assignments)) {
                                Ok(out) => out,
                                Err(err) => err.to_string(),
                            }
                        );

                        break 'cmd;
                        // return;
                    }
                    _ => println!("Unknown command: {}", cmd),
                }
            }
        }
    } else {
        println!("NO DATA !!!");
    }
}
