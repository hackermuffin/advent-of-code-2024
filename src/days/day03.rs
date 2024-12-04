pub fn run(input: String) {
    let split = input.split("mul(");
    let mut parsed = split.map(parse_mul).collect::<Vec<_>>();
    parsed.retain(|x| x.is_some());
    let sum: u32 = parsed.iter().map(|x| x.unwrap()).sum();
    println!("Multiply sum: {sum}");
}

// Parse mul extracting multiplied value, or nothing if the string is invalid
fn parse_mul(val: &str) -> Option<u32> {
    let vals = val.split(")").next()?;
    let mut split = vals.split(",");
    let l = str::parse::<u32>(split.next()?).ok()?;
    let r = str::parse::<u32>(split.next()?).ok()?;
    Some(l * r)
}
