use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::Write;

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

    fn reorder(&mut self, rules: &HashMap<u32, Vec<u32>>) -> Option<()> {
        let graph = Graph::new(&self.0, rules);
        let path = graph.longest_path()?;
        self.0 = path.iter().map(|node| node.val).collect();
        Some(())
    }

    fn middle(&self) -> u32 {
        let len = self.0.len();
        let mid = len / 2;
        *self.0.get(mid).unwrap()
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

#[derive(PartialEq, Eq)]
struct Node<T> {
    val: T,
    links: Vec<usize>,
}

impl<T: std::fmt::Debug> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} -> {:?}", self.val, self.links)
    }
}

struct Graph<T> {
    index: HashMap<T, usize>,
    nodes: Vec<Node<T>>,
}

impl<T: std::fmt::Debug> fmt::Debug for Graph<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nodes: {:?}\n{:?}", self.index, self.nodes)
    }
}

impl<T: std::hash::Hash + std::cmp::Eq + Copy + std::fmt::Debug> Graph<T> {
    fn new(input_nodes: &[T], rules: &HashMap<T, Vec<T>>) -> Self {
        // Construct nodes & node mapping
        let mut index = HashMap::new();
        let mut nodes = Vec::new();
        input_nodes.iter().for_each(|x| {
            let node = Node {
                val: *x,
                links: Vec::new(),
            };
            nodes.push(node);
            index.insert(*x, nodes.len() - 1);
        });

        // Construct links to other nodes
        nodes.iter_mut().for_each(|node| {
            if let Some(prereqs) = rules.get(&node.val) {
                let links = &mut node.links;
                prereqs
                    .iter()
                    .filter(|x| input_nodes.contains(x))
                    .for_each(|prereq| {
                        let curr_index = *index.get(prereq).unwrap();
                        links.push(curr_index);
                    });
            };
        });

        Graph { index, nodes }
    }

    fn children(&self, target: &Node<T>) -> Option<Vec<&Node<T>>> {
        let index = self.index.get(&target.val)?;
        let node = self.nodes.get(*index)?;
        node.links
            .iter()
            .map(|link| self.nodes.get(*link))
            .collect::<Option<Vec<_>>>()
    }

    fn longest_path(&self) -> Option<Vec<&Node<T>>> {
        //println!("Starting looking for longets path in:\n{self:?}");
        let mut curr = vec![self.top_node()];
        self.path(&mut curr)?;
        //println!("Longest path found: {curr:?}");
        Some(curr)
    }

    fn top_node(&self) -> &Node<T> {
        let all_children = self
            .nodes
            .iter()
            .flat_map(|node| self.children(node).unwrap())
            .collect::<Vec<_>>();
        let top_node_index = self
            .nodes
            .iter()
            .map(|node| {
                all_children
                    .iter()
                    .filter(|&x| x == &node)
                    .collect::<Vec<_>>()
                    .len()
            })
            .enumerate()
            .find(|(_, b)| *b == 0)
            .expect("Unable to find top node")
            .0;
        self.nodes.get(top_node_index).unwrap()
    }

    fn path<'a>(&'a self, curr: &mut Vec<&'a Node<T>>) -> Option<()> {
        let last = curr.last().unwrap();
        let children = self
            .children(last)
            .unwrap()
            .into_iter()
            .filter(|x| !curr.contains(x))
            .collect::<Vec<_>>();

        if children.is_empty() {
            // Check if all nodes used
            if curr.len() == self.nodes.len() {
                Some(())
            } else {
                None
            }
        } else {
            for child in children {
                curr.push(child);
                match self.path(curr) {
                    Some(()) => return Some(()),
                    None => curr.pop(),
                };
            }
            None
        }
    }
}

pub fn run(input: String) {
    let mut job = PrintJob::new(input).expect("Unable to parse print job");
    let len = job.updates.len();

    println!("Job contains {len} updates");

    let mut valid_acc = 0;
    let mut reorder_acc = 0;
    for (i, update) in &mut job.updates.iter_mut().enumerate() {
        print!("\r{}%", (100 * i) / (len));
        io::stdout().flush().expect("Failed to flush stdout");
        let valid = update.check(&job.rules);
        if valid {
            valid_acc += update.middle()
        } else {
            update.reorder(&job.rules);
            reorder_acc += update.middle()
        }
    }

    println!("Valid pattern total: {valid_acc:?}");
    println!("Reordered pattern total: {reorder_acc:?}");
}
