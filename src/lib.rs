use serde::{Deserialize, Serialize};
pub mod cmd;
pub mod kmn_serde;
pub mod menu;
use rand::Rng;
use std::convert::From;
use std::error::Error;
use std::fmt;
use std::fmt::Write;

// div_ceil
pub fn div_ceil(left: usize, right: usize) -> usize {
    (left + right - 1) / right
}

/*
// generic test:
fn test_with<F, T>(test: F, tested: T) -> bool
where
    F: Fn(T) -> bool,
{
    test(tested)
}
*/

// Options useful for some procedures
#[derive(PartialEq)]
pub enum KmnOption {
    Force, // force execution despite some errors or warnings
}

// intersection_size
pub fn intersection_size(vec1: &Vec<(usize, usize)>, vec2: &Vec<(usize, usize)>) -> usize {
    // TODO: more efficient implementation for larger vectors
    let mut count: usize = 0;
    for p in vec1 {
        if vec2.contains(p) {
            count = count + 1;
        }
    }
    count
}

// `left_neighbors` of right `id` in `pairs`
pub fn left_neighbors(pairs: &Vec<(usize, usize)>, id: usize) -> Vec<usize> {
    let mut out = vec![];
    for (l, r) in pairs {
        if *r == id {
            out.push(*l);
        }
    }
    out.sort();
    out
}

// `right_neighbors` of left `id` in `pairs`
pub fn right_neighbors(pairs: &Vec<(usize, usize)>, id: usize) -> Vec<usize> {
    let mut out = vec![];
    for (l, r) in pairs {
        if *l == id {
            out.push(*r);
        }
    }
    out.sort();
    out
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LeftVecRightVec {
    pub left: Vec<usize>,
    pub right: Vec<usize>,
}

impl LeftVecRightVec {
    // println JSON or serde error
    pub fn println_serde(&self) {
        match serde_json::to_string(self) {
            Ok(out) => {
                println!("{out}")
            }
            Err(err) => {
                println!("{err}")
            }
        }
    }
}

pub fn all_pairs_from_left_right(left: &Vec<usize>, right: &Vec<usize>) -> Vec<(usize, usize)> {
    let mut out = vec![];
    for l in left {
        for r in right {
            out.push((*l, *r));
        }
    }
    out
}

pub fn filter_pairs_by_left_or_right(
    pairs: &Vec<(usize, usize)>,
    left: &Vec<usize>,
    right: &Vec<usize>,
) -> Vec<(usize, usize)> {
    let mut out = vec![];
    for (l, r) in pairs {
        if left.contains(l) || right.contains(r) {
            out.push((*l, *r));
        }
    }
    out
}

// Side
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
    Left,
    Right,
    LeftPercent(usize), // Any side with probability in % of being Left
}

// Left
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Left(usize);

impl From<usize> for Left {
    fn from(item: usize) -> Self {
        Self(item)
    }
}

impl fmt::Display for Left {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let &Left(i) = &self;
        write!(f, "L_{}", &i)
    }
}

// Right
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Right(usize);

impl From<usize> for Right {
    fn from(item: usize) -> Self {
        Self(item)
    }
}

impl fmt::Display for Right {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let &Right(i) = &self;
        write!(f, "R_{}", &i)
    }
}

// Pair
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pair(Left, Right);

impl From<&(usize, usize)> for Pair {
    fn from(item: &(usize, usize)) -> Self {
        let &(l, r) = &item;
        Self(Left(*l), Right(*r))
    }
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let &Pair(l, r) = &self;
        write!(f, "({}, {})", &l, &r)
    }
}

impl Pair {
    pub fn left(&self) -> usize {
        let Pair(Left(l), _) = self;
        *l
    }

    pub fn right(&self) -> usize {
        let Pair(_, Right(r)) = self;
        *r
    }
}

// Pairs
#[derive(Debug)]
pub struct Pairs(Vec<Pair>);

impl From<&Vec<(usize, usize)>> for Pairs {
    fn from(item: &Vec<(usize, usize)>) -> Self {
        let mut v: Vec<Pair> = vec![];
        for p in item {
            v.push(Pair::from(p));
        }
        Self(v)
    }
}

impl fmt::Display for Pairs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n[\n")?;
        let &Pairs(v) = &self;
        for idx in 0..v.len() {
            write!(f, "    {}\n", &v[idx])?;
        }
        write!(f, "]\n")
    }
}

