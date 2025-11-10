// use kmn_pairs::cmd::*;
use kmn_pairs::menu::*;
use kmn_pairs::*;

// use std::io;
// use text_io::read;
// use rand::Rng;

fn main() {
    let mut assignments_data: Option<Assignments> = None;

    println!(
        r#"
--------------------------------------------------------------------------------------------------------------------
   For integer parameters k,m,n, such that 1 <= k <= m <= n, the program:
   - finds P of size pm, where p=ceiling(kn/m), such that:
        - P is a subset of the Cartesian product {{0,...,m-1}}x{{0,...,n-1}}, and
        - for each l in {{0,...,m-1}}, P contains p pairs from {{l}}x{{0,...,n-1}}  and
        - for each r in {{0,...,n-1}}, P contains either k or k+1 pairs from {{0,...,m-1}}x{{r}}, and
   - inits two permutations p_l: {{0,...,m-1}} -> {{0,...,m-1}} and p_r: {{0,...,n-1}} -> {{0,...,n-1}} to identities.
   The triple (P, p_l, p_r) defines an 'assignment' A consisting of all the pairs (p_l(l),p_r(r)) such that (l,r) is in P.
   The assignment A can be changed by changing:
       - the permutations p_l or p_r (the result is isomorphic), or
       - the underlying set P (the result may be not isomorphic).
   The user can define a set F of 'forbidden pairs' that should not be contained in A.
   The program delivers a set of tools that help in finding satisfying assignments.
   The user can:
       - display A and F, A, or itersection of F and A
         (commands: `p`/`pa`/`pfa`),
       - group A or F by left/right component
         (commands: `gl`/`gr`/`fgl`/`fgr`),
       - add new 'forbidden pairs' to F
         (commands: `af`/`arf`/`arfl`/`arfr`),
       - randomize p_l/p_r  up to some `max` times, until A contains no 'forbidden pairs'
         (commands: `rl`/`rr`/`rlr`/`sr`/`sl`/`slr`/`bsr`/`bslr`),
       - do cross-switching of the pairs in A (that may transform it to "not isomorphic" assignment)
         (command: `sbrk`),
       - restore the saved backup assignement (ususually the one with the minimal number of forbidden pairs)
         (command: `back`),
       - execute the tests checking integrity and discovering some conditions that disable finding assignment without forbidden pairs
         (command: `test`),
       - and view the list of available commands
         (command: `h`).
--------------------------------------------------------------------------------------------------------------------

"#
    );

    kmn_pairs_menu(&mut assignments_data);

    // TEST result
    if let Some(assignments) = assignments_data {
        println!("Data has been set to: {}", assignments);
    } else {
        println!("None assignments data has been set!");
    }
}
