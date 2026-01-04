// action of kmn_pairs menu

use crate::cmd::*;
// use crate::kmn_serde::*;
use crate::menu::*;
use crate::*;

// kmn_pairs_menu actions

// print one-line JSONs for left IDs
pub fn lvrvj_l(assignments: &Assignments, pairs: &Vec<(usize, usize)>) {
    let (_k, m, _n) = assignments.get_kmn();
    for l in 0..m {
        LeftVecRightVec {
            left: vec![l],
            right: right_neighbors(pairs, l),
        }
        .println_serde();
    }
}

// print one-line JSONs for right IDs
pub fn lvrvj_r(assignments: &Assignments, pairs: &Vec<(usize, usize)>) {
    let (_k, _m, n) = assignments.get_kmn();
    for r in 0..n {
        LeftVecRightVec {
            left: left_neighbors(pairs, r),
            right: vec![r],
        }
        .println_serde();
    }
}

pub fn pflvrvj(assignments: &Assignments) {
    let cmd = "pflvrvj";
    let pairs = &assignments.forbidden;

    println!("\n{cmd}: FORBIDDEN for each left:");
    lvrvj_l(assignments, pairs);
    println!("\n{cmd}: FORBIDDEN for each right:");
    lvrvj_r(assignments, pairs);
}

pub fn palvrvj(assignments: &Assignments) {
    let cmd = "palvrvj";
    let pairs = &assignments.get_pairs_of_ids();

    println!("\n{cmd}: ASSIGNMENTS for each left:");
    lvrvj_l(assignments, pairs);
    println!("\n{cmd}: ASSIGNMENTS for each right:");
    lvrvj_r(assignments, pairs);
}

pub fn rl(assignments: &mut Assignments, rng: &mut impl Rng) {
    let cmd = "rl";
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
                assignments.randomize_permutation(Side::Left, max, rng);
            println!(
                "{}: After ({},{}) (left,right)-steps, {}-forbidden-assignment backuped.",
                cmd, l_steps, r_steps, f
            );
        }
        Err(err) => {
            println!("{cmd}: {}", err);
            // continue 'cmd; // try again!
        }
    }
}

pub fn rr(assignments: &mut Assignments, rng: &mut impl Rng) {
    let cmd = "rr";
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
                assignments.randomize_permutation(Side::Right, max, rng);
            println!(
                "{}: After ({},{}) (left,right)-steps, {}-forbidden-assignment backuped.",
                cmd, l_steps, r_steps, f
            );
        }
        Err(err) => {
            println!("{cmd}: {}", err);
            // continue 'cmd; // try again!
        }
    }
}

pub fn rlr(assignments: &mut Assignments, rng: &mut impl Rng) {
    let cmd = "rlr";
    println!(
        "{}: input: max l_percent (0 <= max and 0<= l_percent <= 100): ",
        cmd
    );
    let input = read_line();
    match split_and_parse_input::<usize>(&input, 2) {
        Ok(args) => {
            let (max, l_percent) = (args[0], args[1]);
            println!("max = {}, l_percent = {}", max, l_percent);
            let (Steps(l_steps), Steps(r_steps), Forbidden(f)) =
                assignments.randomize_permutation(Side::LeftPercent(l_percent), max, rng);
            println!(
                "{}: After ({},{}) (left,right)-steps, {}-forbidden-assignment backuped.",
                cmd, l_steps, r_steps, f
            );
        }
        Err(err) => {
            println!("{cmd}: {}", err);
            // continue 'cmd; // try again!
        }
    }
}

pub fn sl(assignments: &mut Assignments, rng: &mut impl Rng) {
    let cmd = "sl";
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
                assignments.random_swaps(Side::Left, max, rng);
            println!(
                "{}: After ({},{}) (left,right)-steps, {}-forbidden-assignment backuped.",
                cmd, l_steps, r_steps, f
            );
        }
        Err(err) => {
            println!("{cmd}: {}", err);
            // continue 'cmd; // try again!
        }
    }
}

pub fn sr(assignments: &mut Assignments, rng: &mut impl Rng) {
    let cmd = "sr";
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
                assignments.random_swaps(Side::Right, max, rng);
            println!(
                "{}: After ({},{}) (left,right)-steps, {}-forbidden-assignment backuped.",
                cmd, l_steps, r_steps, f
            );
        }
        Err(err) => {
            println!("{cmd}: {}", err);
            // continue 'cmd; // try again!
        }
    }
}

pub fn slr(assignments: &mut Assignments, rng: &mut impl Rng) {
    let cmd = "slr";
    println!(
        "{}: input: max l_percent (0 <= max and 0<= l_percent <= 100): ",
        cmd
    );
    let input = read_line();
    match split_and_parse_input::<usize>(&input, 2) {
        Ok(args) => {
            let (max, l_percent) = (args[0], args[1]);
            println!("max = {}, l_percent = {}", max, l_percent);
            let (Steps(l_steps), Steps(r_steps), Forbidden(f)) =
                assignments.random_swaps(Side::LeftPercent(l_percent), max, rng);
            println!(
                "{}: After ({},{}) (left,right)-steps, {}-forbidden-assignment backuped.",
                cmd, l_steps, r_steps, f
            );
        }
        Err(err) => {
            println!("{cmd}: {}", err);
            // continue 'cmd; // try again!
        }
    }
}

