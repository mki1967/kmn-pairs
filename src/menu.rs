use crate::cmd::*;
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

// `kmn_pairs_menu` - define and update Assignments
pub fn kmn_pairs_menu(assignments_data: &mut Option<Assignments>) {
    let mut rng = rand::rng(); // random number generator
    loop {
        // let mut assignments: Assignments;
        loop {
            println!("input k m n (1 <= k <= m <= n): ");
            let input = read_line();
            match split_and_parse_input::<usize>(&input, 3) {
                Ok(args) => {
                    let (k, m, n) = (args[0], args[1], args[2]);
                    if !(1 <= k && k <= m && m <= n) {
                        println!(
                            "You have input (k,m,n)={:?}, that does not meet the condition: 1 <= k <= m <= n !!!",
                            (k, m, n)
                        );
                        continue;
                    } else {
                        // Ok,  set the assignments !
                        let assignments = Assignments::new(k, m, n);
                        *assignments_data = Some(assignments);
                        println!("Default assignments for (k, m, n)=({},{},{}) set!", k, m, n);
                        break; // go to the loop of commands
                    }
                }
                Err(err) => {
                    println!("{}", err);
                    continue; // try again!
                }
            }
        }

        if let Some(assignments) = assignments_data {
            // MENU - Actions on the assigments:
            loop {
                let (_k, m, n) = assignments.get_kmn(); // get the assignments' parameters
                println!(
                    "\n{}\n{}\n{}",
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
            pa       print only assigsments
            pf       print only fordidden
            pfa      print only fordidden in assignments
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
            test     do some tests ...
            restart  restart with new parameters: k,m,n
            quit     end the program
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
                    "rl" => {
                        println!(
                            "{}: input: max (0 <= max) for max trials to find assignments without forbidden: ",
                            cmd
                        );
                        let input = read_line();
                        match split_and_parse_input::<usize>(&input, 1) {
                            Ok(args) => {
                                let max = args[0];
                                println!("max = {}", max);
                                let (Steps(l_steps), Steps(r_steps), Forbidden(f)) =
                                    assignments.randomize_permutation(Side::Left, max, &mut rng);
                                println!(
                                    "{}: After ({},{}) (left,right)-steps, {}-forbidden-assignment backuped.",
                                    cmd, l_steps, r_steps, f
                                );
                            }
                            Err(err) => {
                                println!("{}", err);
                                continue; // try again!
                            }
                        }
                    }
                    "rr" => {
                        println!(
                            "{}: input: max (0 <= max) for max trials to find assignments without forbidden: ",
                            cmd
                        );
                        let input = read_line();
                        match split_and_parse_input::<usize>(&input, 1) {
                            Ok(args) => {
                                let max = args[0];
                                println!("max = {}", max);
                                let (Steps(l_steps), Steps(r_steps), Forbidden(f)) =
                                    assignments.randomize_permutation(Side::Right, max, &mut rng);
                                println!(
                                    "{}: After ({},{}) (left,right)-steps, {}-forbidden-assignment backuped.",
                                    cmd, l_steps, r_steps, f
                                );
                            }
                            Err(err) => {
                                println!("{}", err);
                                continue; // try again!
                            }
                        }
                    }
                    "rlr" => {
                        println!(
                            "{}: input: max l_percent (0 <= max and 0<= l_percent <= 100): ",
                            cmd
                        );
                        let input = read_line();
                        match split_and_parse_input::<usize>(&input, 2) {
                            Ok(args) => {
                                let (max, l_percent) = (args[0], args[1]);
                                println!("max = {}, l_percent = {}", max, l_percent);
                                let (Steps(l_steps), Steps(r_steps), Forbidden(f)) = assignments
                                    .randomize_permutation(
                                        Side::LeftPercent(l_percent),
                                        max,
                                        &mut rng,
                                    );
                                println!(
                                    "{}: After ({},{}) (left,right)-steps, {}-forbidden-assignment backuped.",
                                    cmd, l_steps, r_steps, f
                                );
                            }
                            Err(err) => {
                                println!("{}", err);
                                continue; // try again!
                            }
                        }
                    }
                    "sl" => {
                        println!(
                            "{}: input: max (0 <= max) for max trials to find assignments without forbidden: ",
                            cmd
                        );
                        let input = read_line();
                        match split_and_parse_input::<usize>(&input, 1) {
                            Ok(args) => {
                                let max = args[0];
                                println!("max = {}", max);
                                let (Steps(l_steps), Steps(r_steps), Forbidden(f)) =
                                    assignments.random_swaps(Side::Left, max, &mut rng);
                                println!(
                                    "{}: After ({},{}) (left,right)-steps, {}-forbidden-assignment backuped.",
                                    cmd, l_steps, r_steps, f
                                );
                            }
                            Err(err) => {
                                println!("{}", err);
                                continue; // try again!
                            }
                        }
                    }
                    "sr" => {
                        println!(
                            "{}: input: max (0 <= max) for max trials to find assignments without forbidden: ",
                            cmd
                        );
                        let input = read_line();
                        match split_and_parse_input::<usize>(&input, 1) {
                            Ok(args) => {
                                let max = args[0];
                                println!("max = {}", max);
                                let (Steps(l_steps), Steps(r_steps), Forbidden(f)) =
                                    assignments.random_swaps(Side::Right, max, &mut rng);
                                println!(
                                    "{}: After ({},{}) (left,right)-steps, {}-forbidden-assignment backuped.",
                                    cmd, l_steps, r_steps, f
                                );
                            }
                            Err(err) => {
                                println!("{}", err);
                                continue; // try again!
                            }
                        }
                    }
                    "slr" => {
                        println!(
                            "{}: input: max l_percent (0 <= max and 0<= l_percent <= 100): ",
                            cmd
                        );
                        let input = read_line();
                        match split_and_parse_input::<usize>(&input, 2) {
                            Ok(args) => {
                                let (max, l_percent) = (args[0], args[1]);
                                println!("max = {}, l_percent = {}", max, l_percent);
                                let (Steps(l_steps), Steps(r_steps), Forbidden(f)) = assignments
                                    .random_swaps(Side::LeftPercent(l_percent), max, &mut rng);
                                println!(
                                    "{}: After ({},{}) (left,right)-steps, {}-forbidden-assignment backuped.",
                                    cmd, l_steps, r_steps, f
                                );
                            }
                            Err(err) => {
                                println!("{}", err);
                                continue; // try again!
                            }
                        }
                    }
                    "back" => {
                        let backup = assignments.f_min_backup.clone();
                        if let Some(pairs) = backup {
                            let tmp = assignments.get_pairs_of_ids();
                            assignments.set_pairs_of_ids(&pairs);
                            let old_f = intersection_size(&tmp, &assignments.forbidden);
                            if old_f < intersection_size(&pairs, &assignments.forbidden) {
                                println!("saving {}-backup from current assignments", old_f);
                                assignments.f_min_backup = Some(tmp);
                            }
                            println!("{}: Backup restored.", cmd);
                        } else {
                            println!("{}: There is no backup !!!", cmd);
                        }
                    }
                    "bsr" => {
                        println!(
                            "{}: input: max (0 <= max) for max trials to find assignments without forbidden: ",
                            cmd
                        );
                        let input = read_line();
                        match split_and_parse_input::<usize>(&input, 1) {
                            Ok(args) => {
                                let max = args[0];
                                println!("max = {}", max);
                                let (Steps(l_steps), Steps(r_steps), Forbidden(f)) =
                                    assignments.random_back_swaps(Side::Right, max, &mut rng);
                                println!(
                                    "{}: After ({},{}) (left,right)-steps, {}-forbidden-assignment backuped.",
                                    cmd, l_steps, r_steps, f
                                );
                            }
                            Err(err) => {
                                println!("{}", err);
                                continue; // try again!
                            }
                        }
                    }
                    "bslr" => {
                        println!(
                            "{}: input: max l_percent (0 <= max and 0<= l_percent <= 100): ",
                            cmd
                        );
                        let input = read_line();
                        match split_and_parse_input::<usize>(&input, 2) {
                            Ok(args) => {
                                let (max, l_percent) = (args[0], args[1]);
                                println!("max = {}, l_percent = {}", max, l_percent);
                                let (Steps(l_steps), Steps(r_steps), Forbidden(f)) = assignments
                                    .random_back_swaps(Side::LeftPercent(l_percent), max, &mut rng);
                                println!(
                                    "{}: After ({},{}) (left,right)-steps, {}-forbidden-assignment backuped.",
                                    cmd, l_steps, r_steps, f
                                );
                            }
                            Err(err) => {
                                println!("{}", err);
                                continue; // try again!
                            }
                        }
                    }
                    "sbrk" => {
                        let pairs = assignments.get_pairs_of_ids();
                        let result = assignments.try_switching_endpoints(pairs);
                        match result {
                            Ok(pairs) => {
                                let tmp = assignments.get_pairs_of_ids();
                                assignments.set_pairs_of_ids(&pairs);
                                if let Err(err) = assignments.test_assignments() {
                                    println!("{}", assignments);
                                    println!("{}", err);
                                    assignments.set_pairs_of_ids(&tmp);
                                    println!("Old restored!");
                                } else {
                                    println!("Implemented tests of assignment passed.");
                                };
                            }
                            Err(err) => {
                                println!("{}", err)
                            }
                        }
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
                        println!("Grouped by right.");
                    }
                    "fgr" => {
                        assignments.forbidden.sort_by(|a, b| {
                            let (la, ra) = a;
                            let (lb, rb) = b;
                            (ra, la).cmp(&(rb, lb))
                        });
                        println!("Grouped by right.");
                    }
                    "af" => {
                        loop {
                            println!(
                                "{}: input: l r (0 <= l < {} and 0 <= r < {}) or something else to finish: ",
                                cmd, m, n
                            );
                            let input = read_line();
                            match split_and_parse_input::<usize>(&input, 2) {
                                Ok(args) => {
                                    let (l, r) = (args[0], args[1]);
                                    if !(l < m && r < n) {
                                        println!(
                                            "You have input (l, r)=({}, {}) that does not meet the condition: 0 <= l < {} and 0 <= r < {} !!!",
                                            l, r, m, n
                                        );
                                        break;
                                    } else {
                                        // Ok, do "af"
                                        if let Err(str) = assignments.add_forbidden(l, r) {
                                            println!("{}", str);
                                        } else {
                                            println!("added forbidden: {:?}:", (l, r));
                                        }
                                    }
                                }
                                Err(err) => {
                                    println!("{}", err);
                                    break; // stop the `af` command
                                }
                            }
                        }
                    }
                    "arf" => {
                        println!("{}: input: max (0 <= max):", cmd);
                        let input = read_line();
                        match split_and_parse_input::<usize>(&input, 1) {
                            Ok(args) => {
                                let num = args[0];
                                let mut count = 0;
                                for _i in 0..num {
                                    let l = rng.random_range(0..m);
                                    let r = rng.random_range(0..n);
                                    if let Err(str) = assignments.add_forbidden(l, r) {
                                        println!("{}", str);
                                    } else {
                                        println!("added forbidden: {:?}:", (l, r));
                                        count += 1;
                                    }
                                }
                                println!("{}: added {} random forbidden.", cmd, count);
                            }
                            Err(err) => {
                                println!("{}", err);
                                continue; // try again!
                            }
                        }
                    }
                    "arfl" => {
                        println!("{}: input: max l (0 <= max && l < {}):", cmd, m);
                        let input = read_line();
                        match split_and_parse_input::<usize>(&input, 2) {
                            Ok(args) => {
                                let (num, l) = (args[0], args[1]);
                                if l < m {
                                    let mut count = 0;
                                    for _i in 0..num {
                                        let r = rng.random_range(0..n);
                                        if let Err(str) = assignments.add_forbidden(l, r) {
                                            println!("{}", str);
                                        } else {
                                            println!("added forbidden: {:?}:", (l, r));
                                            count += 1;
                                        }
                                    }
                                    println!("{}: added {} random forbidden.", cmd, count);
                                } else {
                                    println!("{}: Bad input: l = {} >= {} !!!", cmd, l, m);
                                }
                            }
                            Err(err) => {
                                println!("{}", err);
                                continue; // try again!
                            }
                        }
                    }
                    "arfr" => {
                        println!("{}: input: max r (0 <= max && r < {}):", cmd, n);
                        let input = read_line();
                        match split_and_parse_input::<usize>(&input, 2) {
                            Ok(args) => {
                                let (num, r) = (args[0], args[1]);
                                if r < n {
                                    let mut count = 0;
                                    for _i in 0..num {
                                        let l = rng.random_range(0..m);
                                        if let Err(str) = assignments.add_forbidden(l, r) {
                                            println!("{}", str);
                                        } else {
                                            println!("added forbidden: {:?}:", (l, r));
                                            count += 1;
                                        }
                                    }
                                    println!("{}: added {} random forbidden.", cmd, count);
                                } else {
                                    println!("{}: Bad input: r = {} >= {} !!!", cmd, r, n);
                                }
                            }
                            Err(err) => {
                                println!("{}", err);
                                continue; // try again!
                            }
                        }
                    }
                    "test" => {
                        println!("test_assignments ...");
                        if let Err(err) = assignments.test_assignments() {
                            println!("{}", err);
                        } else {
                            println!("Imlemented tests passed.");
                        }
                        println!("test_forbidden ...");
                        if let Err(err) = assignments.test_forbidden() {
                            println!("{}", err);
                        } else {
                            println!("Implemented tests passed.");
                        }
                    }
                    "restart" => {
                        println!(" Restarting !!!");
                        break;
                    }
                    "quit" => {
                        println!("THE END!");
                        return;
                    }
                    _ => println!("Unknown command: {}", cmd),
                }
            }
        } else {
            println!("NO DATA !!!");
        }
    }
}
