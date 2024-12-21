#[derive(Debug, Clone, Copy)]
struct Vector(crate::shared::Vector<u64>);

impl std::ops::Deref for Vector {
    type Target = crate::shared::Vector<u64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Vector {
    fn new(input: &str) -> Option<Self> {
        let (x_str, y_str) = input.split_once(", ")?;
        let x = str::parse(x_str.trim_start_matches(|c| !char::is_ascii_digit(&c))).ok()?;
        let y = str::parse(y_str.trim_start_matches(|c| !char::is_ascii_digit(&c))).ok()?;

        Some(Vector(crate::shared::Vector { x, y }))
    }

    fn signed(&self) -> (i64, i64) {
        (self.x as i64, self.y as i64)
    }
}

#[derive(Debug)]
struct Claw {
    a: Vector,
    b: Vector,
    prize: Vector,
}

impl Claw {
    fn new(input: &str) -> Option<Self> {
        let mut split = input.split("\n");
        let a = Vector::new(split.next()?.trim_start_matches("Button A: "))?;
        let b = Vector::new(split.next()?.trim_start_matches("Button B: "))?;
        let prize = Vector::new(split.next()?.trim_start_matches("Prize: "))?;

        Some(Claw { a, b, prize })
    }

    fn solve(&self) -> Option<(u64, u64)> {
        let b_mul = self.solve_b_coeff()?;
        let a_mul = (self.prize.x - self.b.x * b_mul) / self.a.x;
        let target = *self.a * a_mul + *self.b * b_mul;

        if target.y == self.prize.y {
            Some((a_mul, b_mul))
        } else {
            None
        }
    }

    fn cost(&self) -> Option<u64> {
        let (a, b) = self.solve()?;
        Some(a * 3 + b)
    }

    fn solve_b_coeff(&self) -> Option<u64> {
        let (a_x, a_y) = self.a.signed();
        let (b_x, b_y) = self.b.signed();
        let (t_x, t_y) = self.prize.signed();
        let top = a_x * t_y - a_y * t_x;
        let bottom = a_x * b_y - a_y * b_x;

        if top % bottom == 0 {
            Some((top / bottom) as u64)
        } else {
            None
        }
    }

    fn solve_hard(&self) -> Option<(u64, u64)> {
        let long = Claw {
            a: self.a,
            b: self.b,
            prize: Vector(
                *self.prize
                    + crate::shared::Vector {
                        x: 10000000000000,
                        y: 10000000000000,
                    },
            ),
        };

        long.solve()
    }

    fn cost_hard(&self) -> Option<u64> {
        let (a, b) = self.solve_hard()?;
        Some(a * 3 + b)
    }
}

fn parse(input: &str) -> Option<Vec<Claw>> {
    input.trim().split("\n\n").map(Claw::new).collect()
}

pub fn run(input: String) {
    let claws = parse(&input).expect("Unable to parse claws");

    let cost: u64 = claws.iter().filter_map(|claw| claw.cost()).sum();
    println!("Total cost: {cost}");

    let cost: u64 = claws.iter().filter_map(|claw| claw.cost_hard()).sum();
    println!("Total cost of far locations: {cost}");
}
