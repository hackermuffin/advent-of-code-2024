use core::panic;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl Facing {
    fn next(&self) -> Self {
        match self {
            Facing::Up => Facing::Right,
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
        }
    }

    fn index(&self) -> usize {
        match self {
            Self::Up => 0,
            Self::Right => 1,
            Self::Down => 2,
            Self::Left => 3,
        }
    }

    fn new(val: Self) -> [bool; 4] {
        let mut arr = [false; 4];
        arr[val.index()] = true;
        return arr;
    }

    fn add(curr: &mut [bool; 4], val: Self) {
        curr[val.index()] = true;
    }

    fn get(curr: &mut [bool; 4], val: Self) -> bool {
        curr[val.index()]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Visited([bool; 4]),
    Obstruction,
    Guard(Facing, [bool; 4]),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Empty => '.',
            Self::Visited(_) => 'X',
            Self::Obstruction => '#',
            Self::Guard(Facing::Up, _) => '^',
            Self::Guard(Facing::Down, _) => 'v',
            Self::Guard(Facing::Left, _) => '<',
            Self::Guard(Facing::Right, _) => '>',
        };
        write!(f, "{}", c)
    }
}

impl Cell {
    fn new(c: char) -> Option<Self> {
        match c {
            '.' => Some(Self::Empty),
            '#' => Some(Self::Obstruction),
            '^' => Some(Self::new_guard(Facing::Up)),
            '>' => Some(Self::new_guard(Facing::Right)),
            'v' => Some(Self::new_guard(Facing::Down)),
            '<' => Some(Self::new_guard(Facing::Left)),
            _ => None,
        }
    }

    fn new_guard(dir: Facing) -> Self {
        Cell::Guard(dir, Facing::new(dir))
    }

    fn is_guard(&self) -> bool {
        match self {
            Self::Guard(_, _) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Map<const N: usize> {
    data: [[Cell; N]; N],
    guard: Option<Coord>,
}

impl<const N: usize> fmt::Display for Map<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let grid = self
            .data
            .map(|line| line.map(|char| char.to_string()).join(""))
            .join("\n");
        let guard = match self.guard {
            Some(guard) => format!("Guard at ({},{})", guard.x, guard.y),
            None => "Guard not present".to_string(),
        };

        write!(f, "{grid}\n{guard}")
    }
}

impl<const N: usize> Map<N> {
    fn new(input: &str) -> Option<Self> {
        let data: [[Cell; N]; N] = input
            .trim()
            .split("\n")
            .map(|line| {
                line.chars()
                    .map(Cell::new)
                    .collect::<Option<Vec<_>>>()?
                    .try_into()
                    .ok()
            })
            .collect::<Option<Vec<[Cell; N]>>>()?
            .try_into()
            .ok()?;

        let (y, Some(x)) = data
            .map(|row| -> Option<usize> {
                row.iter()
                    .enumerate()
                    .find(|(_, cell)| cell.is_guard())
                    .map(|(i, _)| i)
            })
            .into_iter()
            .enumerate()
            .find(|(_, x)| x.is_some())?
        else {
            return None;
        };

        Some(Map {
            data,
            guard: Some(Coord { x, y }),
        })
    }

    fn get(&self, coord: Coord) -> &Cell {
        &self.data[coord.y][coord.x]
    }

    fn get_mut(&mut self, coord: Coord) -> &mut Cell {
        &mut self.data[coord.y][coord.x]
    }

    fn set(&mut self, coord: Coord, state: Cell) {
        self.data[coord.y][coord.x] = state
    }

    // Tests if the given state is a looping state
    fn is_loop(&mut self) -> bool {
        //println!("Testing loop");
        while self.guard.is_some() {
            //println!("{}", self.guard.unwrap());
            let repeat = self.next();
            if repeat {
                println!("{self}");
                return true;
            }
        }

        false
    }

    fn next_coord(&self) -> Option<(Coord, Facing)> {
        // Get current guard info
        let guard_coord = self.guard?;
        let guard = self.get(guard_coord);
        let Cell::Guard(facing, _) = guard else {
            panic!("Guard not found at location.")
        };
        let facing = *facing;

        // Calculate possible next coords
        let (x, y) = (guard_coord.x as i32, guard_coord.y as i32);
        let (x, y) = match facing {
            Facing::Up => (x, y - 1),
            Facing::Left => (x - 1, y),
            Facing::Down => (x, y + 1),
            Facing::Right => (x + 1, y),
        };

        // Determine next guard state
        if x >= 0 && y >= 0 && x < N as i32 && y < N as i32 {
            // Coord in bounds, guard has next valid coord
            let next_coord = Coord {
                x: x as usize,
                y: y as usize,
            };

            match self.get(next_coord) {
                Cell::Obstruction => {
                    // Guard rotates on the same spot
                    Some((guard_coord, facing.next()))
                }
                _ => {
                    // Move to next cell
                    Some((next_coord, facing))
                }
            }
        } else {
            // Guard is moving out of bounds
            None
        }
    }

    // Generates next state, returning a bool of if the guard has been in that location before
    fn next(&mut self) -> bool {
        // Get current & next status
        let Some(current_loc) = self.guard else {
            return false;
        };
        let Cell::Guard(current_facing, _) = self.get(current_loc) else {
            return false;
        };
        let current_facing = *current_facing;
        let Some((next_loc, next_facing)) = self.next_coord() else {
            self.set(current_loc, Cell::Visited(Facing::new(current_facing)));
            self.guard = None;
            return false;
        };

        // Check if cell is repeating
        let target = self.get_mut(next_loc);
        let repeat;
        if let Cell::Visited(dirs) = target {
            repeat = Facing::get(dirs, next_facing)
        } else {
            repeat = false
        }

        // Overwrite current cell
        let Cell::Guard(_, current_history) = self.get(current_loc) else {
            return false;
        };
        let current_new_state = Cell::Visited(*current_history);
        self.set(current_loc, current_new_state);

        // Generate new guard cell
        let next = self.get_mut(next_loc);
        let next_new = match next {
            Cell::Empty => Cell::new_guard(next_facing),
            Cell::Visited(dirs) => {
                Facing::add(dirs, next_facing);
                Cell::Guard(next_facing, *dirs)
            }
            _ => panic!["Guard moving to invalid location"],
        };
        self.set(next_loc, next_new);
        self.guard = Some(next_loc);

        repeat
    }

    fn place_obstacle(&mut self) -> Option<()> {
        let (next_coord, _) = self.next_coord()?;
        if self.guard? != next_coord {
            self.set(next_coord, Cell::Obstruction);
            Some(())
        } else {
            println!("Not placing obstruction at {next_coord}, guard currently there");
            None
        }
    }
}

pub fn run(input: String) {
    let mut map: Map<130> = Map::new(&input).expect("Could not parse map");
    //let mut map: Map<10> = Map::new(&input).expect("Could not parse map");

    let mut locs = Vec::new();
    while let Some(loc) = map.guard {
        locs.push(loc);
        map.next();
    }

    let count = map
        .data
        .concat()
        .iter()
        .filter(|x| matches!(x, Cell::Visited(_)))
        .count();
    println!("Visited: {}", count);

    let mut map: Map<130> = Map::new(&input).expect("Could not parse map");
    //let mut map: Map<10> = Map::new(&input).expect("Could not parse map");
    println!("{map}");
    let mut loops = 0;
    while map.guard.is_some() {
        let mut test_map = map;
        test_map.place_obstacle();
        if test_map.is_loop() {
            loops += 1;
        };
        map.next();
    }

    println!("Loops: {loops}");
}
