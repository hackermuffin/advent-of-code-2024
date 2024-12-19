use crate::shared::Cache;
use std::io::{stdout, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Stone(u64);

impl Stone {
    fn split(&self) -> Option<(Stone, Stone)> {
        let str = self.0.to_string();
        let len = str.len();

        if len % 2 == 0 {
            let (l, r) = str.split_at(len / 2);
            let (l, r) = (str::parse(l).ok()?, str::parse(r).ok()?);
            Some((Stone(l), Stone(r)))
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn next(&self) -> Vec<Self> {
        let val = self.0;

        if val == 0 {
            vec![Stone(1)]
        } else if let Some((l, r)) = self.split() {
            vec![l, r]
        } else {
            vec![Stone(val * 2024)]
        }
    }

    fn skip_unordered(&self, n: usize) -> Vec<Self> {
        let mut vec = vec![*self];

        for _ in 0..n {
            let mut new_elems = Vec::new();
            for stone in vec.iter_mut() {
                let new_stone = if stone.0 == 0 {
                    Stone(1)
                } else if let Some((l, r)) = stone.split() {
                    new_elems.push(r);
                    l
                } else {
                    Stone(stone.0 * 2024)
                };
                let _ = std::mem::replace(stone, new_stone);
            }
            new_elems.into_iter().for_each(|x| vec.push(x));
        }

        vec
    }

    fn skip_unordered_cache(&self, n: usize, cache: &mut Cache<Self, Vec<Self>>) -> Vec<Self> {
        cache.get_or_set(*self, || self.skip_unordered(n))
    }
}

#[derive(Debug, Clone)]
struct Stones(Vec<Stone>);

impl Stones {
    fn new(input: &str) -> Option<Self> {
        let stones = input
            .trim()
            .split(" ")
            .map(|x| Some(Stone(str::parse(x).ok()?)))
            .collect::<Option<Vec<_>>>()?;
        Some(Stones(stones))
    }

    #[allow(dead_code)]
    fn next(&mut self) {
        self.0 = self.0.iter().flat_map(|x| x.next()).collect();
    }

    fn skip75(&self) -> u64 {
        // Cache for skipping 25 elements
        let mut cache25 = Cache::<Stone, Vec<Stone>>::new();
        // Cache for skipping from elem 25 to 75 count
        let mut cache_end = Cache::<Stone, u64>::new();

        // Compute 25 step
        let stones25 = self
            .0
            .iter()
            .flat_map(|stone| stone.skip_unordered_cache(25, &mut cache25))
            .collect::<Vec<_>>();

        // Compute 25 -> 75 step, caching where possible
        let count25 = stones25.len();
        let x = stones25
            .iter()
            .enumerate()
            .map(|(i, stone25)| {
                print!("\r{}% - {:?}", i * 100 / count25, stone25);
                stdout().flush().expect("Failed to flush stdout");
                cache_end.get_or_set(*stone25, || {
                    // Value wasn't in instant cache, need to compute
                    let stones50 = stone25.skip_unordered_cache(25, &mut cache25);

                    let mut acc: u64 = 0;
                    for stone50 in stones50 {
                        let count75 = stone50.skip_unordered_cache(25, &mut cache25).len();
                        acc += count75 as u64;
                    }

                    acc
                })
            })
            .sum();
        println!();

        x
    }
}

impl std::cmp::PartialEq for Stones {
    fn eq(&self, other: &Self) -> bool {
        let mut l = self.0.clone();
        l.sort();
        let mut r = other.0.clone();
        r.sort();
        l == r
    }
}

pub fn run(input: String) {
    let stones = Stones::new(&input).expect("Unable to parse input");
    let pt1 = stones
        .0
        .iter()
        .flat_map(|stone| stone.skip_unordered(25))
        .count();
    println!("After 25 iterations, {pt1} stones present.");

    let pt2 = stones.skip75();
    println!("After 75 iterations, {pt2} stones present.");
}
