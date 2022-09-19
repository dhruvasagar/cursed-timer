/* ************************************************************************** */
/*                                                                            */
/*                                                            .               */
/*   scramble.rs                                             / \              */
/*                                                          /   \             */
/*   By: charles <charles.cabergs@gmail.com>               /o  o \            */
/*                                                        /  v    \           */
/*   Created: 2020/06/25 13:24:17 by charles             /    _    \          */
/*   Updated: 2020/06/25 15:29:01 by charles            '-----------'         */
/*                                                                            */
/* ************************************************************************** */

use std::fmt;
use std::str;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Clone)]
pub struct Scramble(Vec<Move>);

impl Scramble {
    pub fn new_rand(n: usize) -> Scramble {
        let mut sequence: Vec<Move> = Vec::with_capacity(n);

        while sequence.len() != n {
            let direction = rand::random::<Direction>();
            let modifier = rand::random::<Modifier>();

            if let Some(l) = sequence.last() {
                if l.direction == direction {
                    continue;
                }
            }
            sequence.push(Move {
                direction,
                modifier,
            });
        }
        Scramble(sequence)
    }
}

impl fmt::Display for Scramble {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|m| m.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl str::FromStr for Scramble {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let strs = s.split(" ");
        let mut scramble = Scramble(Vec::new());
        for s in strs {
            scramble.0.push(s.parse()?);
        }
        Ok(scramble)
    }
}

// impl Clone for Scramble {
//     fn clone(&self) -> Scramble {
//         let v = self.0;
//         Scramble(v)
//     }
// }

impl str::FromStr for Move {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        use Modifier::*;

        let mut cs = s.chars();
        let direction = match cs.next() {
            Some('F') => Front,
            Some('B') => Back,
            Some('D') => Down,
            Some('U') => Up,
            Some('R') => Right,
            Some('L') => Left,
            Some(_) => return Err("Move direction isn't valid"),
            None => return Err("Move format is empty"),
        };
        let modifier = match cs.next() {
            Some('\'') => Prime,
            Some('2') => Twice,
            Some(_) => return Err("Move modifier isn't valid"),
            None => No,
        };
        if let Some(_) = cs.next() {
            return Err("Unexpected character in move");
        }
        Ok(Move {
            direction,
            modifier,
        })
    }
}

#[derive(PartialEq, Clone)]
enum Direction {
    Front,
    Back,
    Down,
    Up,
    Right,
    Left,
}

#[derive(Clone)]
enum Modifier {
    No,
    Twice,
    Prime,
}

#[derive(Clone)]
struct Move {
    direction: Direction,
    modifier: Modifier,
}

// https://stackoverflow.com/questions/48490049
impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        use Direction::*;
        match rng.gen_range(0..6) {
            0 => Front,
            1 => Back,
            2 => Down,
            3 => Up,
            4 => Right,
            _ => Left,
        }
    }
}

impl Distribution<Modifier> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Modifier {
        use Modifier::*;
        match rng.gen_range(0..3) {
            0 => No,
            1 => Twice,
            _ => Prime,
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Direction::*;
        use Modifier::*;
        let letter = match self.direction {
            Front => "F",
            Back => "B",
            Down => "D",
            Up => "U",
            Right => "R",
            Left => "L",
        };
        let modifier = match self.modifier {
            No => "",
            Twice => "2",
            Prime => "'",
        };
        write!(f, "{}{}", letter, modifier)
    }
}
