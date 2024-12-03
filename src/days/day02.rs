use crate::shared::*;

pub fn run(input: String) {
    // Parse input data
    let parsed = match parse::<u32>(&input) {
        Err(why) => panic!("Unable to parse input: {}", why),
        Ok(parsed) => parsed,
    };

    let safe = parsed.iter().map(|x| is_safe(x)).filter(|x| *x).count();
    println!("Safe plans: {safe}");
}

fn is_safe(plan: &[u32]) -> bool {
    if plan.is_empty() {
        return false;
    };

    let mut forward = plan.to_vec();
    let mut rev = forward.clone();
    rev.reverse();

    if plan.is_sorted() || rev.is_sorted() {
        let mut other = forward.clone();
        other.rotate_left(1);
        other.pop();
        forward.pop();

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
