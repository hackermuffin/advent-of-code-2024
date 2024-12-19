use crate::shared::*;
use std::fmt;

struct Garden<const N: usize>(Grid<char, N>);

impl<const N: usize> std::ops::Deref for Garden<N> {
    type Target = Grid<char, N>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> Garden<N> {
    fn new(input: &str) -> Option<Self> {
        Some(Garden(Grid::new(input.trim(), "")?))
    }

    fn connected(&self, start: Coord<N>) -> Region<N> {
        fn explore<const M: usize>(garden: &Garden<M>, curr: &mut Vec<Coord<M>>) {
            let top = *curr.last().expect("Unable to explore empty curr vec");
            for dir in Direction::orthogonal() {
                if let Some(next) = top.next(dir) {
                    if garden.get(&top) == garden.get(&next) && !curr.contains(&next) {
                        curr.push(next);
                        explore(garden, curr);
                    }
                }
            }
        }

        let mut coords = vec![start];
        explore(self, &mut coords);
        Region(coords)
    }

    fn regions(&self) -> Vec<Region<N>> {
        let mut grid = Grid::<_, N>::fill(false);
        let mut regions = Vec::new();

        loop {
            if let Some((coord, _)) = grid.enumerate().iter().find(|(_, x)| !**x) {
                let region = self.connected(*coord);
                region.0.iter().for_each(|coord| {
                    grid.set(true, *coord);
                });
                regions.push(region);
            } else {
                return regions;
            }
        }
    }
}

struct Region<const N: usize>(Vec<Coord<N>>);

impl<const N: usize> Region<N> {
    fn perimiter(&self, garden: &Garden<N>) -> usize {
        let region = &self.0;
        region
            .iter()
            .map(|coord| {
                let target_char = garden.get(
                    region
                        .first()
                        .expect("Cannot find perimiter of empty region"),
                );
                Direction::orthogonal()
                    .filter(|dir| match coord.next(*dir) {
                        Some(coord) => *garden.get(&coord) != *target_char,
                        None => true,
                    })
                    .count()
            })
            .sum()
    }

    fn area(&self) -> usize {
        self.0.len()
    }

    fn cost(&self, garden: &Garden<N>) -> usize {
        self.perimiter(garden) * self.area()
    }
}

impl<const N: usize> fmt::Display for Region<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut grid = Grid::<_, N>::fill('.');
        self.0.iter().for_each(|coord| grid.set('X', *coord));
        write!(f, "{}", grid)
    }
}

pub fn run(input: String) {
    const N: usize = 140;
    let garden = Garden::<N>::new(&input).expect("Unable to parse garden");
    let regions = garden.regions();
    let total: usize = regions.iter().map(|region| region.cost(&garden)).sum();
    println!("Total cost: {}", total);
}
