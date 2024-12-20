use crate::shared::*;
use core::panic;
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

    fn sides(&self, garden: &Garden<N>) -> usize {
        // Calculate the next step from an edge
        enum NextStep {
            Continue,
            Clockwise,
            AntiClockwise,
        }

        // Requires a coord on the edge, and a direction of that edge
        fn next_step<const M: usize>(
            garden: &Garden<M>,
            inside: &Coord<M>,
            edge_dir: &Direction,
        ) -> NextStep {
            let region_char = *garden.get(inside);
            let next_side_coord = inside.next(side_direction(*edge_dir));
            if let Some(outside) = inside.next(*edge_dir) {
                // Not outside edge
                if let Some(next_side_coord) = next_side_coord {
                    // Random mid square, check values
                    let next_side = *garden.get(&next_side_coord);
                    let next_side_outside =
                        *garden.get(&outside.next(side_direction(*edge_dir)).unwrap());
                    match (next_side == region_char, next_side_outside == region_char) {
                        (true, false) => NextStep::Continue,
                        (true, true) => NextStep::AntiClockwise,
                        (_, _) => NextStep::Clockwise,
                    }
                } else {
                    // Hit an edge, turn
                    NextStep::Clockwise
                }
            } else {
                // Outside edge, just check if next cell is valid
                if let Some(next_side_coord) = next_side_coord {
                    if *garden.get(&next_side_coord) == region_char {
                        return NextStep::Continue;
                    }
                }
                NextStep::Clockwise
            }
        }

        // Function to find side dir given outside dir
        fn side_direction(dir: Direction) -> Direction {
            match dir {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                _ => panic!("Invalid outside direction, must be orthogonal"),
            }
        }

        // Track which edges has been explored
        let region_char = *garden.get(self.0.first().expect("Cannot find sides of empty region"));
        let mut unexplored = self
            .0
            .iter()
            .flat_map(|coord| {
                Direction::orthogonal().filter_map(|dir| match coord.next(dir) {
                    Some(adj) => {
                        if *garden.get(&adj) == region_char {
                            None
                        } else {
                            Some((*coord, dir))
                        }
                    }
                    None => Some((*coord, dir)),
                })
            })
            .collect::<Vec<_>>();

        let mut count = 0;
        while let Some((start, start_dir)) = unexplored.first() {
            let start = *start;
            let start_dir = *start_dir;

            // Starting values
            let mut curr = start;
            let mut outside_dir = start_dir;

            loop {
                // Note down that this is explored
                unexplored.retain(|x| *x != (curr, outside_dir));
                match next_step(garden, &curr, &outside_dir) {
                    NextStep::Continue => {
                        let side_dir = side_direction(outside_dir);
                        curr = curr.next(side_dir).unwrap();
                    }
                    NextStep::Clockwise => {
                        outside_dir = side_direction(outside_dir);
                        count += 1
                    }
                    NextStep::AntiClockwise => {
                        // Eww
                        let side_dir = side_direction(outside_dir);
                        curr = curr.next(side_dir).unwrap();
                        curr = curr.next(outside_dir).unwrap();
                        outside_dir = side_direction(outside_dir);
                        outside_dir = side_direction(outside_dir);
                        outside_dir = side_direction(outside_dir);
                        count += 1
                    }
                }

                if curr == start && outside_dir == start_dir {
                    break;
                }
            }
        }

        count
    }

    fn cost(&self, garden: &Garden<N>) -> usize {
        self.perimiter(garden) * self.area()
    }

    fn discount_cost(&self, garden: &Garden<N>) -> usize {
        self.sides(garden) * self.area()
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
    //const N: usize = 6;
    let garden = Garden::<N>::new(&input).expect("Unable to parse garden");
    let regions = garden.regions();
    let total: usize = regions.iter().map(|region| region.cost(&garden)).sum();
    println!("Total cost: {}", total);
    let total: usize = regions
        .iter()
        .map(|region| region.discount_cost(&garden))
        .sum();
    println!("Discounted cost is: {}", total);
}
