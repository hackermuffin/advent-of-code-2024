use std::fmt;

#[derive(Debug, Clone)]
struct Equation {
    target: u64,
    values: Vec<u32>,
}

impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let values = self
            .values
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
            .to_string();
        write!(f, "{}: {}", self.target, values)
    }
}

impl Equation {
    fn new(input: &str) -> Option<Self> {
        let mut split = input.split(": ");
        let target = str::parse(split.next()?).ok()?;
        let values = split
            .next()?
            .split(" ")
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .ok()?;
        Some(Equation { target, values })
    }

    fn solve(self, ops: &[Operator]) -> Option<Solution> {
        fn helper(partial: &mut PartialSolution, ops: &[Operator]) -> Option<Solution> {
            if partial.values.len() - 1 == partial.ops.len() {
                return partial.check();
            }

            for op in ops.into_iter() {
                partial.push(*op);
                if let Some(sol) = helper(partial, ops) {
                    return Some(sol);
                };
                partial.pop();
            }

            None
        }

        let mut partial = PartialSolution::new(self);
        helper(&mut partial, ops)
    }
}

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Concat,
    Mul,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Add => "+",
                Self::Concat => "||",
                Self::Mul => "*",
            }
        )
    }
}

impl Operator {
    fn get_fn<
        A: std::ops::Add<Output = C>
            + std::ops::Mul<Output = C>
            + std::convert::From<B>
            + fmt::Display,
        B: fmt::Display,
        C: std::str::FromStr,
    >(
        &self,
    ) -> impl Fn(A, B) -> C
    where
        <C as std::str::FromStr>::Err: fmt::Debug,
    {
        match self {
            Self::Add => |l, r| l + B::into(r),
            Self::Concat => |l: A, r: B| str::parse::<C>(&format!("{l}{r}")).unwrap(),
            Self::Mul => |l, r| l * B::into(r),
        }
    }

    fn get_undo<
        A: std::ops::Sub<Output = C>
            + std::ops::Div<Output = C>
            + std::convert::From<B>
            + fmt::Display,
        B: fmt::Display,
        C: std::str::FromStr,
    >(
        &self,
    ) -> impl Fn(A, B) -> C
    where
        <C as std::str::FromStr>::Err: fmt::Debug,
    {
        match self {
            Self::Add => |l, r| l - B::into(r),
            Self::Mul => |l, r| l / B::into(r),
            Self::Concat => |l, r| {
                let rlen = format!("{r}").len();
                let l = format!("{l}");
                let res = &l[..l.len() - rlen];
                str::parse(res).expect("Unable to undo concat")
            },
        }
    }
}

#[derive(Debug)]
struct PartialSolution {
    target: u64,
    curr: u64,
    values: Vec<u32>,
    ops: Vec<Operator>,
}

impl PartialSolution {
    fn new(eq: Equation) -> Self {
        Self {
            target: eq.target,
            curr: *eq
                .values
                .first()
                .expect("Cannot setup solution for empty equation") as u64,
            values: eq.values,
            ops: Vec::new(),
        }
    }

    fn push(&mut self, op: Operator) {
        let next_val = *self
            .values
            .get(self.ops.len() + 1)
            .expect("Insufficient values to push operator");
        self.curr = op.get_fn()(self.curr, next_val);
        self.ops.push(op)
    }

    fn pop(&mut self) {
        let val = *self
            .values
            .get(self.ops.len())
            .expect("Insufficient values to remove");
        self.curr = self
            .ops
            .pop()
            .expect("Insufficient ops to remove")
            .get_undo()(self.curr, val);
    }

    fn check(&self) -> Option<Solution> {
        if self.curr == self.target {
            // TODO check if lengths match up
            Some(Solution {
                target: self.target,
                values: self.values.clone(),
                ops: self.ops.clone(),
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Solution {
    target: u64,
    values: Vec<u32>,
    ops: Vec<Operator>,
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut values = self.values.iter().map(|x| x.to_string());
        let ops = self.ops.iter().map(|x| x.to_string());
        let mut solution = values.next().unwrap_or_default();
        ops.zip(values).for_each(|(op, val)| {
            let res = format!(" {op} {val}");
            solution.push_str(&res);
        });

        write!(f, "{} = {}", self.target, solution)
    }
}

impl Solution {
    //fn new(eq: &Equation, ops: &Vec<Operator>) -> Option<Self> {
    //    let mut eq = eq.clone();
    //    let mut ops = ops.clone();
    //    let mut acc = eq.values.pop()? as u64;

    //    while let Some(val) = eq.values.pop() {
    //        let op = ops.pop()?;
    //    }

    //    if acc == eq.target {
    //        Some(Solution {
    //            target: eq.target,
    //            values: eq.values,
    //            ops: ops.clone(),
    //        })
    //    } else {
    //        None
    //    }
    //}
}

pub fn run(input: String) {
    let equations = input
        .trim()
        .split("\n")
        .map(Equation::new)
        .collect::<Option<Vec<_>>>()
        .expect("Unable to parse input");

    let solutions = equations
        .clone()
        .into_iter()
        .map(|eq| eq.solve(&[Operator::Add, Operator::Mul]))
        .filter(|x| x.is_some())
        .collect::<Option<Vec<_>>>()
        .unwrap();

    let solutions_concat = equations
        .into_iter()
        .map(|eq| eq.solve(&[Operator::Add, Operator::Concat, Operator::Mul]))
        .filter(|x| x.is_some())
        .collect::<Option<Vec<_>>>()
        .unwrap();

    let target_total: u64 = solutions.iter().map(|x| x.target).sum();
    let target_total_contcat: u64 = solutions_concat.iter().map(|x| x.target).sum();

    println!("Total valid targets: {target_total}");
    println!("Total valid targets with concat: {target_total_contcat}");
}