impl Pairs {
    pub fn kmnp_pairs(
        k: Option<usize>,
        m: usize,
        n: usize,
        p: Option<usize>,
    ) -> Result<Pairs, &'static str> {
        match (k, p) {
            (None, None) => {
                return Err("kmnp_pairs: (k, p) = (None, None) !!!");
            }
            (Some(k), None) => {
                if 1 <= k && k <= m && m <= n {
                    return Ok(Self::kmn_pairs(k, m, n));
                } else {
                    return Err("kmn_pairs: 1 <= k <= m <= n is not satisfied !!! ");
                }
            }
            (None, Some(p)) => {
                if 1 <= p && p <= n && n <= m {
                    let k = (p * m) / n; // should be p = div_ceil( k * n / m ) and 1 <= k
                    return Ok(Self::kmn_pairs(k, m, n));
                } else {
                    return Err("kmn_pairs: 1 <= p <= n <= m is not satisfied !!! ");
                }
            }
            _ => {
                return Err("kmnp_pairs: Not ipmlemented yet !!!");
            }
        }
    }

    pub fn kmn_pairs(k: usize, m: usize, n: usize) -> Pairs {
        // check the parameters: 1 <= k <= m <= n
        // if !(1 <= k && k <= m && m <= n) // it was before
        if !(1 <= k && k <= m) {
            panic!(
                "kmn_pairs( {}, {}, {} ) should be 1 <= k <= m <= n or 1 <= p <= n <= m !!!",
                k, m, n
            );
        }

        let p = div_ceil(k * n, m); // p = ceil( k*n / m );
        if !(1 <= p && p <= n) {
            panic!(
                "kmn_pairs( {}, {}, {} ) should be 1 <= p <= n for p = ceil( k*n / m ) !!!",
                k, m, n
            );
        }

        let mut out = vec![Pair(Left(m * n), Right(m * n)); p * m];
        let mut offset = n - 1; // -1  modulo n
        for step in 0..p * m {
            if step % m == 0 && step % n == 0 {
                offset = (offset + 1) % n;
            }
            out[step] = Pair(Left(step % m), Right((step + offset) % n))
        }

        Pairs(out)

        // TODO: test the  function
    }

    pub fn len(&self) -> usize {
        let Pairs(p) = self;
        p.len()
    }

    pub fn pair(&self, i: usize) -> &Pair {
        let Pairs(p) = self;
        &p[i]
    }

    pub fn with(&self, left: Left) -> usize {
        let &Pairs(v) = &self;
        let mut sum = 0;
        for idx in 0..v.len() {
            let Pair(l, _) = &v[idx];
            if *l == left {
                sum = sum + 1;
            }
        }
        sum
    }

    pub fn sort_by_left(&mut self) {
        let Pairs(v) = self;
        v.sort_by(|a, b| {
            let Pair(Left(la), Right(ra)) = a;
            let Pair(Left(lb), Right(rb)) = b;
            (la, ra).cmp(&(lb, rb))
        });
    }

    pub fn sort_by_right(&mut self) {
        let Pairs(v) = self;
        v.sort_by(|a, b| {
            let Pair(Left(la), Right(ra)) = a;
            let Pair(Left(lb), Right(rb)) = b;
            (ra, la).cmp(&(rb, lb))
        });
    }
}

// Permutation
#[derive(Debug)]
pub struct Permutation(Vec<usize>);

impl Permutation {
    // returns identity permutation of size `len`
    pub fn new(len: usize) -> Self {
        let mut out = vec![0; len];
        for i in 0..len {
            out[i] = i;
        }
        Permutation(out) // return
    }

    pub fn value(&self, i: usize) -> usize {
        let Permutation(p) = self;
        p[i]
    }

    pub fn swap(&mut self, i: usize, j: usize) {
        let Permutation(p) = self;
        let n = p.len();
        if i >= n || j >= n {
            return; // ignore - consider some panic! instead
        }
        p.swap(i, j);
    }

    pub fn randomize(&mut self, rng: &mut impl Rng) {
        let Permutation(p) = self;
        let n = p.len();
        for i in (1..n).rev() {
            let r: usize = rng.random_range(0..i);
            p.swap(r, n - i);
        }
    }
}

// Assignments
#[derive(Debug)]
pub struct Assignments {
    k: usize,
    m: usize, // len of l_permutation
    n: usize, // len of r_permutation
    pairs: Pairs,
    l_permutation: Permutation,
    r_permutation: Permutation,
    forbidden: Vec<(usize, usize)>,
    f_min_backup: Option<Vec<(usize, usize)>>, // may be not valid if `forbidden` change !!!
}

impl fmt::Display for Assignments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.assignments_header())?;
        write!(f, "{}", self.assignments_body())?;
        write!(f, "{}", self.forbidden_header())?;
        write!(f, "{}", self.forbidden_body())?;
        write!(f, "")
    }
}

impl Assignments {
    pub fn new(k: usize, m: usize, n: usize) -> Self {
        let pairs = Pairs::kmn_pairs(k, m, n);
        let l_permutation = Permutation::new(m);
        let r_permutation = Permutation::new(n);
        let forbidden = Vec::new();
        let f_min_backup = Option::None;
        Self {
            k,
            m,
            n,
            pairs,
            l_permutation,
            r_permutation,
            forbidden,
            f_min_backup,
        }
    }

    pub fn new_kmnp(k: Option<usize>, m: usize, n: usize, p: Option<usize>) -> Self {
        // check k and p parameters
        if k == None && p == None {
            panic!("Assignments::new_kmnp: must not be k = p = None !!!")
        }
        let pairs = Pairs::kmnp_pairs(k, m, n, p);
        if let Err(err) = pairs {
            panic!("Assignments::new_kmnp: {err}");
        }
        let l_permutation = Permutation::new(m);
        let r_permutation = Permutation::new(n);
        let forbidden = Vec::new();
        let f_min_backup = Option::None;
        // either we have `k` or we have `p` and compute k (`or_else` - for laziness)
        let k = k.unwrap_or_else(|| (p.unwrap() * m) / n);
        Self {
            k,
            m,
            n,
            pairs: pairs.expect("Assignments::new_kmnp: pairs shoult be OK here !!!"),
            l_permutation,
            r_permutation,
            forbidden,
            f_min_backup,
        }
    }

    // sorts and tests left_ids and creates remapping of the right ids to {0, ..., left_ids.len()-1}
    pub fn sort_and_test_subset_of_left_ids(
        &self,
        left_ids: &mut Vec<usize>,
    ) -> Result<Vec<Option<usize>>, Box<dyn Error>> {
        let mut err = String::new();
        left_ids.sort();
        let m = self.m;
        let mut prev = None;
        for i in 0..left_ids.len() {
            let l = left_ids[i];
            if l >= m {
                writeln!(&mut err, "In left_ids: {l} >= m = {m} !!!",)?;
            }
            if Some(l) == prev {
                writeln!(&mut err, "In left_ids: duplicate of {l} !!!",)?;
            }
            prev = Some(l);
        }

        if err.len() > 0 {
            Err(err.into())
        } else {
            let mut map: Vec<Option<usize>> = vec![None; m];
            for i in 0..left_ids.len() {
                map[left_ids[i]] = Some(i);
            }
            Ok(map)
        }
    }

