use crate::Assignments;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SerdeKmnAssignment {
    k: usize,
    m: usize, // len of l_permutation
    n: usize, // len of r_permutation
    assignments: Vec<(usize, usize)>,
    forbidden: Vec<(usize, usize)>,
}

impl From<&Assignments> for SerdeKmnAssignment {
    fn from(item: &Assignments) -> Self {
        Self {
            k: item.k,
            m: item.m,
            n: item.n,
            assignments: item.get_pairs_of_ids(),
            forbidden: item.forbidden.clone(),
        }
    }
}

impl From<&SerdeKmnAssignment> for Assignments {
    fn from(item: &SerdeKmnAssignment) -> Self {
        let mut out = Self::new(item.k, item.m, item.n);
        out.forbidden = item.forbidden.clone();
        out.set_pairs_of_ids(&item.assignments);
        out // returns not testeted !!!
    }
}

/*
#[derive(Serialize, Deserialize)]
Vec<(usize,usize)>
*/
