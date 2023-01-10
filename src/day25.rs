use std::iter::Sum;
use std::{str::FromStr, ops::Add};

use itertools::Itertools;
use itertools::EitherOrBoth;


#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Digit {
    One,
    Two,
    Zero,
    Minus,
    DoubleMinus,
}

impl Add for Digit {
    type Output = (Digit, Digit);

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // adding zero does nothing
            (Digit::Zero, d) | (d, Digit::Zero) => (d, Digit::Zero),
            // adding these to each other cancels
            (Digit::One, Digit::Minus) | (Digit::Minus, Digit::One) => (Digit::Zero, Digit::Zero),
            (Digit::Two, Digit::DoubleMinus) | (Digit::DoubleMinus, Digit::Two) => (Digit::Zero, Digit::Zero),
            // these part-cancel
            (Digit::One, Digit::DoubleMinus) | (Digit::DoubleMinus, Digit::One) => (Digit::Minus, Digit::Zero),
            (Digit::Two, Digit::Minus) | (Digit::Minus, Digit::Two) => (Digit::One, Digit::Zero),
            // these combine but don't overflow
            (Digit::One, Digit::One) => (Digit::Two, Digit::Zero),
            (Digit::Minus, Digit::Minus) => (Digit::DoubleMinus, Digit::Zero),
            // these overflow and give a carry
            (Digit::One, Digit::Two) | (Digit::Two, Digit::One) => (Digit::DoubleMinus, Digit::One),
            (Digit::Minus, Digit::DoubleMinus) | (Digit::DoubleMinus, Digit::Minus) => (Digit::Two, Digit::Minus),
            (Digit::Two, Digit::Two) => (Digit::Minus, Digit::One),
            (Digit::DoubleMinus, Digit::DoubleMinus) => (Digit::One, Digit::Minus),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SnafuNumber {
    // stored in units-first order
    digits: Vec<Digit>,
}

impl Add for SnafuNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut digits = Vec::new();
        let mut previous_carry = Digit::Zero;
        for pair in self.digits.iter().zip_longest(rhs.digits) {
            let (result, carry) = match pair {
                EitherOrBoth::Both(lhs, rhs) => *lhs + rhs,
                EitherOrBoth::Left(lhs) => (*lhs, Digit::Zero),
                EitherOrBoth::Right(rhs) => (rhs, Digit::Zero),
            };
            let (adjusted_from_carry, extra) = result + previous_carry;
            let (new_carry, nothing) = carry + extra;
            // this should have given us no excess carry
            assert!(nothing == Digit::Zero);
            digits.push(adjusted_from_carry);
            previous_carry = new_carry;
        }

        // now all the digits have been added - don't forget the final carry
        if previous_carry != Digit::Zero {
            digits.push(previous_carry);
        }

        Self::Output { digits }
    }
}

impl Sum for SnafuNumber {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut result = Self { digits: vec![Digit::Zero] };
        for n in iter {
            result = result + n;
        }
        result
    }
}

impl std::fmt::Display for SnafuNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for d in self.digits.iter().rev() {
            match d {
                Digit::One => write!(f, "1")?,
                Digit::Two => write!(f, "2")?,
                Digit::Zero => write!(f, "0")?,
                Digit::Minus => write!(f, "-")?,
                Digit::DoubleMinus => write!(f, "=")?,
            }
        }

        Ok(())
    }
}

impl FromStr for SnafuNumber {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = Vec::new();
        for c in s.chars().rev() {
            digits.push(match c {
                '2' => Digit::Two,
                '1' => Digit::One,
                '0' => Digit::Zero,
                '-' => Digit::Minus,
                '=' => Digit::DoubleMinus,
                _ => unreachable!()
            });
        }
        Ok(Self {digits })
    }
}

pub struct Input {}

#[aoc_generator(day25)]
pub fn input_generator_part1(input: &str) -> Vec<SnafuNumber> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}


#[aoc(day25, part1)]
pub fn solve_part1(input: &Vec<SnafuNumber>) -> String {
    input.iter().cloned().sum::<SnafuNumber>().to_string()
}


#[aoc(day25, part2)]
pub fn solve_part2(_input: &Vec<SnafuNumber>) -> usize {
    0
}

#[test]
fn test_day25_input1() {
    let input =
r#"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
"#;

    let parsed_input = input_generator_part1(input);
    let part1_result = solve_part1(&parsed_input);
    let part2_result = solve_part2(&parsed_input);

    assert_eq!(part1_result, "2=-1=0".to_owned());
    assert_eq!(part2_result, 0);
}

#[test]
fn test_day25_adding() {
    let nine: SnafuNumber = "2-".parse().unwrap();
    let two_hundred_one: SnafuNumber = "2=01".parse().unwrap();
    let nine_hundred_six: SnafuNumber = "12111".parse().unwrap();

    let result = nine_hundred_six.clone() + nine_hundred_six + two_hundred_one + nine;

    assert_eq!(result.to_string(), "1=11-2");
}