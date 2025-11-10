use crate::Assignments;
use crate::Side;
use crate::intersection_size;
use rand::Rng;
use std::error::Error;
use std::fmt::Write;
// use std::io;
use std::str::FromStr;

// Split `input` and check the `number` of fragments.
pub fn split_and_check_number(input: &String, number: usize) -> Result<Vec<&str>, Box<dyn Error>> {
    let input = input.trim();
    let args: Vec<&str> = input.split_ascii_whitespace().collect();
    if args.len() != number {
        let mut err = String::new();
        write!(
            &mut err,
            "Input line: `{}` contained {} arguments instead of {} !!!",
            input,
            args.len(),
            number
        )?;

        return Err(err.into()); // `err.into()` - because the type of err is `Box<dyn Error>`
    }

    Ok(args) // the input was Ok
}

// `parse_args` assumes that all arguments of type `T`
pub fn parse_args<T: std::str::FromStr>(args: &Vec<&str>) -> Result<Vec<T>, Box<dyn Error>>
where
    <T as FromStr>::Err: std::error::Error,
    <T as FromStr>::Err: 'static,
{
    let mut out: Vec<T> = vec![];
    for arg in args {
        out.push(arg.parse()?);
    }
    Ok(out) // happily parsed all args :-)
}

/// split by white space and then parse aguments (all of type `T`)
pub fn split_and_parse_input<T: std::str::FromStr>(
    input: &String,
    number: usize,
) -> Result<Vec<T>, Box<dyn Error>>
where
    <T as FromStr>::Err: std::error::Error,
    <T as FromStr>::Err: 'static,
{
    let args = split_and_check_number(input, number)?;
    parse_args::<T>(&args) // returns Vec<T>
}

// Steps
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Steps(pub usize);

// Forbidden
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Forbidden(pub usize);

pub trait SideSearch {
    fn action_left(assignments: &mut Assignments, rng: &mut impl Rng);

    fn action_right(assignments: &mut Assignments, rng: &mut impl Rng);

    fn search(
        assignments: &mut Assignments,
        side: Side,
        max: usize,
        rng: &mut impl Rng,
    ) -> (Steps, Steps, Forbidden) {
        /*
        let current_pairs = assignments.get_pairs_of_ids(); // before we start
        let mut f_min = intersection_size(&current_pairs, &assignments.forbidden);
        // compare and update assignments.f_min_backup to the actual f_min_backup
        match &assignments.f_min_backup {
            None => {
                assignments.f_min_backup = Some(current_pairs);
            }
            Some(pairs) => {
                let f = intersection_size(&pairs, &assignments.forbidden);
                if f < f_min {
                    f_min = f; // update f_min
                } else {
                    assignments.f_min_backup = Some(current_pairs); // update f_min_backup
                }
            }
        };
        */

        // have backup before we start
        let mut f_min = assignments.f_min_backup_update(assignments.get_pairs_of_ids());
        let mut l_steps = 0;
        let mut r_steps = 0;
        for _step in 1..=max {
            match side {
                Side::Left => {
                    Self::action_left(assignments, rng);
                    l_steps += 1;
                }
                Side::Right => {
                    Self::action_right(assignments, rng);
                    r_steps += 1;
                }
                Side::LeftPercent(l_p) => {
                    if rng.random_range(0..100) < l_p {
                        Self::action_left(assignments, rng);
                        l_steps += 1;
                    } else {
                        Self::action_right(assignments, rng);
                        r_steps += 1;
                    }
                }
            }
            let current_pairs = assignments.get_pairs_of_ids();
            let f = intersection_size(&current_pairs, &assignments.forbidden);
            // here: `f_min` is actual for current backup
            if f < f_min {
                f_min = f;
                assignments.f_min_backup = Some(current_pairs); // fearless overwrite ! ;-)
            }
            if f == 0 {
                // We have zero forbidden !!!
                return (Steps(l_steps), Steps(r_steps), Forbidden(f));
            }
        }
        // Zero forbidden not encountered
        (Steps(l_steps), Steps(r_steps), Forbidden(f_min))
    }
}

//  Permute
pub struct Permute();

impl SideSearch for Permute {
    fn action_left(assignments: &mut Assignments, rng: &mut impl Rng) {
        assignments.randomize_left(rng);
    }

    fn action_right(assignments: &mut Assignments, rng: &mut impl Rng) {
        assignments.randomize_right(rng);
    }
}

// Swap
pub struct Swap();

impl SideSearch for Swap {
    fn action_left(assignments: &mut Assignments, rng: &mut impl Rng) {
        assignments.random_swaps_of_l_forbidden(rng);
    }

    fn action_right(assignments: &mut Assignments, rng: &mut impl Rng) {
        assignments.random_swaps_of_r_forbidden(rng);
    }
}

// TODO: BackSwap();

pub struct BackSwap();

impl SideSearch for BackSwap {
    fn action_left(assignments: &mut Assignments, rng: &mut impl Rng) {
        // restore backup
        assignments.f_min_backup_restore();
        // do one step
        assignments.random_swaps_of_l_forbidden(rng);
    }

    fn action_right(assignments: &mut Assignments, rng: &mut impl Rng) {
        // restore backup
        assignments.f_min_backup_restore();
        // do one step
        assignments.random_swaps_of_r_forbidden(rng);
    }
}

// Assignments
impl Assignments {
    pub fn randomize_permutation(
        &mut self,
        side: Side,
        max: usize,
        rng: &mut impl Rng,
    ) -> (Steps, Steps, Forbidden) {
        Permute::search(self, side, max, rng)
    }

    pub fn random_swaps(
        &mut self,
        side: Side,
        max: usize,
        rng: &mut impl Rng,
    ) -> (Steps, Steps, Forbidden) {
        Swap::search(self, side, max, rng)
    }

    pub fn random_back_swaps(
        &mut self,
        side: Side,
        max: usize,
        rng: &mut impl Rng,
    ) -> (Steps, Steps, Forbidden) {
        BackSwap::search(self, side, max, rng)
    }
}