    // try to create Assignments with left ids reduced to sorted left_ids with forbidden left sides remapped to their new ids
    pub fn left_reduced_to(
        &self,
        left_ids: &Vec<usize>,
        k: Option<usize>,
        p: Option<usize>,
    ) -> Result<Self, Box<dyn Error>> {
        let mut left_ids = left_ids.clone();
        // test left_ids
        let test_result = self.sort_and_test_subset_of_left_ids(&mut left_ids);
        let map: Vec<Option<usize>>;
        match test_result {
            Err(err) => {
                return Err(err.into());
            }
            Ok(map1) => {
                map = map1;
            }
        }
        let (m, n) = (left_ids.len(), self.n);
        // test (k,m,n,p)
        if let Err(err) = Pairs::kmnp_pairs(k, m, n, p) {
            return Err(err.into());
        }
        let mut assignments = Assignments::new_kmnp(k, m, n, p);
        for (l, r) in &self.forbidden {
            if let Some(l1) = map[*l] {
                assignments.forbidden.push((l1, *r));
            }
        }
        Ok(assignments)
    }

    // sorts and tests right_ids and creates remapping of the right ids to {0, ..., right_ids.len()-1}
    pub fn sort_and_test_subset_of_right_ids(
        &self,
        right_ids: &mut Vec<usize>,
    ) -> Result<Vec<Option<usize>>, Box<dyn Error>> {
        let mut err = String::new();
        right_ids.sort();
        let n = self.n;
        let mut prev = None;
        for i in 0..right_ids.len() {
            let r = right_ids[i];
            if r >= n {
                writeln!(&mut err, "In right_ids: {r} >= n = {n} !!!",)?;
            }
            if Some(r) == prev {
                writeln!(&mut err, "In right_ids: duplicate of {r} !!!",)?;
            }
            prev = Some(r);
        }

        if err.len() > 0 {
            Err(err.into())
        } else {
            let mut map: Vec<Option<usize>> = vec![None; n];
            for i in 0..right_ids.len() {
                map[right_ids[i]] = Some(i);
            }
            Ok(map)
        }
    }

    // try to create Assignments with right ids reduced to sorted right_ids with forbidden right sides remapped to their new ids
    pub fn right_reduced_to(
        &self,
        right_ids: &Vec<usize>,
        k: Option<usize>,
        p: Option<usize>,
    ) -> Result<Self, Box<dyn Error>> {
        let mut right_ids = right_ids.clone();
        // test right_ids
        let test_result = self.sort_and_test_subset_of_right_ids(&mut right_ids);
        let map: Vec<Option<usize>>;
        match test_result {
            Err(err) => {
                return Err(err.into());
            }
            Ok(map1) => {
                map = map1;
            }
        }
        let (m, n) = (self.m, right_ids.len());
        // test (k,m,n,p)
        if let Err(err) = Pairs::kmnp_pairs(k, m, n, p) {
            return Err(err.into());
        }
        let mut assignments = Assignments::new_kmnp(k, m, n, p);
        for (l, r) in &self.forbidden {
            if let Some(r1) = map[*r] {
                assignments.forbidden.push((*l, r1));
            }
        }
        Ok(assignments)
    }

    // get tuple with parameters: (k,m,n)
    pub fn get_kmn(&self) -> (usize, usize, usize) {
        (self.k, self.m, self.n)
    }

    pub fn p(&self) -> usize {
        let (k, m, n) = self.get_kmn();
        div_ceil(k * n, m) // ceil( k*n/m )
    }

    pub fn forbidden(&self) -> &Vec<(usize, usize)> {
        &self.forbidden
    }

    pub fn assignments_header(&self) -> String {
        format!(
            "Assignments (k,m,n) = {:?} [p = {}]",
            self.get_kmn(),
            self.p()
        )
    }

    pub fn assignments_body(&self) -> String {
        let mut out = String::from("  [\n");
        for (l, r) in self.get_pairs_of_ids() {
            let mut warn = "";
            if self.forbidden.contains(&(l, r)) {
                warn = " !!!";
            }
            out = format!("{}    {} {}{}\n", out, l, r, warn);
        }
        out = format!("{}  ]\n", out);
        out
    }

    pub fn assignments_in_forbidden(&self) -> String {
        let mut out = String::from("Forbidden in assignments:\n  [\n");
        for (l, r) in self.get_pairs_of_ids() {
            if self.forbidden.contains(&(l, r)) {
                out = format!("{}    {} {}\n", out, l, r);
            }
        }
        out = format!("{}  ]\n", out);
        out
    }

    pub fn forbidden_header(&self) -> String {
        let count = intersection_size(&self.get_pairs_of_ids(), &self.forbidden);
        let warn = if count > 0 { " !!!" } else { "" };
        format!("Forbidden ({} / {}){}", &self.forbidden.len(), count, warn)
    }

    pub fn forbidden_body(&self) -> String {
        let mut out = String::from("  [\n");
        let assignment_pairs = self.get_pairs_of_ids();
        for (l, r) in self.forbidden.clone() {
            let mut warn = "";
            if assignment_pairs.contains(&(l, r)) {
                warn = " !!!";
            }
            out = format!("{}    {} {}{}\n", out, l, r, warn);
        }
        out = format!("{}  ]\n", out);
        out
    }

    pub fn backup_header(&self) -> String {
        if let Some(backup) = &self.f_min_backup {
            format!(
                "{}-forbidden backup",
                intersection_size(&backup, &self.forbidden)
            )
        } else {
            format!("No backup")
        }
    }

