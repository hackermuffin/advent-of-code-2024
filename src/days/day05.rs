use std::collections::HashMap;

use crate::shared::*;

#[derive(Debug)]
struct Rule {
    before: u32,
    after: u32,
}

impl Rule {
    fn new(line: &str) -> Option<Self> {
        let (before, after) = line.split_once("|")?;
        let before = str::parse::<u32>(before).ok()?;
        let after = str::parse::<u32>(after).ok()?;

        Some(Rule { before, after })
    }
}

#[derive(Debug)]
struct Update(Vec<u32>);

impl Update {
    fn check(&self, rules: &HashMap<u32, Vec<u32>>) -> bool {
        for (i, elem) in self.0.iter().enumerate() {
            let after = &self.0[i..];
            let Some(required_preceeding) = rules.get(elem) else {
                return true;
            };
            for x in after {
                if required_preceeding.contains(x) {
                    return false;
                }
            }
        }

        true
    }

    fn reorder(&mut self, rules: &HashMap<u32, Vec<u32>>) {
        let vec = &self.0;

        fn bottom_node(vec: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> Option<usize> {
            for (i, elem) in vec.iter().enumerate() {
                let after = &vec[i..];
                let prereq = rules.get(elem).expect("No dependencies");
                let mut prereq_filter = prereq.iter().filter(|x| after.contains(x));
                if prereq_filter.next().is_none() {
                    return Some(i);
                }
            }
            None
        }

        let x = vec.first().unwrap();
        let after = &vec[1..];
        if let Some(prereq) = rules.get(x) {
            let filtered = prereq
                .iter()
                .filter(|x| after.contains(x))
                .collect::<Vec<_>>();
            println!("Val: {x}\n{vec:?}\n{filtered:?}");
        };

        //for i in 0..vec.len() {
        //    let mut updates = true;
        //    while updates {
        //        updates = false;
        //        let curr = vec.get(i).unwrap();
        //        let after = &vec[i..];
        //        let Some(req_pre) = rules.get(curr) else {
        //            continue;
        //        };

        //        for x in after {
        //            if req_pre.contains(x) {

        //                // Move
        //            }
        //        }
        //    }
        //}
    }
}

#[derive(Debug)]
struct PrintJob {
    rules: HashMap<u32, Vec<u32>>,
    updates: Vec<Update>,
}

impl PrintJob {
    fn new(input: String) -> Option<Self> {
        let (rules, updates) = input.trim().split_once("\n\n")?;

        let rules = rules
            .split("\n")
            .map(Rule::new)
            .collect::<Option<Vec<_>>>()?;
        let mut rules_table: HashMap<u32, Vec<u32>> = HashMap::new();
        for rule in rules {
            let entry = rules_table.get_mut(&rule.after);
            match entry {
                Some(entry) => entry.push(rule.before),
                None => {
                    let _ = rules_table.insert(rule.after, vec![rule.before]);
                }
            }
        }

        let updates = parse::<u32>(updates, ",")
            .ok()?
            .into_iter()
            .map(Update)
            .collect();

        Some(PrintJob {
            rules: rules_table,
            updates,
        })
    }
}

pub fn run(input: String) {
    let mut job = PrintJob::new(input).expect("Unable to parse print job");

    let mut acc = 0;
    for update in &mut job.updates {
        let valid = update.check(&job.rules);
        if valid {
            let len = update.0.len();
            let mid = len / 2;
            let mid_elem = update.0.get(mid).unwrap();

            acc += mid_elem;
        } else {
            update.reorder(&job.rules);
        }
    }

    println!("Valid pattern total: {acc:?}");
}
