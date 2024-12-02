use std::str::FromStr;

pub fn parse<T: FromStr>(input: &str) -> Result<Vec<Vec<T>>, <T as FromStr>::Err> {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .filter(|x| x != &"")
                .map(|x| str::parse::<T>(x))
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()
}