    // TODO: Use if needed:
    // fn f_min_backup_overwrite(&mut self, pairs: Vec<(usize,usize)>) {
    //    self.f_min_backup = Some( pairs );
    //

    pub fn f_min_backup_restore(&mut self) {
        let backup = self.f_min_backup.clone();
        if let Some(pairs) = backup {
            self.set_pairs_of_ids(&pairs);
        }
    }

    // `f_min_backup_update(&mut self, pairs: Vec<(usize.usize))` updates and returns actual `f_min`
    pub fn f_min_backup_update(&mut self, pairs: Vec<(usize, usize)>) -> usize {
        let f_min = intersection_size(&pairs, &self.forbidden);
        // compare and update assignments.f_min_backup to the actual f_min_backup
        if self.f_min_backup == None {
            self.f_min_backup = Some(pairs);
            return f_min; // `pairs` is the new backup
        } else if let Some(backup) = &self.f_min_backup {
            let f = intersection_size(&backup, &self.forbidden);
            if f < f_min {
                return f; // old backup remains: `f` is better than `f_min`
            }
        }
        // here `pairs` is better than old backup
        self.f_min_backup = Some(pairs);
        f_min // `f_min` of the new backup
    }

    // get pair of assigned IDs for a pair from self.pairs
    pub fn get_pair_of_ids(&self, pair: &Pair) -> (usize, usize) {
        (
            self.l_permutation.value(pair.left()),
            self.r_permutation.value(pair.right()),
        ) // returns this pair
    }

    // get vector of assigned pairs of IDs
    pub fn get_pairs_of_ids(&self) -> Vec<(usize, usize)> {
        let mut v: Vec<(usize, usize)> = vec![];
        let Pairs(pairs) = &self.pairs;
        for pair in pairs {
            v.push(self.get_pair_of_ids(pair));
        }
        v
    }

    // `set_pairs_of_ids` - sets new `pairs` - use only for correct `pairs` !!!
    pub fn set_pairs_of_ids(&mut self, pairs: &Vec<(usize, usize)>) {
        let (_k, m, n) = self.get_kmn();
        self.pairs = Pairs::from(pairs);
        self.l_permutation = Permutation::new(m);
        self.r_permutation = Permutation::new(n);
        // TODO: use `test_assignments`
    }

    pub fn number_of_forbidden_used(&self) -> usize {
        intersection_size(&self.get_pairs_of_ids(), &self.forbidden)
    }

    pub fn group_by_left(&mut self) {
        self.pairs.sort_by_left();
    }

    pub fn group_by_right(&mut self) {
        self.pairs.sort_by_right();
    }

    // add_forbidden
    pub fn add_forbidden(&mut self, l: usize, r: usize) -> Result<(), Box<dyn Error>> {
        let mut err = String::new();
        if l >= self.m {
            writeln!(
                &mut err,
                "adding forbidden ({l},{r}) with {l} >= m = {}, where the left set is {{0,...,m-1}}",
                self.m
            )?;
            Err(err.into())
        } else if r >= self.n {
            writeln!(
                &mut err,
                "adding forbidden ({l},{r}) with {r} >= n = {}, where the right set is {{0,...,n-1}}",
                self.n
            )?;
            Err(err.into())
        } else if self.forbidden.contains(&(l, r)) {
            writeln!(
                &mut err,
                "adding forbidden ({l},{r}) that is already in forbidden"
            )?;
            Err(err.into())
        } else {
            self.forbidden.push((l, r));
            self.forbidden.sort();
            Ok(())
        }
    }

    pub fn extract_forbidden_by<T: Fn((usize, usize)) -> bool>(
        &mut self,
        test: T,
    ) -> Vec<(usize, usize)> {
        let mut i = 0;
        let mut out = vec![];
        while i < self.forbidden.len() {
            if test(self.forbidden[i]) {
                out.push(self.forbidden.swap_remove(i));
            } else {
                i += 1;
            }
        }
        out
    }

    // `assigned_to_left`, for `l_id`, returns sorted vector of right IDs assigned to `l_id`
    pub fn assigned_to_left(&self, l_id: usize) -> Result<Vec<usize>, Box<dyn Error>> {
        let mut err = String::new();
        if l_id >= self.m {
            writeln!(
                &mut err,
                "assigned_to_left({l_id}) with  {l_id} >= m = {}, where the left set is {{0,...,m-1}}",
                self.m
            )?;
            Err(err.into())
        } else {
            let mut out = vec![];
            for (l, r) in self.get_pairs_of_ids() {
                if l == l_id {
                    out.push(r);
                }
            }
            out.sort();
            Ok(out) // sorted but not tested
        }
    }

    // `assigned_to_right`, for `r_id`, returns sorted vector of left IDs assigned to `r_id`
    pub fn assigned_to_right(&self, r_id: usize) -> Result<Vec<usize>, Box<dyn Error>> {
        let mut err = String::new();
        if r_id >= self.n {
            writeln!(
                &mut err,
                "assigned_to_right({r_id}) with  {r_id} >= n = {}, where the right set is {{0,...,n-1}}",
                self.n
            )?;
            Err(err.into())
        } else {
            let mut out = vec![];
            for (l, r) in self.get_pairs_of_ids() {
                if r == r_id {
                    out.push(l);
                }
            }
            out.sort();
            Ok(out) // sorted but not tested
        }
    }

    // TESTS

    pub fn test_left_ids(&self, ids: &Vec<usize>) -> Result<(), Box<dyn Error>> {
        let (_k, m, _n) = self.get_kmn();
        let mut err = String::new();
        for id in ids {
            if *id > m {
                writeln!(&mut err, "Found left id {} >= m = {} !!!", id, m)?;
            }
        }
        if err.len() > 0 {
            Err(err.into())
        } else {
            Ok(())
        }
    }

