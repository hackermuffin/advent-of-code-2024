use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Char {
    X,
    M,
    A,
    S,
}

impl Char {
    fn to_char(self) -> char {
        match self {
            Self::X => 'X',
            Self::M => 'M',
            Self::A => 'A',
            Self::S => 'S',
        }
    }
}

impl fmt::Display for Char {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Char::X => 'X',
            Char::M => 'M',
            Char::A => 'A',
            Char::S => 'S',
        };
        write!(f, "{output}")
    }
}

//impl fmt::Display for Option<Char> {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        let output = match self {
//            Some(c) => c.to_string(),
//            None => ".".to_string(),
//        };
//        write!(f, "{output}")
//    }
//}

#[derive(Debug, Clone, Copy)]
enum Direction {
    UpLeft,
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
}

impl Direction {
    fn iter() -> impl Iterator<Item = Self> {
        [
            Self::UpLeft,
            Self::Up,
            Self::UpRight,
            Self::Right,
            Self::DownRight,
            Self::Down,
            Self::DownLeft,
            Self::Left,
        ]
        .iter()
        .copied()
    }
}

#[derive(Debug, Copy, Clone)]
struct Coord<const N: usize> {
    x: usize,
    y: usize,
}

impl<const N: usize> Coord<N> {
    fn new(x: i32, y: i32) -> Option<Coord<N>> {
        if x >= 0 && y >= 0 && x < N as i32 && y < N as i32 {
            let x = x as usize;
            let y = y as usize;
            Some(Coord { x, y })
        } else {
            None
        }
    }

    fn adj(&self, dir: &Direction) -> Option<Coord<N>> {
        let (x, y) = (self.x as i32, self.y as i32);
        let (new_x, new_y) = match dir {
            Direction::UpLeft => (x - 1, y - 1),
            Direction::Up => (x, y - 1),
            Direction::UpRight => (x + 1, y - 1),
            Direction::Right => (x + 1, y),
            Direction::DownRight => (x + 1, y + 1),
            Direction::Down => (x, y + 1),
            Direction::DownLeft => (x - 1, y + 1),
            Direction::Left => (x - 1, y),
        };
        Coord::new(new_x, new_y)
    }
}

struct WordSearch<const N: usize> {
    data: [[Option<Char>; N]; N], //data: Vec<Vec<Option<Char>>>,
}

impl<const N: usize> WordSearch<N> {
    fn new(input: &str) -> Option<WordSearch<N>> {
        fn parse_char(c: char) -> Option<Char> {
            match c {
                'X' => Some(Char::X),
                'M' => Some(Char::M),
                'A' => Some(Char::A),
                'S' => Some(Char::S),
                _ => None,
            }
        }

        fn parse_line<const M: usize>(line: &str) -> Option<[Option<Char>; M]> {
            line.chars()
                .map(parse_char)
                .collect::<Vec<_>>()
                .try_into()
                .ok()
        }

        let lines = input.split("\n");
        let data = lines
            .map(parse_line::<N>)
            .collect::<Option<Vec<_>>>()?
            .try_into()
            .ok()?;

        Some(WordSearch { data })
    }

    fn get(&self, coord: Coord<N>) -> &Option<Char> {
        &self.data[coord.y][coord.x]
    }

    fn get_word<const LEN: usize>(&self, coord: &Coord<N>, dir: Direction) -> Option<[char; LEN]> {
        // Generate coords list
        let mut coords = vec![*coord];
        for _ in 1..LEN {
            coords.push(coords.last()?.adj(&dir)?)
        }
        let coords: [_; LEN] = coords.try_into().ok()?;

        // Get chars
        let chars = coords
            .map(|x| *self.get(x))
            .into_iter()
            .collect::<Option<Vec<_>>>()?
            .iter()
            .map(|x| x.to_char())
            .collect::<Vec<_>>()
            .try_into()
            .ok();
        chars
    }

    fn is_cross_mas(&self, coord: &Coord<N>) -> bool {
        const TARGET: [char; 3] = ['M', 'A', 'S'];
        const TARGET_REV: [char; TARGET.len()] = ['S', 'A', 'M'];
        const LEN: usize = TARGET.len();

        let Some(up_left) = coord.adj(&Direction::UpLeft) else {
            return false;
        };
        let Some(up_right) = coord.adj(&Direction::UpRight) else {
            return false;
        };

        let Some(word1) = self.get_word::<LEN>(&up_left, Direction::DownRight) else {
            return false;
        };
        let Some(word2) = self.get_word::<LEN>(&up_right, Direction::DownLeft) else {
            return false;
        };

        (word1 == TARGET || word1 == TARGET_REV) && (word2 == TARGET || word2 == TARGET_REV)
    }
}

impl<const N: usize> fmt::Display for WordSearch<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = self
            .data
            .iter()
            .map(|line| {
                line.iter()
                    .map(|char| match char {
                        Some(c) => c.to_string(),
                        None => ".".to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{output}")
    }
}

pub fn run(input: String) {
    const DIM: usize = 140;
    //const DIM: usize = 10;

    let ws = WordSearch::<DIM>::new(input.trim())
        .expect(&format!("Failed to parse {DIM}x{DIM} wordsearch")[..]);

    let mut xmas_count = 0;
    let mut cross_mas_count = 0;
    for y in 0..DIM {
        for x in 0..DIM {
            let coord = Coord::new(x as i32, y as i32).unwrap();
            for dir in Direction::iter() {
                if ws.get_word::<4>(&coord, dir) == Some(['X', 'M', 'A', 'S']) {
                    xmas_count += 1
                }
            }
            if ws.is_cross_mas(&coord) {
                cross_mas_count += 1;
            }
        }
    }

    println!("XMAS count: {xmas_count}");
    println!("X-MAS count: {cross_mas_count}");
}
