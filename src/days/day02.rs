use crate::shared::*;

pub fn run(input: String) {
    // Parse input data
    let parsed = match parse::<u32>(&input) {
        Err(why) => panic!("Unable to parse input: {}", why),
        Ok(parsed) => parsed,
    };

    let safe = parsed.iter().map(is_safe).filter(|x| *x).count();
    println!("{}", safe);
}

fn is_safe(plan: &Vec<u32>) -> bool {
    if plan.is_empty() {
        return false;
    };

    println!("{plan:?}");
    let mut forward = plan.clone();
    let mut rev = plan.clone();
    rev.reverse();

    if plan.is_sorted() || rev.is_sorted() {
        let mut other = plan.clone();
        other.rotate_left(1);
        other.pop();
        forward.pop();

        let diff: Vec<_> = other
            .iter()
            .zip(forward.clone())
            .map(|(x, y)| (((*x) as i64) - (y as i64)).abs())
            .collect();
        let safe = other
            .iter()
            .zip(forward)
            .map(|(x, y)| (((*x) as i64) - (y as i64)).abs())
            .filter(|x| !((1..4).contains(x)))
            .count()
            == 0;

        return safe;
    }

    false
}