pub fn back(assignments: &mut Assignments) {
    let cmd = "back";
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

pub fn bsr(assignments: &mut Assignments, rng: &mut impl Rng) {
    let cmd = "bsr";
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
                assignments.random_back_swaps(Side::Right, max, rng);
            println!(
                "{}: After ({},{}) (left,right)-steps, {}-forbidden-assignment backuped.",
                cmd, l_steps, r_steps, f
            );
        }
        Err(err) => {
            println!("{cmd}: {}", err);
            // continue 'cmd; // try again!
        }
    }
}

pub fn bslr(assignments: &mut Assignments, rng: &mut impl Rng) {
    let cmd = "bslr";
    println!(
        "{}: input: max l_percent (0 <= max and 0<= l_percent <= 100): ",
        cmd
    );
    let input = read_line();
    match split_and_parse_input::<usize>(&input, 2) {
        Ok(args) => {
            let (max, l_percent) = (args[0], args[1]);
            println!("max = {}, l_percent = {}", max, l_percent);
            let (Steps(l_steps), Steps(r_steps), Forbidden(f)) =
                assignments.random_back_swaps(Side::LeftPercent(l_percent), max, rng);
            println!(
                "{}: After ({},{}) (left,right)-steps, {}-forbidden-assignment backuped.",
                cmd, l_steps, r_steps, f
            );
        }
        Err(err) => {
            println!("{cmd}: {}", err);
            // continue 'cmd; // try again!
        }
    }
}

pub fn sbrk(assignments: &mut Assignments) {
    let cmd = "sbrk";
    let pairs = assignments.get_pairs_of_ids();
    let result = assignments.try_switching_endpoints(pairs);
    match result {
        Ok(pairs) => {
            let tmp = assignments.get_pairs_of_ids();
            assignments.set_pairs_of_ids(&pairs);
            if let Err(err) = assignments.test_assignments() {
                println!("{}", assignments);
                println!("{cmd}: {}", err);
                assignments.set_pairs_of_ids(&tmp);
                println!("{cmd}: Old restored!");
            } else {
                println!("{cmd}: Implemented tests of assignment passed.");
            };
        }
        Err(err) => {
            println!("{}", err)
        }
    }
}

pub fn df(assignments: &mut Assignments) {
    let cmd = "df";
    let (_k, m, n) = assignments.get_kmn();
    println!("{cmd}: input l r (0 <= l <{m} and 0<= r <={n})");
    let input = read_line();
    match split_and_parse_input::<usize>(&input, 2) {
        Ok(args) => {
            let (l, r) = (args[0], args[1]);
            if !(l < m && r < n) {
                println!(
                    "You have input (l, r)=({l}, {r}) that does not meet the condition: 0 <= l < {m} and 0 <= r < {n} !!!",
                );
                // break;
            } else {
                // Ok, do "df"
                match serde_json::to_string(&assignments.extract_forbidden_by(|x| x == (l, r))) {
                    Ok(out) => {
                        println!("extracted:\n{}", out)
                    }
                    Err(err) => {
                        println!("{}", err)
                    }
                }
            }
        }
        Err(err) => {
            println!("{cmd}: {}", err);
            // break; // stop the `af` command
        }
    }
}

pub fn dfl(assignments: &mut Assignments) {
    let cmd = "dfl";
    let (_k, m, _n) = assignments.get_kmn();
    'dfl: loop {
        println!("{cmd}: input l (0 <= l < {m})");
        let input = read_line();
        match split_and_parse_input::<usize>(&input, 1) {
            Ok(args) => {
                let l = args[0];
                if !(l < m) {
                    println!(
                        "You have input l ={l} that does not meet the condition: 0 <= l < {m} !!!",
                    );
                    // break;
                } else {
                    // Ok, do "df"
                    match serde_json::to_string(&assignments.extract_forbidden_by(|x| {
                        let (q, _) = x;
                        q == l
                    })) {
                        Ok(out) => {
                            println!("extracted:\n{}", out)
                        }
                        Err(err) => {
                            println!("{}", err)
                        }
                    }
                }
            }
            Err(err) => {
                println!("{cmd}: {}", err);
                break 'dfl; // stop the command
            }
        }
    }
}

