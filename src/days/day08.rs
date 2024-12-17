use crate::shared::*;

struct Map<const N: usize>(Grid<char, N>);

impl<const N: usize> std::ops::Deref for Map<N> {
    type Target = Grid<char, N>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> Map<N> {
    fn new(input: &str) -> Option<Self> {
        Some(Map(Grid::<char, N>::new(input, "")?))
    }

    fn unique(&self) -> std::collections::HashSet<char> {
        let mut set = std::collections::HashSet::new();
        self.iter().for_each(|x| {
            set.insert(*x);
        });
        set
    }
}

fn pairwise<const N: usize>(values: Vec<Coord<N>>) -> impl Iterator<Item = (Coord<N>, Coord<N>)> {
    values
        .clone()
        .into_iter()
        .flat_map(|x| values.clone().into_iter().map(move |y| (x, y)))
        .filter(|(x, y)| *x != *y)
        .collect::<Vec<_>>()
        .into_iter()
}

fn compute<const N: usize>(
    map: &Map<N>,
    pairfn: fn((Coord<N>, Coord<N>)) -> Vec<Coord<N>>,
) -> Vec<Coord<N>> {
    map.unique()
        .iter()
        .filter_map(|char| {
            if *char == '.' {
                return None;
            }

            let coords = map.find(*char);
            Some(pairwise(coords).map(pairfn))
        })
        .flatten()
        .flatten()
        .collect::<Vec<_>>()
}

fn pt1<const N: usize>((l, r): (Coord<N>, Coord<N>)) -> Vec<Coord<N>> {
    let (xdiff, ydiff) = r - l;
    let (lx, ly) = (l.x() as i64, l.y() as i64);
    let (rx, ry) = (r.x() as i64, r.y() as i64);

    let new1 = (lx - xdiff, ly - ydiff);
    let new2 = (rx + xdiff, ry + ydiff);
    let new1: Option<Coord<N>> = new1.try_into().ok();
    let new2: Option<Coord<N>> = new2.try_into().ok();

    vec![new1, new2].into_iter().flatten().collect::<Vec<_>>()
}

fn pt2<const N: usize>((l, r): (Coord<N>, Coord<N>)) -> Vec<Coord<N>> {
    let (xdiff, ydiff) = r - l;
    let (lx, ly) = (l.x() as i64, l.y() as i64);
    let (rx, ry) = (r.x() as i64, r.y() as i64);

    let mut res = Vec::new();

    let (mut x, mut y) = (lx, ly);
    while let Ok(coord) = Coord::try_from((x, y)) {
        res.push(coord);
        (x, y) = (x - xdiff, y - ydiff);
    }

    let (mut x, mut y) = (rx, ry);
    while let Ok(coord) = Coord::try_from((x, y)) {
        res.push(coord);
        (x, y) = (x + xdiff, y + ydiff);
    }

    res
}

pub fn run(input: String) {
    const N: usize = 50;
    let map: Map<N> = Map::new(&input).expect("Could not parse map");
    let locations = compute(&map, pt1);
    let mut dedup = std::collections::HashSet::new();
    locations.iter().for_each(|x| {
        dedup.insert(x);
    });

    println!("{}, {}", locations.len(), dedup.len());

    let locations = compute(&map, pt2);
    let mut dedup = std::collections::HashSet::new();
    locations.iter().for_each(|x| {
        dedup.insert(x);
    });

    println!("{}, {}", locations.len(), dedup.len());
}
