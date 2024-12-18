use std::ops::Deref;

use crate::shared::*;

struct Map<const N: usize>(Grid<u8, N>);

impl<const N: usize> Deref for Map<N> {
    type Target = Grid<u8, N>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> Map<N> {
    fn new(input: &str) -> Option<Self> {
        Some(Map(Grid::new(input.trim(), "")?))
    }

    fn paths(&self, start: Coord<N>) -> Vec<Vec<Coord<N>>> {
        fn explore<const M: usize>(
            map: &Map<M>,
            curr: &mut Vec<Coord<M>>,
            complete: &mut Vec<Vec<Coord<M>>>,
        ) {
            //println!("{curr:?}");
            let curr_coord = *curr.last().expect("Current cells empty");
            if *map.get(&curr_coord) == 9 {
                // Route complete, add to finished vector
                complete.push(curr.clone());
            } else {
                for dir in Direction::orthogonal() {
                    //println!("Trying {dir:?}");
                    let next_coord = (curr_coord).next(dir);
                    if let Some(next_coord) = next_coord {
                        //println!(
                        //    "Trying coord: {next_coord} with val {}",
                        //    map.get(&next_coord)
                        //);
                        if map.get(&curr_coord) + 1 == *map.get(&next_coord) {
                            // Is a valid next coord
                            curr.push(next_coord);
                            explore(map, curr, complete);
                            curr.pop();
                        }
                    }
                }
            }
        }

        let mut curr = vec![start];
        let mut res = Vec::new();
        explore(self, &mut curr, &mut res);

        res
    }

    fn score(&self, start: Coord<N>) -> usize {
        let mut set = std::collections::HashSet::new();
        self.paths(start)
            .iter()
            .map(|x| x.last().unwrap())
            .for_each(|x| {
                set.insert(*x);
            });
        set.len()
    }

    fn total_score(&self) -> usize {
        self.find(0).iter().map(|x| self.score(*x)).sum()
    }
}

pub fn run(input: String) {
    const N: usize = 57;
    let map = Map::<N>::new(&input).expect("Unable to parse map");
    println!("Total score: {}", map.total_score());
}
