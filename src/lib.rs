use rand::Rng;
use std::fmt;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// div_ceil
pub fn div_ceil(left: usize, right: usize) -> usize {
    (left + right - 1) / right
}

// Left
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Left(usize);

impl fmt::Display for Left {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let &Left(i) = &self;
        write!(f, "L_{}", &i)
    }
}

// Right
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Right(usize);

impl fmt::Display for Right {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let &Right(i) = &self;
        write!(f, "R_{}", &i)
    }
}

// Pair
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pair(Left, Right);

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
    pub fn kmn_pairs(k: usize, m: usize, n: usize) -> Pairs {
        // check the parameters: 1 <= k <= m <= n
        if !(1 <= k && k <= m && m <= n) {
            panic!(
                "kmn_pairs( {}, {}, {} ) should be 1 <= k && k <= m && m <= n",
                k, m, n
            );
        }

        let p = div_ceil(k * n, m); // p = ceil( k*n / m );

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
        let base = v.len(); // treat as two-digit numbers positional system with the base
        v.sort_by(|a, b| {
            let Pair(Left(la), Right(ra)) = a;
            let Pair(Left(lb), Right(rb)) = b;
            let a = la * base + ra;
            let b = lb * base + rb;
            (a).cmp(&b)
        });
    }

    pub fn sort_by_right(&mut self) {
        let Pairs(v) = self;
        let base = v.len(); // treat as two-digit numbers positional system with the base
        v.sort_by(|a, b| {
            let Pair(Left(la), Right(ra)) = a;
            let Pair(Left(lb), Right(rb)) = b;
            let a = ra * base + la;
            let b = rb * base + lb;
            (a).cmp(&b)
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
}

impl fmt::Display for Assignments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\nAssignments (k,m,n) = {:?} :\n  [\n",
            (self.k, self.m, self.n)
        )?;
        let Pairs(v) = &self.pairs;
        let mut count: usize = 0;
        for idx in 0..v.len() {
            let (l, r) = (
                self.l_permutation.value(v[idx].left()),
                self.r_permutation.value(v[idx].right()),
            );
            let mut warn = "";
            if self.forbidden.contains(&(l, r)) {
                count = count + 1;
                warn = " !!!";
            }
            write!(f, "    {} {}{}\n", l, r, warn)?;
        }
        write!(f, "  ]\n")?;
        write!(
            f,
            "\nForbidden ({} / {}):\n  [\n",
            &self.forbidden.len(),
            count
        )?;
        for (i, j) in &self.forbidden {
            write!(f, "    {} {}\n", i, j)?;
        }
        write!(f, "  ]\n")
    }
}

impl Assignments {
    pub fn new(k: usize, m: usize, n: usize) -> Self {
        let pairs = Pairs::kmn_pairs(k, m, n);
        let l_permutation = Permutation::new(m);
        let r_permutation = Permutation::new(n);
        let forbidden = Vec::new();
        Self {
            k,
            m,
            n,
            pairs,
            l_permutation,
            r_permutation,
            forbidden,
        }
    }

    // get tuple with parameters: (k,m,n)
    pub fn get_kmn(&self) -> (usize, usize, usize) {
        (self.k, self.m, self.n)
    }

    pub fn number_of_forbidden_used(&self) -> usize {
        let Pairs(v) = &self.pairs;
        let mut count: usize = 0;
        for idx in 0..v.len() {
            let (l, r) = (
                self.l_permutation.value(v[idx].left()),
                self.r_permutation.value(v[idx].right()),
            );
            if self.forbidden.contains(&(l, r)) {
                count = count + 1;
            }
        }
        count
    }

    pub fn group_by_left(&mut self) {
        self.pairs.sort_by_left();
    }

    pub fn group_by_right(&mut self) {
        self.pairs.sort_by_right();
    }

    pub fn add_forbidden(&mut self, l: usize, r: usize) -> Result<(), &'static str> {
        if l >= self.m {
            Err("adding forbidden (l,r) with l >= m, where the left set is {0,...,m-1}")
        } else if r >= self.n {
            Err("adding forbidden (l,r) with r >= n, where the right set is {0,...,n-1}")
        } else if self.forbidden.contains(&(l, r)) {
            Err("adding forbidden (l,r) that is already in forbidden")
        } else {
            self.forbidden.push((l, r));
            self.forbidden.sort();
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
        for i in 0..self.pairs.len() {
            let Pairs(v) = &self.pairs;
            let (l, r) = (
                self.l_permutation.value(v[i].left()),
                self.r_permutation.value(v[i].right()),
            );
            if self.forbidden.contains(&(l, r)) {
                self.r_permutation.swap(r, rng.random_range(0..n));
            }
        }
    }

    pub fn random_swaps_of_l_forbidden(&mut self, rng: &mut impl Rng) {
        let m = self.m; // for left side
        for i in 0..self.pairs.len() {
            let Pairs(v) = &self.pairs;
            let (l, r) = (
                self.l_permutation.value(v[i].left()),
                self.r_permutation.value(v[i].right()),
            );
            if self.forbidden.contains(&(l, r)) {
                self.l_permutation.swap(l, rng.random_range(0..m));
            }
        }
    }
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
