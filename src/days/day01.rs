pub fn run(input: String) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    // Parse out left and right arrays
    input
        .lines()
        .map(
            |line| match line.split(" ").filter(|x| x != &"").collect::<Vec<&str>>()[..] {
                [l, r] => {
                    let l = match str::parse::<u32>(l) {
                        Err(why) => panic!("Could not parse {}: {}", l, why),
                        Ok(l) => l,
                    };
                    let r = match str::parse::<u32>(r) {
                        Err(why) => panic!("Could not parse {}: {}", r, why),
                        Ok(r) => r,
                    };
                    left.push(l);
                    right.push(r);
                }
                _ => {
                    println!("Skipping reading in line: {line}");
                }
            },
        )
        .for_each(drop);

    left.sort();
    right.sort();

    let mut total: i64 = 0;
    std::iter::zip(left, right)
        .map(|(l, r)| {
            let diff: i64 = (r as i64) - (l as i64);
            let diff: i64 = diff.abs();
            total += diff
        })
        .for_each(drop);

    println!("Total distance is {total}")
}
