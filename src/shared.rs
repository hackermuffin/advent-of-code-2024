use std::str::FromStr;

pub fn parse<T: FromStr>(
    input: &str,
    deliminator: &str,
) -> Result<Vec<Vec<T>>, <T as FromStr>::Err> {
    input
        .lines()
        .map(|line| {
            line.split(deliminator)
                .filter(|x| x != &"")
                .map(|x| str::parse::<T>(x))
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()
}
