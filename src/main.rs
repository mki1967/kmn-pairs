use kmn_pairs::*;
use std::io;
// use text_io::read;
// use rand::Rng;

fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line from input)");
    input
}

fn main() {
    println!(
        r#"
--------------------------------------------------------------------------------------------------------------------
   For integer parameters k,m,n, such that 1 <= k <= m <= n, the program:
   - finds P of size pm, where p=ceiling(kn/m), such that:
        - P is a subset of the Cartesian product {{0,...,m-1}}x{{0,...,n-1}}, and
        - for each l in {{0,...,m-1}}, P contains p pairs from {{l}}x{{0,...,n-1}}  and
        - for each r in {{0,...,n-1}}, P contains either k or k+1 pairs from {{0,...,m-1}}x{{r}}, and
   - inits two permutations p_l: {{0,...,m-1}} -> {{0,...,m-1}} and p_r: {{0,...,m-1}} -> {{0,...,m-1}} to identities.
   The triple (P, p_l, p_r) defines an 'assignment' A consisting of all the pairs (p_l(l),p_r(r)) such that (l,r) is in P.
   The assignment A can be changed by changing the permutations p_l or p_r.
   The user can define a set F of 'forbidden pairs' that should not be contained in A.
   The program displays:
       - the assignment A (marking the encountered 'forbidden pairs' with '!!!') and then
       - the set F.
   The user can:
       - display A grouped by left/right component (commands: `gl`/`gr`),
       - add new 'forbidden pair' to F (command: `af`),
       - randomize p_l/p_r  up to some max times, until A contains no 'forbidden pairs' (commands: `rl`/`rr`).
--------------------------------------------------------------------------------------------------------------------

"#
    );

    let mut rng = rand::rng(); // random number generator
    loop {
        let mut assignments: Assignments;
        loop {
            println!("input k m n (1 <= k <= m <= n): ");
            let input = read_line();
            let input = input.trim();
            let args: Vec<&str> = input.split_ascii_whitespace().collect();
            if args.len() != 3 {
                println!(
                    "Input line \"{}\" contained {} arguments instead of 3 !!!",
                    &input,
                    args.len()
                );
                continue;
            }
            let (k, m, n) = (
                args[0].parse::<usize>(),
                args[1].parse::<usize>(),
                args[2].parse::<usize>(),
            );
            if let (Ok(k), Ok(m), Ok(n)) = (k, m, n) {
                if !(1 <= k && k <= m && m <= n) {
                    println!(
                        "You have input (k,m,n)={:?}, that does not meet the condition: 1 <= k <= m <= n !!!",
                        (k, m, n)
                    );
                    continue;
                } else {
                    // Ok,  set the assignments !
                    assignments = Assignments::new(k, m, n);
                    break;
                }
            } else {
                println!(
                    "Input line contained '{}' instead of 3 positive integers !!!",
                    &input
                );
                continue;
            }
        }
        // MENU - Actions on the assigments:
        loop {
            let (_k, m, n) = assignments.get_kmn(); // get the assignments' parameters
            println!("{}", assignments);
            println!("Input command (h for help): ");
            let cmd = read_line();
            let cmd = cmd.trim();
            match cmd {
                "h" => {
                    println!(
                        "
        command action:
            gl       group by left
            gr       group by right
            rl       randomly permute left IDs
            rr       randomly permute right IDs
            af       add forbidden
            restart  restart with new parameters: k,m,n
            quit     end the program

            (press ENTER to continue)
"
                    );
                    read_line();
                }
                "rl" => {
                    println!(
                        "{}: input max (0 <= max) for max trials to find assignments without forbidden: ",
                        cmd
                    );
                    let mut max = 0; // default max value
                    let input = read_line();
                    let input = &input.trim();
                    let args: Vec<&str> = input.split_ascii_whitespace().collect();
                    if args.len() != 1 {
                        println!(
                            "Input line \"{}\" contained {} arguments instead of 1 !!!",
                            &input,
                            args.len()
                        );
                    } else {
                        let m = args[0].parse::<usize>();
                        if let Ok(m) = m {
                            max = m;
                        }
                        println!("max = {}", max);
                        for step in 1..=max {
                            assignments.randomize_left(&mut rng);
                            //println!("Randomly permuted left IDs.");
                            if assignments.number_of_forbidden_used() == 0 {
                                println!("Found zero forbidden in step {}!", step);
                                break;
                            }
                        }
                        println!("{}: done.", cmd);
                    }
                }
                "rr" => {
                    println!(
                        "{}: input max (0 <= max) for max trials to find assignments without forbidden: ",
                        cmd
                    );
                    let mut max = 0; // default max value
                    let input = read_line();
                    let input = &input.trim();
                    let args: Vec<&str> = input.split_ascii_whitespace().collect();
                    if args.len() != 1 {
                        println!(
                            "Input line \"{}\" contained {} arguments instead of 1 !!!",
                            &input,
                            args.len()
                        );
                    } else {
                        let m = args[0].parse::<usize>();
                        if let Ok(m) = m {
                            max = m;
                        }
                        println!("max = {}", max);
                        for step in 1..=max {
                            assignments.randomize_right(&mut rng);
                            //println!("Randomly permuted left IDs.");
                            if assignments.number_of_forbidden_used() == 0 {
                                println!("Found zero forbidden in step {}!", step);
                                break;
                            }
                        }
                        println!("{}: done.", cmd);
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
                "af" => {
                    loop {
                        println!(
                            "{}: input l r (0 <= l < {} and 0 <= r < {}) or something else to finish: ",
                            cmd, m, n
                        );
                        let input = read_line();
                        let input = &input.trim();
                        let args: Vec<&str> = input.split_ascii_whitespace().collect();
                        if args.len() != 2 {
                            println!(
                                "Input line \"{}\" contained {} arguments instead of 2 !!!",
                                &input,
                                args.len()
                            );
                            break;
                        }
                        let (l, r) = (args[0].parse::<usize>(), args[1].parse::<usize>());
                        if let (Ok(l), Ok(r)) = (l, r) {
                            if !(l < m && r < n) {
                                println!(
                                    "You have input (l, r)={:?}, that does not meet the condition: 0 <= l < {} and 0 <= r < {} !!!",
                                    (l, r),
                                    m,
                                    n
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
                        } else {
                            println!(
                                "Input line contained '{}' instead of 2 non-negative integers !!!",
                                &input
                            );
                            break;
                        }
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
    }
}