    pub fn test_right_ids(&self, ids: &Vec<usize>) -> Result<(), Box<dyn Error>> {
        let (_k, _m, n) = self.get_kmn();
        let mut err = String::new();
        for id in ids {
            if *id > n {
                writeln!(&mut err, "Found right id {} >= n = {} !!!", id, n)?;
            }
        }
        if err.len() > 0 {
            Err(err.into())
        } else {
            Ok(())
        }
    }

    // test correctness of a set assigned to left ID
    pub fn test_assigned_to_left(&self, id: usize) -> Result<(), Box<dyn Error>> {
        let (k, m, n) = self.get_kmn();
        let p = div_ceil(k * n, m); // p = ceil( k*n / m );
        let neighbors = self.assigned_to_left(id)?;
        let mut err = String::new();
        let nb = neighbors.len();
        if nb != p {
            writeln!(
                &mut err,
                "Numer of assigned_to_left({}) is: {} != ceil( k*n / m ) = {} !!!",
                id, nb, p,
            )?;
        }
        if let Err(e) = self.test_right_ids(&neighbors) {
            writeln!(&mut err, "{}", e.to_string())?;
        }
        let mut prev = Option::None;
        for r in neighbors {
            if Some(r) == prev {
                writeln!(
                    &mut err,
                    "In assigned_to_left({}): duplicate of {} !!!",
                    id, r
                )?;
            }
            prev = Some(r);
        }
        if err.len() > 0 {
            Err(err.into())
        } else {
            Ok(())
        }
    }

    // test correctness of a set assigned to right ID
    pub fn test_assigned_to_right(&self, id: usize) -> Result<(), Box<dyn Error>> {
        let (k, _m, _n) = self.get_kmn();
        // let p = div_ceil(k * n, m); // p = ceil( k*n / m );
        let neighbors = self.assigned_to_right(id)?;
        let mut err = String::new();
        let nb = neighbors.len();
        if !(k..=k + 1).contains(&nb) {
            writeln!(
                &mut err,
                "Numer of assigned_to_right({}) is: {} not in {{k, k+1}}={{{}, {}}} !!!",
                id,
                nb,
                k,
                k + 1
            )?;
        }
        if let Err(e) = self.test_left_ids(&neighbors) {
            writeln!(&mut err, "{}", e.to_string())?;
        }
        let mut prev = Option::None;
        for r in neighbors {
            if Some(r) == prev {
                writeln!(
                    &mut err,
                    "In `assigned_to_right({})`: duplicate of {} !!!",
                    id, r
                )?;
            }
            prev = Some(r);
        }
        if err.len() > 0 {
            Err(err.into())
        } else {
            Ok(())
        }
    }

    pub fn test_assignment_pairs(&self) -> Result<(), Box<dyn Error>> {
        let (k, m, n) = self.get_kmn();
        let p = div_ceil(k * n, m); // p = ceil( k*n / m );
        let mut err = String::new();
        // previous version: if !(1 <= k && k <= m && m <= n) {
        if !(1 <= k && 1 <= p && k <= m && p <= n) {
            writeln!(
                &mut err,
                // previous version: "(k,m,n)={:?}, that does not meet the condition: 1 <= k <= m <= n !!!",
                "test_assignment_pairs: (k, m, n, p)={:?}, that does not meet the condition: 1 <= k && 1<=p && k <= m && p <= n !!!",
                (k, m, n, p)
            )?;
        }
        // test the number of pairs
        let pairs = self.get_pairs_of_ids();
        if pairs.len() != p * m {
            writeln!(
                &mut err,
                "Number of assigment pairs is {} != ceil(k * n / m)*m={}",
                pairs.len(),
                p * m
            )?;
        }

        // test degrees and bad ids on both sides
        let mut l_deg = vec![0; m];
        let mut r_deg = vec![0; n];
        {
            let mut bad_pairs = 0;
            for (l, r) in pairs {
                if l < m && r < n {
                    l_deg[l] += 1;
                    r_deg[r] += 1;
                } else {
                    bad_pairs += 1;
                    writeln!(&mut err, "Bad pair ({}, {}) !!! (m={}, n={})", l, r, m, n)?;
                }
            }
            if bad_pairs != 0 {
                writeln!(
                    &mut err,
                    "{} bad pairs ignored in l_deg/r_deg tests !!!",
                    bad_pairs
                )?;
            }
        }
        {
            let mut bad_degs = 0;
            for i in 0..l_deg.len() {
                if l_deg[i] != p {
                    bad_degs += 1;
                    writeln!(
                        &mut err,
                        "left_deg({}) = {} != ceil(k * n / m) = {} !!!",
                        i, l_deg[i], p
                    )?;
                }
            }
            if bad_degs != 0 {
                writeln!(&mut err, "{} bad l_degs found !!!", bad_degs)?;
            }
        }
        {
            let mut bad_degs = 0;
            let mut degs_k_plus_1 = 0;
            for i in 0..r_deg.len() {
                if !(k..=k + 1).contains(&r_deg[i]) {
                    bad_degs += 1;
                    writeln!(
                        &mut err,
                        "right_deg({}) = {} not in {{k, k+1}}={{{},{}}} !!!",
                        i,
                        r_deg[i],
                        k,
                        k + 1
                    )?;
                } else {
                    if r_deg[i] == k + 1 {
                        degs_k_plus_1 += 1;
                    }
                }
            }
            if bad_degs != 0 {
                writeln!(&mut err, "{} bad r_degs found !!!", bad_degs)?;
            }
            if degs_k_plus_1 != (m * p) - (k * n) {
                writeln!(
                    &mut err,
                    "Number of right ids with degree k+1 = {} is {} != m*p - (k*n) = {} !!!",
                    k + 1,
                    degs_k_plus_1,
                    (m * p) - (k * n)
                )?;
            }
        }
        {
            let l_deg_sum: usize = l_deg.iter().sum();
            if l_deg_sum != p * m {
                writeln!(
                    &mut err,
                    "left_degree sum is {} != ceil(k * n / m)*m={}",
                    l_deg_sum,
                    p * m
                )?;
            }
        }
        {
            let r_deg_sum: usize = r_deg.iter().sum();
            if r_deg_sum != p * m {
                writeln!(
                    &mut err,
                    "right_degree sum is {} != ceil(k * n / m)*m={}",
                    r_deg_sum,
                    p * m
                )?;
            }
        }

        // ...

        if err.len() > 0 {
            Err(err.into())
        } else {
            Ok(())
        }
    }

