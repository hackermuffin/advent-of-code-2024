pub fn run(input: String) {
    println!("Multiply sum: {}", sum_all_multiplies(&input));

    // Part 2
    let do_split = input.split("do()");
    let trimmed = do_split.map(|x| x.split("don't()").next().unwrap());
    let sum: u32 = trimmed.map(sum_all_multiplies).sum();
    println!("Multiply sum with control flow: {sum}");
}

fn sum_all_multiplies(input: &str) -> u32 {
    let split = input.split("mul(");
    let mut parsed = split.map(parse_mul).collect::<Vec<_>>();
    parsed.retain(|x| x.is_some());
    parsed.iter().map(|x| x.unwrap()).sum()
}

// Parse mul extracting multiplied value, or nothing if the string is invalid
fn parse_mul(val: &str) -> Option<u32> {
    let vals = val.split(")").next()?;
    let mut split = vals.split(",");
    let l = str::parse::<u32>(split.next()?).ok()?;
    let r = str::parse::<u32>(split.next()?).ok()?;
    Some(l * r)
}
