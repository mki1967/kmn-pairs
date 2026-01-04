// `rank`
use kmn_pairs::kmn_serde::*;
use kmn_pairs::menu::*;
use kmn_pairs::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Write;
/*
pub fn hello() {
    println!("hello from rank mod");
}
*/

pub fn rank_menu() {
    let mut ranking_data: Option<Ranking> = None;
    input_menu(&mut ranking_data);
    if let Some(mut ranking) = ranking_data {
        edit_menu(&mut ranking);
        println!("\nRANKING JSON:\n");
        match serde_json::to_string(&SerdeRanking::from(&ranking)) {
            Ok(out) => {
                println!("{}\n", out)
            }
            Err(err) => {
                panic!("{}", err)
            }
        }
    } else {
        println!("NO RANKING SET!");
        return;
    }
}

fn input_menu(ranking_data: &mut Option<Ranking>) {
    'input: loop {
        println!("\nDEFINE RANKING");
        println!("-> Input command (h for help): ");
        let cmd = read_line();
        let cmd = cmd.trim();
        match cmd {
            "h" => {
                println!(
                    "
        command action:
            ia         input assignments for new ranking
            json       input one-line JSON ranking data
            quit       quit 'DEFINE RANKING' menu without defining ranking
            "
                );
            }
            "quit" => {
                break 'input;
            }
            "ia" => {
                println!("DEFINE SOME ASSIGNMENT AND THEN 'quit':");
                let mut ranking = Ranking::new();
                kmn_pairs_menu(&mut ranking.assignments_data);
                if let Some(_) = ranking.assignments_data {
                    ranking.make_vectors();
                    *ranking_data = Some(ranking);
                    break 'input;
                }
            }
            "json" => {
                println!("input one-line json: ");
                let input = read_line();
                let deserialized: Result<SerdeRanking, serde_json::Error> =
                    serde_json::from_str(&input);
                match deserialized {
                    Ok(deserialized) => {
                        let mut ranking = Ranking::from(&deserialized);
                        match ranking.test() {
                            Ok(()) => {
                                if let Err(err) = ranking.warnings() {
                                    println!("{err}");
                                }
                                *ranking_data = Some(ranking);
                                println!("Ranking set!");
                                // TODO: print warnings for this ranking
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

pub fn edit_menu(ranking: &mut Ranking) {
    let mut rng = rand::rng(); // random number generator
    if ranking.assignments_data.is_none() {
        println!("You have to define some assigments first!!!");
        return;
    }

    // uncomment below if needed
    // let (k, m, n) = ranking.assignments_data.as_ref().unwrap().get_kmn(); // unwrap should be save here
    //let p = ranking.assignments_data.as_ref().unwrap().p();

    'edit: loop {
        // TODO

        println!("\nEDIT RANKING");
        if let Some(assignments) = &ranking.assignments_data {
            println!(
                "{}\n{}",
                assignments.assignments_header(),
                assignments.forbidden_header()
            );
            if let Err(err) = ranking.warnings() {
                println!("{err}");
            }
        } else {
            println!("edit_menu: NO ASSIGNMENTS !!!");
            break;
        }

        println!("-> Input command (h for help): ");

        let cmd = read_line();
        let cmd = cmd.trim();
        match cmd {
            "h" => {
                println!(
                    "
        command action:
            ea         edit assignments
            prlvrvj    print rankings
            prankersf  print rankers' infos with forbidden
            prkrsfar   print rankers' infos with forbidden, assignments and rankings
            pscores    print scores
            presults   print results
            PSCORES!   force printing scores (despite of invalid rankings)
            PRESULTS!  force printing results (despite of invalid rankings)
            tr         test rankings
            json       input one-line JSON ranking data
            iranker    input rankers' info label
            iranked    input ranked info label
            irlvrvj    input rankings from one-line JSONs
            dr         delete rankings
            simid      simulate rankings by identit rankers' scores +/- random_dev
            simrand    simulate rankings by random rankers' scores
            quit       quit 'EDIT RANKING' menu without defining ranking
            "
                );
            }
            "ea" => {
                kmn_pairs_menu(&mut ranking.assignments_data);
            }
            "json" => {
                println!("\nRANKING JSON:\n");
                match serde_json::to_string(&SerdeRanking::from(&*ranking)) {
                    Ok(out) => {
                        println!("{}", out)
                    }
                    Err(err) => {
                        println!("{}", err)
                    }
                }
            }
            "irlvrvj" => {
                irlvrvj(ranking);
            }
            "prlvrvj" => {
                prlvrvj(ranking);
            }
            "prankersf" => {
                prankersf(ranking);
            }
            "prkrsfar" => {
                prkrsfar(ranking);
            }
            "pscores" => {
                pscores(ranking, &vec![]);
            }
            "presults" => {
                presults(ranking, &vec![]);
            }
            "PSCORES!" => {
                println!("FORCING !!!");
                pscores(ranking, &vec![KmnOption::Force]);
            }
            "PRESULTS!" => {
                println!("FORCING !!!");
                presults(ranking, &vec![KmnOption::Force]);
            }
            "tr" => {
                println!("{cmd}: Testing the rankings:\n");
                if let Err(err) = ranking.test_rankings() {
                    println!("{err}");
                } else {
                    println!("{cmd}: Ok");
                }
            }
            "dr" => 'dr: loop {
                println!("{cmd}: input ranker < {}", ranking.rankers.len());
                let input = read_line();
                match serde_json::from_str::<usize>(&input) {
                    Ok(ranker) => {
                        if ranker >= ranking.rankers.len() {
                            println!(
                                "{cmd}: You have input ranker = {ranker} >= {} !!!",
                                ranking.rankers.len()
                            );
                        } else {
                            ranking.rankers[ranker].ranking = None;
                            println!("{cmd}: ranking of ranker {ranker} deleted!");
                        }
                    }
                    Err(err) => {
                        println!("{err}");
                        break 'dr;
                    }
                }
            },
            "iranker" => 'iranker: loop {
                println!("{cmd}: input ranker's id (id < {}):", ranking.rankers.len());
                let input = read_line();
                match serde_json::from_str::<usize>(&input) {
                    Ok(id) => {
                        if id >= ranking.rankers.len() {
                            println!(
                                "{cmd}: You have input ranker's id = {id} >= {} !!!",
                                ranking.rankers.len()
                            );
                        } else {
                            println!("{cmd}: input info label for ranker {id}:");
                            let mut info = read_line();
                            info = info.trim().to_string();
                            ranking.rankers[id].info = Some(String::from(info));
                            println!(
                                "{cmd}: info of ranker {id} set to: {:?}",
                                ranking.rankers[id].info
                            );
                        }
                    }
                    Err(err) => {
                        println!("{err}");
                        break 'iranker;
                    }
                }
            },
            "iranked" => 'iranker: loop {
                println!("{cmd}: input ranked id (id < {}):", ranking.ranked.len());
                let input = read_line();
                match serde_json::from_str::<usize>(&input) {
                    Ok(id) => {
                        if id >= ranking.rankers.len() {
                            println!(
                                "{cmd}: You have input ranked id = {id} >= {} !!!",
                                ranking.ranked.len()
                            );
                        } else {
                            println!("{cmd}: input info label for ranker {id}:");
                            let mut info = read_line();
                            info = info.trim().to_string();
                            ranking.ranked[id].info = Some(String::from(info));
                            println!(
                                "{cmd}: info of ranker {id} set to: {:?}",
                                ranking.ranked[id].info
                            );
                        }
                    }
                    Err(err) => {
                        println!("{err}");
                        break 'iranker;
                    }
                }
            },
            "simid" => {
                let (f, f_str) = (
                    |ranked: usize| ranked as f64,
                    "score(ranked) = ranked +/- random_dev",
                );
                println!("{cmd}: simulating by: '{}'", f_str);
                println!("{cmd}: input max_dev:");
                let input = read_line();
                let Ok(max_dev) = serde_json::from_str::<f64>(&input) else {
                    continue;
                };
                match ranking.set_rankings_by(&mut rng, max_dev, f) {
                    Ok(()) => {
                        prlvrvj(ranking);
                    }
                    Err(err) => {
                        println!("{err}");
                    }
                }
            }
            "simrand" => {
                let (f, f_str) = (|_ranked: usize| 0 as f64, "score(ranked) = random");
                println!("{cmd}: simulating by: '{}'", f_str);
                match ranking.set_rankings_by(&mut rng, 1.0, f) {
                    Ok(()) => {
                        prlvrvj(ranking);
                    }
                    Err(err) => {
                        println!("{err}");
                    }
                }
            }
            "quit" => {
                break 'edit;
            }
            _ => println!("Unknown command: {}", cmd),
        }
    }
}

// menu actions:

pub fn pscores(ranking: &Ranking, options: &Vec<KmnOption>) {
    let cmd = "pscores";
    println!("{cmd}: Scores of the ranked:");
    match ranking.collected_scores(options) {
        Ok(scores) => {
            for score in scores {
                match serde_json::to_string(&score) {
                    Ok(out) => {
                        println!("{}", out)
                    }
                    Err(err) => {
                        println!("{}", err)
                    }
                }
            }
        }
        Err(err) => {
            println!("{err}");
        }
    }
}

pub fn presults(ranking: &Ranking, options: &Vec<KmnOption>) {
    let cmd = "presults";
    println!("{cmd}: Positions (by average score) of the ranked:");
    match ranking.results(options) {
        Ok(results) => {
            for position in results {
                println!("\nPosition 'after {}':\n", position.after);
                for ranked_avg in position.ranked_set {
                    match serde_json::to_string(&ranked_avg) {
                        Ok(out) => {
                            println!("  {}", out)
                        }
                        Err(err) => {
                            println!("{}", err)
                        }
                    }
                }
            }
        }
        Err(err) => {
            println!("{err}");
        }
    }
}

// Print rankings:
// print one-line ranking JSONs for rankers (left IDs)
pub fn prlvrvj(ranking: &Ranking) {
    let cmd = "prlvrvj";
    println!("{cmd}: RANKINGS (left = [ranker], right = ranking):");
    for l in 0..ranking.rankers.len() {
        prllvrvj(ranking, l);
    }
}
// print one-line ranking JSON for left ID
pub fn prllvrvj(ranking: &Ranking, l: usize) {
    // `l` (left) is the ranker
    LeftVecRightVec {
        left: vec![l],
        right: if let Some(ranking) = &ranking.rankers[l].ranking {
            (*ranking).clone()
        } else {
            vec![]
        },
    }
    .println_serde();
}

// print rankers' infos with forbidden
pub fn prankersf(ranking: &Ranking) {
    let cmd = "prankersf";
    let Some(assignments) = &ranking.assignments_data else {
        println!("{cmd}: assignments_data == None");
        return;
    };
    // TODO
    let forbidden = assignments.forbidden();
    for l in 0..ranking.rankers.len() {
        println!("\n---------------------------------------------------------------------\n");
        println!(
            "ID = {l},  INFO = {:?}",
            if let Some(i) = &ranking.rankers[l].info {
                i
            } else {
                ""
            }
        );
        println!("Forbidden (left = [ranker], right = forbidden):");
        LeftVecRightVec {
            left: vec![l],
            right: right_neighbors(forbidden, l),
        }
        .println_serde();
    }
    println!("\n---------------------------------------------------------------------\n");
}

// print rankers' infos with forbidden, assignments and  rankings
pub fn prkrsfar(ranking: &Ranking) {
    let cmd = "prankersf";
    let Some(assignments) = &ranking.assignments_data else {
        println!("{cmd}: assignments_data == None");
        return;
    };
    // TODO
    let forbidden = assignments.forbidden();
    let assignments = assignments.get_pairs_of_ids();
    for l in 0..ranking.rankers.len() {
        println!("\n---------------------------------------------------------------------\n");
        println!(
            "ID = {l},  INFO = {:?}",
            if let Some(i) = &ranking.rankers[l].info {
                i
            } else {
                ""
            }
        );
        println!("Forbidden (left = [ranker], right = forbidden):");
        LeftVecRightVec {
            left: vec![l],
            right: right_neighbors(forbidden, l),
        }
        .println_serde();
        println!("Assigned (left = [ranker], right = assigned):");
        LeftVecRightVec {
            left: vec![l],
            right: right_neighbors(&assignments, l),
        }
        .println_serde();
        println!("Ranking (left = [ranker], right = assigned):");
        prllvrvj(ranking, l);
        if let Some(rnk) = &ranking.rankers[l].ranking {
            if let Err(err) = ranking.test_ranking(l, rnk) {
                println!("{err}");
            }
        }
    }
    println!("\n---------------------------------------------------------------------\n");
}

// input one-line ranking JSONs for rankers (left IDs)
pub fn irlvrvj(ranking: &mut Ranking) {
    let cmd = "irlvrvj";
    'ir: loop {
        // if you want `break 'ir`
        // loop {
        println!("{cmd}: input one-line json: ");
        let input = read_line();
        let deserialized: Result<LeftVecRightVec, serde_json::Error> = serde_json::from_str(&input);
        match deserialized {
            Ok(deserialized) => {
                let left = &deserialized.left;
                let right = &deserialized.right;
                if left.len() != 1 {
                    println!("{cmd}: left = {left:?} should have one element - the ranker id");
                } else {
                    // Ok, do "ir"
                    let ranker = left[0];
                    if let Err(err) = ranking.set_ranking(ranker, right.clone()) {
                        println!("{cmd}: {err}");
                    } else {
                        println!("{cmd}: added ranking: {ranker} -> {right:?}:");
                    }
                }
            }

            Err(err) => {
                println!("{err}");
                break 'ir;
            }
        }
    }
}

// Ranker
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ranker {
    // id: usize,                 // let id be its position in Ranking.rankers
    info: Option<String>,        // optional info
    ranking: Option<Vec<usize>>, // this ranker's ranking of it's p assigned ranked
}

impl Ranker {
    pub fn ranking_set(&mut self, ranking: Option<Vec<usize>>) {
        self.ranking = ranking;
    }
}

// Ranked
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ranked {
    // id: usize,                  // let id be its position in Ranking.ranked
    info: Option<String>, // optional info
}

// Ranking
#[derive(Debug)]
pub struct Ranking {
    assignments_data: Option<Assignments>,
    rankers: Vec<Ranker>, // sequence of m rankers
    ranked: Vec<Ranked>,  // sequence of n ranked
}

impl Ranking {
    pub fn new() -> Self {
        Self {
            assignments_data: None,
            rankers: vec![],
            ranked: vec![],
        }
    }

    pub fn make_vectors(&mut self) {
        if let Some(assignments) = &self.assignments_data {
            let (_k, m, n) = assignments.get_kmn();
            for i in 0..m {
                self.rankers.push(Ranker {
                    info: Some(i.to_string()),
                    ranking: None,
                });
            }
            for i in 0..n {
                self.ranked.push(Ranked {
                    info: Some(i.to_string()),
                });
            }
        }
    }

    // make ranker scores for simulations
    pub fn make_ranker_scores_by<F>(
        &self,
        ranker: usize,
        score: F,
    ) -> Result<Vec<(usize, f64)>, Box<dyn Error>>
    where
        F: Fn(usize) -> f64,
    {
        let assigned = self.assigned_to_ranker(ranker)?; // also checks: ranker < m
        let mut scores = vec![];
        for ranked in assigned {
            scores.push((ranked, score(ranked)));
        }
        Ok(scores)
    }

    pub fn set_rankings_by<F>(
        &mut self,
        rng: &mut impl Rng,
        max_dev: f64,
        score: F,
    ) -> Result<(), Box<dyn Error>>
    where
        F: Fn(usize) -> f64,
    {
        for ranker in 0..self.rankers.len() {
            let mut scores = self.make_ranker_scores_by(ranker, &score)?;
            for (_, r) in &mut scores {
                *r = *r + max_dev * (2.0 * rng.random::<f64>() - 1.0);
            }
            self.set_ranking_by_ranker_scores(ranker, &mut scores)?;
        }
        Ok(())
    }

    // set ranker's ranking by private scores of assigned ranked.
    pub fn set_ranking_by_ranker_scores(
        &mut self,
        ranker: usize,
        scores: &mut Vec<(usize, f64)>,
    ) -> Result<(), Box<dyn Error>> {
        let mut err = String::new();
        let assigned = self.assigned_to_ranker(ranker)?; // also checks: ranker < m
        if assigned.len() != scores.len() {
            writeln!(
                &mut err,
                "set_ranking_by_ranker_scores: for ranker = {ranker}, assigned.len() = {} != scores.len() = {}. NOT SET !!!",
                assigned.len(),
                scores.len(),
            )?;
            return Err(err.into());
        }
        let mut scored = scores
            .iter()
            .map(|x| {
                let (l, _) = x;
                *l
            })
            .collect::<Vec<usize>>();
        scored.sort();
        if &assigned != &scored {
            writeln!(
                &mut err,
                "set_ranking_by_ranker_scores: assigned = {:?} != scored = {:?}. NOT SET !!!",
                assigned, scored,
            )?;
            return Err(err.into());
        }

        // get `ranking` as the left elements by the decreasing sorted scores
        scores.sort_by(|a, b| {
            let (_, ra) = a;
            let (_, rb) = b;
            rb.total_cmp(ra) // instead of `cmp` - some problems with NaN ... ;-)
        });
        let ranking = scores
            .iter()
            .map(|x| {
                let (l, _) = x;
                *l
            })
            .collect::<Vec<usize>>();

        // return
        self.set_ranking(ranker, ranking)
    }

    // test_ranking
    pub fn test_ranking(&self, ranker: usize, ranking: &Vec<usize>) -> Result<(), Box<dyn Error>> {
        let mut err = String::new();
        let assigned = self.assigned_to_ranker(ranker)?; // also checks: ranker < m
        let mut sorted_ranked = ranking.clone();
        sorted_ranked.sort(); // now - sorted!
        if assigned != sorted_ranked {
            writeln!(
                &mut err,
                "test_ranking: for ranker = {ranker}, assigned = {assigned:?} != sorted_ranked = {sorted_ranked:?} !!!"
            )?;
            return Err(err.into());
        }
        if err.len() > 0 {
            Err(err.into())
        } else {
            Ok(())
        }
    }

    pub fn test_rankings(&self) -> Result<(), Box<dyn Error>> {
        let mut err = String::new();

        for ranker in 0..self.rankers.len() {
            match &self.rankers[ranker].ranking {
                None => {
                    writeln!(
                        &mut err,
                        "test_rankings: rankers[{ranker}].ranking is None !!!"
                    )?;
                }
                Some(ranking) => {
                    if let Err(err1) = self.test_ranking(ranker, &ranking) {
                        writeln!(&mut err, "{err1}")?;
                    }
                }
            }
        }
        if err.len() > 0 {
            Err(err.into())
        } else {
            Ok(())
        }
    }

    // set ranking for the ranker
    pub fn set_ranking(
        &mut self,
        ranker: usize,
        ranking: Vec<usize>,
    ) -> Result<(), Box<dyn Error>> {
        if ranking.len() == 0 {
            // SPECIAL CASE: treat [] as `None`
            self.rankers[ranker].ranking_set(None);
        } else {
            self.test_ranking(ranker, &ranking)?; // test for not [] only
            self.rankers[ranker].ranking_set(Some(ranking));
        };

        Ok(())
    }

    // get sorted ranked assigned to the ranker
    pub fn assigned_to_ranker(&self, ranker: usize) -> Result<Vec<usize>, Box<dyn Error>> {
        let mut err = String::new();
        let Some(assignments) = &self.assignments_data else {
            writeln!(&mut err, "assigned_to_ranker: assignments_data == None")?;
            return Err(err.into());
        };
        let (_, m, _) = assignments.get_kmn();

        if ranker > m {
            writeln!(&mut err, "assigned_to_ranker: ranker = {ranker} > m = {m}")?;
            return Err(err.into());
        };
        if err.len() > 0 {
            Err(err.into())
        } else {
            Ok(right_neighbors(&assignments.get_pairs_of_ids(), ranker))
        }
    }

    // get sorted rankers assigned to the ranked
    pub fn assigned_to_ranked(&self, ranked: usize) -> Result<Vec<usize>, Box<dyn Error>> {
        let mut err = String::new();
        let Some(assignments) = &self.assignments_data else {
            writeln!(&mut err, "assigned_to_ranked: assignments_data == None")?;
            return Err(err.into());
        };
        let (_, _, n) = assignments.get_kmn();

        if ranked > n {
            writeln!(&mut err, "assigned_to_ranked: ranked = {ranked} > n = {n}")?;
            return Err(err.into());
        };
        if err.len() > 0 {
            Err(err.into())
        } else {
            Ok(left_neighbors(&assignments.get_pairs_of_ids(), ranked))
        }
    }

    // test
    pub fn test(&self) -> Result<(), Box<dyn Error>> {
        let mut err = String::new();
        let Some(assignments) = &self.assignments_data else {
            writeln!(&mut err, "Ranking.test: assignments_data is None !!!")?;
            return Err(err.into());
        };
        assignments.test_assignments()?;
        // TODO: implement remaining tests!
        if err.len() > 0 {
            Err(err.into())
        } else {
            Ok(())
        }
    }

    // `warnings` must be used after `test`
    // (`&mut self` for `assignments.test_forbidden()`)
    pub fn warnings(&mut self) -> Result<(), Box<dyn Error>> {
        let mut err = String::new();
        let Some(assignments) = &mut self.assignments_data else {
            writeln!(
                &mut err,
                "Ranking.warnings: assignments_data == None (`warnings` must be used after `test` !!!)"
            )?;
            return Err(err.into());
        };

        assignments.test_forbidden()?;
        // self.test_rankings()?;    // to noisy ...
        // TODO: more warnings ...
        if err.len() > 0 {
            Err(err.into())
        } else {
            Ok(())
        }
    }

    // collected_scores sorted by avg
    pub fn collected_scores(
        &self,
        options: &Vec<KmnOption>,
    ) -> Result<Vec<ScoresFromRankers>, Box<dyn Error>> {
        self.test()?;
        // let (k, m, n) = self.assignments_data.as_ref().unwrap().get_kmn(); // unwrap should be save here
        // From here we assume correct assignments_data:
        let p = self.assignments_data.as_ref().unwrap().p();
        let dummy_score = (p + 1) as f64 / 2.0;
        let use_force = options.contains(&KmnOption::Force);
        if let Err(err) = self.test_rankings() {
            if use_force {
                println!("{err}");
                println!(
                    "
######## WARNING !!! ########
Option `Force` will use dummy_score = (p + 1) / 2.0 = {dummy_score} from invalid rankings.
"
                );
            } else {
                return Err(err);
            }
        }

        //  `scores_of_ranked[ranked]` shall be vector of (ranker,score from his ranking)
        let mut scores_of_ranked: Vec<Vec<(usize, f64)>> = vec![vec![]; self.ranked.len()];
        for ranker in 0..self.rankers.len() {
            // the first test:
            let mut ranking_ok = !(self.rankers[ranker].ranking.is_none());
            if ranking_ok {
                let ranking = self.rankers[ranker].ranking.as_ref().unwrap();
                // the second test:
                if let Err(_) = self.test_ranking(ranker, ranking) {
                    ranking_ok = false;
                }
            }
            if ranking_ok {
                let ranking = self.rankers[ranker].ranking.as_ref().unwrap();
                for i in 0..ranking.len() {
                    let ranked = ranking[i]; // ranked at position i
                    let score = (p - i) as f64; // score of this ranked from this ranker
                    scores_of_ranked[ranked].push((ranker, score));
                }
            } else {
                // TODO use ranked from ranker's assigment and  `dummy_score`
                let assigned = self.assigned_to_ranker(ranker)?; // also checks: ranker < m
                for ranked in assigned {
                    scores_of_ranked[ranked].push((ranker, dummy_score));
                    println!(
                        "dummy_score = {dummy_score} from ranker = {ranker} to ranked = {ranked} !!!"
                    );
                }
            }
        }
        let mut collected_scores: Vec<ScoresFromRankers> = vec![];
        for ranked in 0..scores_of_ranked.len() {
            // Paranoic test:
            let test_rankers = scores_of_ranked[ranked]
                .iter()
                .map(|x| {
                    let (l, _) = x;
                    *l
                })
                .collect::<Vec<usize>>();
            let assigned = self.assigned_to_ranked(ranked)?;
            if test_rankers != assigned {
                let mut err = String::new();
                writeln!(
                    &mut err,
                    "Ranking.collected_scores: for ranked = {ranked}, test_rankers = {test_rankers:?} != assigned = {assigned:?} !!!"
                )?;
                if use_force {
                    println!("{err}");
                } else {
                    return Err(err.into());
                }
            }
            let k1 = scores_of_ranked[ranked].len(); // check if in {k, k+1} ???
            let avg = scores_of_ranked[ranked]
                .iter()
                .map(|x| {
                    let (_, r) = x;
                    *r
                })
                .sum::<f64>()
                / (k1 as f64);
            let ranked_scores = ScoresFromRankers {
                ranked,
                rankers_scores: scores_of_ranked[ranked].clone(),
                avg,
            };
            collected_scores.push(ranked_scores);
        }
        collected_scores.sort_by(|a, b| b.avg.total_cmp(&a.avg));
        Ok(collected_scores)
    }

    // results
    pub fn results(&self, options: &Vec<KmnOption>) -> Result<Vec<Position>, Box<dyn Error>> {
        let collected_scores = self.collected_scores(&options)?;
        // From here we assume correct rankings and assignments_data:
        let (k, m, n) = self.assignments_data.as_ref().unwrap().get_kmn(); // unwrap should be save here
        let p = self.assignments_data.as_ref().unwrap().p();
        // compute final positions
        let mut results: Vec<Position> = vec![];
        let epsilon = 1.0 / ((k * m * n * p * 10000) as f64);
        let mut avg_base = p as f64 + epsilon + 1.0;
        let mut position = Position {
            after: n,
            ranked_set: vec![],
        }; // a placeholder - should be replaced before used !!!
        for i in 0..collected_scores.len() {
            let ranked = collected_scores[i].ranked;
            let avg = collected_scores[i].avg;
            let ranked_avg = RankedAvg { ranked, avg };
            if avg < avg_base - epsilon {
                if position.ranked_set.len() > 0 {
                    results.push(position); // push real position
                };
                position = Position {
                    after: i, // number of all ranked from higher positions
                    ranked_set: vec![ranked_avg],
                };
                avg_base = avg; // new avg_base for new position
            } else {
                position.ranked_set.push(ranked_avg);
            }
        }
        results.push(position); // push real position (or placeholder if somehow `collected_scores.len() ==  0`)
        Ok(results)
    }
}

// Results <-> presentations

#[derive(Serialize, Deserialize, Debug)]
pub struct ScoresFromRankers {
    ranked: usize,
    rankers_scores: Vec<(usize, f64)>,
    avg: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RankedAvg {
    ranked: usize,
    avg: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    after: usize, // number of all ranked from higher positions
    ranked_set: Vec<RankedAvg>,
}

/*
#[derive(Serialize, Deserialize, Debug)]
pub struct Results {
    positions: Vec<Position>,
}
*/

// Ranking <-> JSON

#[derive(Serialize, Deserialize, Debug)]
pub struct SerdeRanking {
    assignments_data: Option<SerdeKmnAssignment>,
    rankers: Vec<Ranker>, // sequence of m rankers
    ranked: Vec<Ranked>,  // sequence of n ranked
}

impl From<&Ranking> for SerdeRanking {
    fn from(item: &Ranking) -> Self {
        let assignments_data = match &item.assignments_data {
            Some(assignments) => Some(SerdeKmnAssignment::from(assignments)),
            None => None,
        };

        Self {
            assignments_data,
            rankers: item.rankers.clone(),
            ranked: item.ranked.clone(),
        }
    }
}

impl From<&SerdeRanking> for Ranking {
    fn from(item: &SerdeRanking) -> Self {
        let assignments_data = match &item.assignments_data {
            Some(assignments) => Some(Assignments::from(assignments)),
            None => None,
        };

        Self {
            assignments_data,
            rankers: item.rankers.clone(),
            ranked: item.ranked.clone(),
        }
    }
}