    pub fn test_assignments(&self) -> Result<(), Box<dyn Error>> {
        self.test_assignment_pairs()?;
        let (_k, m, n) = self.get_kmn();
        for id in 0..m {
            self.test_assigned_to_left(id)?;
        }
        for id in 0..n {
            self.test_assigned_to_right(id)?;
        }
        // tests passed!
        Ok(())
    }

    // TESTS of forbidden

    // `test_forbidden` tests correctnes and some efects of fobidden.
    // As side effect it sorts `forbidden` to check whether they are all different.
    pub fn test_forbidden(&mut self) -> Result<(), Box<dyn Error>> {
        let (k, m, n) = self.get_kmn();
        let p = div_ceil(k * n, m); // p = ceil( k*n / m );
        let mut err = String::new();

        if self.forbidden.len() > (m * n) - (m * p) {
            writeln!(
                &mut err,
                "#WARNING# In `forbidden`: forbidden.len() = {} >= (m*n)-(m*p) = {} !!!",
                self.forbidden.len(),
                (m * n) - (m * p)
            )?;
        }

        self.forbidden.sort(); // As side effect it sorts `forbidden`

        // test degrees and bad ids on both sides and uniqueness of pairs
        let mut l_deg = vec![0; m];
        let mut r_deg = vec![0; n];
        let mut prev: Option<(usize, usize)> = Option::None; // to test uniqueness in sorted
        {
            let mut bad_pairs = 0;
            for (l, r) in &self.forbidden {
                let (l, r) = (*l, *r); //
                if l < m && r < n {
                    l_deg[l] += 1;
                    r_deg[r] += 1;
                    if prev == Some((l, r)) {
                        writeln!(&mut err, "In `forbidden`: duplicate of ({}, {}) !!!", l, r)?;
                    }
                    prev = Some((l, r)); // update prev
                } else {
                    bad_pairs += 1;
                    writeln!(
                        &mut err,
                        "In `forbidden`: bad pair ({}, {}) !!! (m={}, n={})",
                        l, r, m, n
                    )?;
                }
            }
            if bad_pairs != 0 {
                writeln!(
                    &mut err,
                    "In `forbidden`: {} bad pairs ignored in l_deg/r_deg tests !!!",
                    bad_pairs
                )?;
            }
        }

        // TODO: some tests for evident infeasibility of assignments without forbidden ...
        {
            let mut bad_degs = 0;
            for i in 0..l_deg.len() {
                if n - l_deg[i] < p {
                    bad_degs += 1;
                    writeln!(
                        &mut err,
                        "#WARNING# In `forbidden`: n-l_deg({}) = {} < p = {} !!!",
                        i,
                        n - l_deg[i],
                        p
                    )?;
                }
            }
            if bad_degs != 0 {
                writeln!(
                    &mut err,
                    "#WARNING# In `forbidden`: {} bad l_degs found !!!",
                    bad_degs
                )?;
            }
        }
        {
            let mut bad_degs = 0;
            let mut degs_above_m_minus_k_minus_1 = 0;
            for i in 0..r_deg.len() {
                if m - r_deg[i] < k {
                    bad_degs += 1;
                    writeln!(
                        &mut err,
                        "#WARNING# In `forbidden`: m - r_deg({}) = {} < k = {} !!!",
                        i,
                        m - r_deg[i],
                        k
                    )?;
                }
                if m - r_deg[i] < k + 1 {
                    degs_above_m_minus_k_minus_1 += 1;
                }
            }
            if bad_degs != 0 {
                writeln!(
                    &mut err,
                    "#WARNING# In `forbidden`: {} bad r_degs found !!!",
                    bad_degs
                )?;
            }
            if degs_above_m_minus_k_minus_1 > n - ((m * p) - (k * n)) {
                writeln!(
                    &mut err,
                    "#WARNING# In `forbidden`: Number of right ids i with  m - r_deg[i] < k+1 = {} is {} > n - ((m*p) - (k*n)) = {} !!!",
                    k + 1,
                    degs_above_m_minus_k_minus_1,
                    n - ((m * p) - (k * n))
                )?;
            }
        }

        if err.len() > 0 {
            Err(err.into())
        } else {
            Ok(())
        }
    }

    // randomizes l_permutation
    pub fn randomize_left(&mut self, rng: &mut impl Rng) {
        self.l_permutation.randomize(rng);
    }

    // randomizes r_permutation
    pub fn randomize_right(&mut self, rng: &mut impl Rng) {
        self.r_permutation.randomize(rng);
    }

    pub fn random_swaps_of_r_forbidden(&mut self, rng: &mut impl Rng) {
        let n = self.n; // for right side
        let Pairs(pairs) = &self.pairs;
        for pair in pairs {
            let (l, r) = self.get_pair_of_ids(pair); // can notice new frobidden just introduced
            if self.forbidden.contains(&(l, r)) {
                self.r_permutation.swap(r, rng.random_range(0..n));
            }
        }
    }