pub fn dfr(assignments: &mut Assignments) {
    let cmd = "dfr";
    let (_k, _m, n) = assignments.get_kmn();
    'dfr: loop {
        println!("{cmd}: input r (0 <= r < {n})");
        let input = read_line();
        match split_and_parse_input::<usize>(&input, 1) {
            Ok(args) => {
                let r = args[0];
                if !(r < n) {
                    println!(
                        "You have input r ={r} that does not meet the condition: 0 <= r < {n} !!!",
                    );
                    // break;
                } else {
                    // Ok, do "df"
                    match serde_json::to_string(&assignments.extract_forbidden_by(|x| {
                        let (_, q) = x;
                        q == r
                    })) {
                        Ok(out) => {
                            println!("extracted:\n{}", out)
                        }
                        Err(err) => {
                            println!("{}", err)
                        }
                    }
                }
            }
            Err(err) => {
                println!("{cmd}: {}", err);
                break 'dfr; // stop the command
            }
        }
    }
}

pub fn af(assignments: &mut Assignments) {
    let cmd = "af";
    let (_k, m, n) = assignments.get_kmn();
    'af: loop {
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
                        "{cmd}: You have input (l, r)=({l}, {r}) that does not meet the condition: 0 <= l < {m} and 0 <= r < {n} !!!",
                    );
                    break 'af;
                } else {
                    // Ok, do "af"
                    if let Err(str) = assignments.add_forbidden(l, r) {
                        println!("{}", str);
                    } else {
                        println!("{cmd}: added forbidden: {:?}:", (l, r));
                    }
                }
            }
            Err(err) => {
                println!("{cmd}: {}", err);
                break 'af; // stop the `af` command
            }
        }
    }
}

pub fn arf(assignments: &mut Assignments, rng: &mut impl Rng) {
    let cmd = "arf";
    let (_k, m, n) = assignments.get_kmn();
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
                    println!("{cmd}: {}", str);
                } else {
                    println!("{cmd}: added forbidden: {:?}:", (l, r));
                    count += 1;
                }
            }
            println!("{}: added {} random forbidden.", cmd, count);
        }
        Err(err) => {
            println!("{cmd}: {}", err);
            // continue 'cmd; // try again!
        }
    }
}

pub fn arfl(assignments: &mut Assignments, rng: &mut impl Rng) {
    let cmd = "arfl";
    let (_k, m, n) = assignments.get_kmn();
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
                        println!("{cmd}: {}", str);
                    } else {
                        println!("{cmd}: added forbidden: {:?}:", (l, r));
                        count += 1;
                    }
                }
                println!("{}: added {} random forbidden.", cmd, count);
            } else {
                println!("{}: Bad input: l = {} >= {} !!!", cmd, l, m);
            }
        }
        Err(err) => {
            println!("{cmd}: {}", err);
            // continue 'cmd; // try again!
        }
    }
}

pub fn arfr(assignments: &mut Assignments, rng: &mut impl Rng) {
    let cmd = "arfr";
    let (_k, m, n) = assignments.get_kmn();
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
                        println!("{cmd}: {}", str);
                    } else {
                        println!("{cmd}: added forbidden: {:?}:", (l, r));
                        count += 1;
                    }
                }
                println!("{}: added {} random forbidden.", cmd, count);
            } else {
                println!("{}: Bad input: r = {} >= {} !!!", cmd, r, n);
            }
        }
        Err(err) => {
            println!("{cmd}: {}", err);
            // continue 'cmd; // try again!
        }
    }
}

pub fn aflvrvj(assignments: &mut Assignments) {
    let cmd = "aflvrvj";
    let (_k, m, n) = assignments.get_kmn();
    'af: loop {
        println!("input one-line json: ");
        let input = read_line();
        let deserialized: Result<LeftVecRightVec, serde_json::Error> = serde_json::from_str(&input);
        match deserialized {
            Ok(deserialized) => {
                let left = &deserialized.left;
                let right = &deserialized.right;
                let pairs = all_pairs_from_left_right(left, right);
                // 'af1: for (l, r) in pairs {
                for (l, r) in pairs {
                    if !(l < m && r < n) {
                        println!(
                            "{cmd}: You have input (l, r)=({l}, {r}) that does not meet the condition: 0 <= l < {m} and 0 <= r < {n} !!!",
                        );
                        // break 'af1; // should we continue?
                    } else {
                        // Ok, do "af"
                        if let Err(err) = assignments.add_forbidden(l, r) {
                            println!("{cmd}: {err}");
                        } else {
                            println!("{cmd}: added forbidden: {:?}:", (l, r));
                        }
                    }
                }
            }
            Err(err) => {
                println!("{err}");
                break 'af;
            }
        }
    }
}

/////

/*
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
                    "af" => {
                    }
                    "arf" => {
                    }
                    "arfl" => {
                    }
                    "arfr" => {
                    }
                    "test" => {
                        println!("test_assignments ...");
                        if let Err(err) = assignments.test_assignments() {
                            println!("{cmd}: {}", err);
                        } else {
                            println!("Imlemented tests passed.");
                        }
                        println!("test_forbidden ...");
                        if let Err(err) = assignments.test_forbidden() {
                            println!("{cmd}: {}", err);
                        } else {
                            println!("Implemented tests passed.");
                        }
                    }
                    "quit" => {
                        println!("EDIT ASSIGNMENTS: THE END!");
                        println!(
                            "JSON:\n{}",
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


*/
