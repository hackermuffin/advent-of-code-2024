use std::collections::HashMap;

use crate::shared::*;

pub fn run(input: String) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    // Parse input data
    let parsed = match parse::<u32>(&input) {
        Err(why) => panic!("Unable to parse input: {}", why),
        Ok(parsed) => parsed,
    };

    // Format into left and right arrays
    parsed
        .iter()
        .map(|line| {
            if let [l, r] = line[..] {
                left.push(l);
                right.push(r);
            }
        })
        .for_each(drop);

    left.sort();
    right.sort();

    // Total up distances
    let mut total: i64 = 0;
    std::iter::zip(left.clone(), right.clone())
        .map(|(l, r)| {
            let diff: i64 = (r as i64) - (l as i64);
            let diff: i64 = diff.abs();
            total += diff
        })
        .for_each(drop);

    println!("Total distance is {total}");

    let mut lfreq = HashMap::new();

    left.iter()
        .map(|x| lfreq.insert(*x, freq(*x, &right)))
        .for_each(drop);

    let sim: usize = left
        .iter()
        .map(|x| *x as usize * *lfreq.get(x).expect("Failed to find frequency for {x}"))
        .sum();

    println!("Similarity: {sim}");
}

fn freq(target: u32, list: &[u32]) -> usize {
    list.iter().filter(|&&x| x == target).count()
}