    pub fn random_swaps_of_l_forbidden(&mut self, rng: &mut impl Rng) {
        let m = self.m; // for left side
        let Pairs(pairs) = &self.pairs;
        for pair in pairs {
            let (l, r) = self.get_pair_of_ids(pair); // can notice new frobidden just introduced
            if self.forbidden.contains(&(l, r)) {
                self.l_permutation.swap(l, rng.random_range(0..m));
            }
        }
    }

    // `try_switching_endpoints`:
    // `pairs` must be legal assignment,
    // returns the number of remaining forbidden in pairs
    // transformed `pairs` may be not isomorfic with input `pairs`
    pub fn try_switching_endpoints(
        &mut self,
        mut pairs: Vec<(usize, usize)>,
    ) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
        let (k, m, n) = self.get_kmn();
        let p = self.p();

        let mut err = String::from("");

        if pairs.len() != m * p {
            writeln!(
                &mut err,
                "#WARNING# In `try_switching_endpoints`: pairs.len() = {} != m*p = {}!!!",
                pairs.len(),
                m * p
            )?;
            return Err(err.into()); // nothing to do
        }
        // init data structures
        let mut nbrs_of_l: Vec<Vec<usize>> = vec![vec![]; m]; // neighbors of m left endpoints
        let mut nbrs_of_r: Vec<Vec<usize>> = vec![vec![]; n]; // neighbors of n right endpoints
        let mut pairs_forbidden: Vec<(usize, usize)> = vec![];
        let mut pairs_not_forbidden: Vec<(usize, usize)> = vec![];

        while let Some((l, r)) = pairs.pop() {
            if l >= m || r > n {
                writeln!(
                    &mut err,
                    "#WARNING# In `try_switching_endpoints`: pairs contained ({},{}) not in {{0,...,m-1}} x {{0,...,m-1}}",
                    l, r
                )?;
                return Err(err.into()); // nothing to do
            }
            nbrs_of_l[l].push(r);
            nbrs_of_r[r].push(l);
            if self.forbidden.contains(&(l, r)) {
                pairs_forbidden.push((l, r));
            } else {
                pairs_not_forbidden.push((l, r));
            }
        }
        // check sizes of nbrs
        for l in 0..nbrs_of_l.len() {
            if nbrs_of_l[l].len() != p {
                writeln!(
                    &mut err,
                    "#WARNING# In `try_switching_endpoints`: nbrs_of_l[{}].len() = {} != p = {} !!!",
                    l,
                    nbrs_of_l[l].len(),
                    p
                )?;
                return Err(err.into()); // nothing to do
            }
        }
        for r in 0..nbrs_of_r.len() {
            if !(k..=k + 1).contains(&nbrs_of_r[r].len()) {
                writeln!(
                    &mut err,
                    "#WARNING# In `try_switching_endpoints`: nbrs_of_r[{}].len()={} not in {{k, k+1}}, for k = {} !!!",
                    r,
                    nbrs_of_r[r].len(),
                    k
                )?;
                return Err(err.into()); // nothing to do
            }
        }

