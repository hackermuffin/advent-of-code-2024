use std::fmt;
use std::num::TryFromIntError;
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

#[derive(Clone, Copy, Debug)]
pub struct Grid<T, const N: usize>([[T; N]; N]);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coord<const N: usize> {
    x: usize,
    y: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl<T: FromStr, const N: usize> Grid<T, N> {
    pub fn new(input: &str, deliminator: &str) -> Option<Self> {
        // Parse items to vec of vec
        let vec = parse::<T>(input.trim(), deliminator).ok()?;
        // Try to convert into fixed len array
        let arr = vec
            .into_iter()
            .map(|x| x.try_into())
            .collect::<Result<Vec<[T; N]>, _>>()
            .ok()?
            .try_into()
            .ok()?;

        Some(Grid(arr))
    }
}

impl<T: FromStr + Copy, const N: usize> Grid<T, N> {
    pub fn fill(val: T) -> Self {
        Grid([[val; N]; N])
    }
}

impl<T, const N: usize> Grid<T, N> {
    pub fn get(&self, coord: &Coord<N>) -> &T {
        &self.0[coord.y][coord.x]
    }

    pub fn set(&mut self, val: T, coord: Coord<N>) {
        self.0[coord.y][coord.x] = val;
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter().flat_map(|x| x.iter())
    }

    pub fn enumerate(&self) -> Vec<(Coord<N>, &T)> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(|(x, val)| (Coord::<N>::new(x, y).unwrap(), val))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }
}

impl<T: Eq, const N: usize> Grid<T, N> {
    pub fn find(&self, target: T) -> Vec<Coord<N>> {
        self.enumerate()
            .iter()
            .filter(|(_, val)| **val == target)
            .map(|(coord, _)| *coord)
            .collect()
    }
}

impl<const N: usize> Coord<N> {
    pub fn new(x: usize, y: usize) -> Option<Self> {
        if x < N && y < N {
            Some(Coord { x, y })
        } else {
            None
        }
    }

    pub fn to_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn next(&self, dir: Direction) -> Option<Coord<N>> {
        let (x, y) = (self.x as i64, self.y as i64);
        let (x, y) = match dir {
            Direction::Up => (x, y - 1),
            Direction::UpRight => (x + 1, y - 1),
            Direction::Right => (x + 1, y),
            Direction::DownRight => (x + 1, y + 1),
            Direction::Down => (x, y + 1),
            Direction::DownLeft => (x - 1, y + 1),
            Direction::Left => (x - 1, y),
            Direction::UpLeft => (x - 1, y - 1),
        };
        (x, y).try_into().ok()
    }
}

impl<const N: usize> std::ops::Add for Coord<N> {
    type Output = (i64, i64);
    fn add(self, rhs: Self) -> Self::Output {
        (self.x as i64 + rhs.x as i64, self.y as i64 + rhs.y as i64)
    }
}

impl<const N: usize> std::ops::Sub for Coord<N> {
    type Output = (i64, i64);
    fn sub(self, rhs: Self) -> Self::Output {
        (self.x as i64 - rhs.x as i64, self.y as i64 - rhs.y as i64)
    }
}

impl<const N: usize> std::convert::TryFrom<(i64, i64)> for Coord<N> {
    type Error = TryFromIntError;
    fn try_from((x, y): (i64, i64)) -> Result<Self, Self::Error> {
        let x: usize = x.try_into()?;
        let y: usize = y.try_into()?;

        match Coord::new(x, y) {
            Some(coord) => Ok(coord),
            None => Err(<u8 as TryFrom<u16>>::try_from(300).unwrap_err()),
        }
    }
}

impl Direction {
    pub fn orthogonal() -> impl Iterator<Item = Direction> {
        vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
        .into_iter()
    }
}

// Display functions
impl<T: fmt::Display, const N: usize> fmt::Display for Grid<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = self.0.iter().fold(String::new(), |curr, line| {
            let line = &line
                .iter()
                .fold(String::new(), |curr, elem| format!("{curr}{elem}"));
            curr + line + "\n"
        });
        write!(f, "{res}")
    }
}
impl<const N: usize> fmt::Display for Coord<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

pub struct Cache<K: std::cmp::Eq + std::hash::Hash, V>(std::collections::HashMap<K, V>);

impl<K: std::cmp::Eq + std::hash::Hash, V> Cache<K, V> {
    pub fn new() -> Self {
        Cache(std::collections::HashMap::new())
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.0.get(key)
    }
}

impl<K: std::cmp::Eq + std::hash::Hash + Clone, V: Clone> Cache<K, V> {
    pub fn get_or_set<F>(&mut self, key: K, func: F) -> V
    where
        F: FnOnce() -> V,
    {
        if let Some(value) = self.get(&key) {
            value.clone()
        } else {
            let value = func();
            self.0.insert(key.clone(), value);
            self.0.get(&key).unwrap().clone()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
}

impl<T: std::ops::Add<Output = T>> std::ops::Add for Vector<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: std::ops::Sub<Output = T>> std::ops::Sub for Vector<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: std::ops::Mul<u64, Output = T>> std::ops::Mul<u64> for Vector<T> {
    type Output = Self;
    fn mul(self, rhs: u64) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: std::ops::Div<Output = T>> std::ops::Div for Vector<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