        // TODO Implement switching
        let mut failure = false;
        '_l0: while !failure && pairs_forbidden.len() > 0 {
            // try to reduce some forbiden in each iteration
            let len_before = pairs_forbidden.len(); // note before the this iteration
            let mut not_reduced = vec![]; // the pairs not reduced after this iteration
            // take each pair (l1,r1) from `pairs_forbidden` and try reduce with it
            'l1: while let Some((l1, r1)) = pairs_forbidden.pop() {
                let mut tmp: Vec<(usize, usize)> = vec![];
                let mut was_reduction = vec![false, false];
                // println!("BEFORE 'l2: (l1, r1) = ({}, {}), pf={}, pnf={}", l1, r1, pairs_forbidden.len(), pairs_not_forbidden.len() );
                // test if you can reduce by crossing between pairs_forbidden
                'l2: while let Some((l2, r2)) = pairs_forbidden.pop() {
                    // try whether (l1,r2) (l2,r1) can replace (l1,r1) and (l2,r2)
                    if l1 != l2
                        && r1 != r2
                        && !nbrs_of_l[l1].contains(&r2)
                        && !nbrs_of_l[l2].contains(&r1)
                        && !nbrs_of_r[r1].contains(&l2)
                        && !nbrs_of_r[r2].contains(&l1)
                    {
                        // can be cross-switched
                        was_reduction[0] = !self.forbidden.contains(&(l1, r2));
                        was_reduction[1] = !self.forbidden.contains(&(l2, r1));
                        if was_reduction[0] || was_reduction[1] {
                            // anyway, do cross-switching
                            // update neighors of left sides
                            if let Some(idx) = nbrs_of_l[l1].iter().position(|x| *x == r1) {
                                nbrs_of_l[l1][idx] = r2;
                            } else {
                                panic!(
                                    "l1-r `try_switching_endpoints`: Bad nbrs in cross-switching ({}, {}) with ({}, {}) !!!",
                                    l1, r1, l2, r2
                                )
                            }
                            if let Some(idx) = nbrs_of_l[l2].iter().position(|x| *x == r2) {
                                nbrs_of_l[l2][idx] = r1;
                            } else {
                                panic!(
                                    "l2-r `try_switching_endpoints`: Bad nbrs in cross-switching ({}, {}) with ({}, {}) !!!",
                                    l1, r1, l2, r2
                                )
                            }
                            // update neighors of right sides
                            if let Some(idx) = nbrs_of_r[r1].iter().position(|x| *x == l1) {
                                nbrs_of_r[r1][idx] = l2;
                            } else {
                                panic!(
                                    "l-r1 `try_switching_endpoints`: Bad nbrs in cross-switching ({}, {}) with ({}, {}) !!!",
                                    l1, r1, l2, r2
                                )
                            }
                            if let Some(idx) = nbrs_of_r[r2].iter().position(|x| *x == l2) {
                                nbrs_of_r[r2][idx] = l1;
                            } else {
                                panic!(
                                    "l-r2 `try_switching_endpoints`: Bad nbrs in cross-switching ({}, {}) with ({}, {}) !!!",
                                    l1, r1, l2, r2
                                )
                            }
                            // push to `pairs_not_forbidden` the reduced and to `tmp` the still forbidden
                            if was_reduction[0] {
                                pairs_not_forbidden.push((l1, r2));
                            } else {
                                tmp.push((l1, r2));
                            }
                            if was_reduction[1] {
                                pairs_not_forbidden.push((l2, r1));
                            } else {
                                tmp.push((l2, r1));
                            }
                            break 'l2; // (l2,r2) and (l1,r1) has already done reduction for  this (l1,r1)
                        } else {
                            // here we have resigned from the possible cross-switching
                            tmp.push((l2, r2));
                        }
                    } else {
                        // here cross-switching was not possible due to common neighbors
                        tmp.push((l2, r2));
                    }
                } // `l2: while let Some((l2, r2)) = pairs_forbidden.pop()
                // restore from pairs `tmp` after finishing or breaking the loop
                pairs_forbidden = [tmp, pairs_forbidden].concat();
                // println!("AFTER 'l2: (l1, r1) = ({}, {}), pf={}, pnf={}", l1, r1, pairs_forbidden.len(), pairs_not_forbidden.len() );
                if was_reduction[0] || was_reduction[1] {
                    continue 'l1; // this (l1,r1) has already reduction
                }

                // We are here, sice (l1, r1) has not been reduced above
                // Test if you can reduce by crossing between (l,r1) and and some (l2,r2) from `pairs_not_forbidden`
                let mut tmp: Vec<(usize, usize)> = vec![];
                let mut was_reduction = vec![false, false];
                'l3: while let Some((l2, r2)) = pairs_not_forbidden.pop() {
                    // try whether (l1,r2) (l2,r1) can replace (l1,r1) and (l2,r2)
                    if l1 != l2
                        && r1 != r2
                        && !nbrs_of_l[l1].contains(&r2)
                        && !nbrs_of_l[l2].contains(&r1)
                        && !nbrs_of_r[r1].contains(&l2)
                        && !nbrs_of_r[r2].contains(&l1)
                    {
                        // can be cross-switched
                        was_reduction[0] = !self.forbidden.contains(&(l1, r2));
                        was_reduction[1] = !self.forbidden.contains(&(l2, r1));
                        if was_reduction[0] && was_reduction[1] {
                            // we know that both are not forbidden, do cross-switching
                            pairs_not_forbidden.push((l2, r1));
                            pairs_not_forbidden.push((l1, r2));
                            // update neighors of left sides
                            if let Some(idx) = nbrs_of_l[l1].iter().position(|x| *x == r1) {
                                nbrs_of_l[l1][idx] = r2;
                            } else {
                                panic!(
                                    "l1-r `try_switching_endpoints`: Bad nbrs in cross-switching ({}, {}) with ({}, {}) !!!",
                                    l1, r1, l2, r2
                                )
                            }
                            if let Some(idx) = nbrs_of_l[l2].iter().position(|x| *x == r2) {
                                nbrs_of_l[l2][idx] = r1;
                            } else {
                                panic!(
                                    "l2-r `try_switching_endpoints`: Bad nbrs in cross-switching ({}, {}) with ({}, {}) !!!",
                                    l1, r1, l2, r2
                                )
                            }
                            // update neighors of right sides
                            if let Some(idx) = nbrs_of_r[r1].iter().position(|x| *x == l1) {
                                nbrs_of_r[r1][idx] = l2;
                            } else {
                                panic!(
                                    "l-r1 `try_switching_endpoints`: Bad nbrs in cross-switching ({}, {}) with ({}, {}) !!!",
                                    l1, r1, l2, r2
                                )
                            }
                            if let Some(idx) = nbrs_of_r[r2].iter().position(|x| *x == l2) {
                                nbrs_of_r[r2][idx] = l1;
                            } else {
                                panic!(
                                    "l-r2 `try_switching_endpoints`: Bad nbrs in cross-switching ({}, {}) with ({}, {}) !!!",
                                    l1, r1, l2, r2
                                )
                            }
                            break 'l3; // this (l2,r2) and (l1,r1) has already done its job
                        } else {
                            // here we have resigned from the possible cross-switching
                            tmp.push((l2, r2));
                        }
                    } else {
                        // here cross-switching was not possible due to common neighbors
                        tmp.push((l2, r2));
                    }
                } // 'l3: while let Some((l2, r2)) = pairs_not_forbidden.pop()
                // restore from pairs `tmp` after finishing or breaking the loop 'l3
                pairs_not_forbidden = [tmp, pairs_not_forbidden].concat();
                if was_reduction[0] && was_reduction[1] {
                    continue 'l1; // this (l1,r1) has already reduction
                }

                not_reduced.push((l1, r1)); // Here: (l1,r1) could not be reduced
            } // 'l1: while let Some((l1, r1)) = pairs_forbidden.pop()
            failure = not_reduced.len() == len_before; // this iteration made no reduction :-(
            pairs_forbidden = not_reduced; // only `not_reduced` in 'l1 remain forbidden
        } // '_l0: while !failure && pairs_forbidden.len() > 0

        let result = [pairs_not_forbidden, pairs_forbidden].concat();

        if err.len() > 0 {
            Err(err.into())
        } else {
            Ok(result) // the number of remaining forbidden
        }
    }
}

/* example for tests
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}


// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/
