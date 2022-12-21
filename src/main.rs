use std::{fmt, ops};

mod dice;
mod strategy;

use clap::Parser;
use rand::prelude::*;
use squirrel_rng::SquirrelRng;

use crate::dice::Dice;

#[derive(Debug, Parser)]
struct Args {
    /// print average results of selected strategy over n iterations
    average: Option<usize>,

    #[command(subcommand)]
    command: Option<Command>,
}

impl Args {
    fn strategy(&self) -> &Command {
        self.command.as_ref().unwrap_or(&Command::Traditional)
    }
}

#[derive(Debug, Parser)]
enum Command {
    /// 4d6 drop lowest
    Traditional,

    /// 4d6 drop lowest + drop lowest stat
    DropTwice,
}

impl Command {
    fn get_provider<'a>(&self, rng: &'a mut impl Rng) -> Box<dyn FnMut() -> Outcome + 'a> {
        let mut provider = rng.roll_d6();
        match self {
            Command::Traditional => Box::new(move || strategy::traditional(&mut provider)),
            Command::DropTwice => Box::new(move || strategy::drop_twice(&mut provider)),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct Accumulator {
    count: usize,
    values: [f64; 6],
}

impl fmt::Display for Accumulator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [n1, n2, n3, n4, n5, n6] = self.values;
        let n1 = n1 / self.count as f64;
        let n2 = n2 / self.count as f64;
        let n3 = n3 / self.count as f64;
        let n4 = n4 / self.count as f64;
        let n5 = n5 / self.count as f64;
        let n6 = n6 / self.count as f64;
        write!(
            f,
            "{n1:.02}, {n2:.02}, {n3:.02}, {n4:.02}, {n5:.02}, {n6:.02}"
        )
    }
}

impl ops::AddAssign<Outcome> for Accumulator {
    fn add_assign(&mut self, rhs: Outcome) {
        let cells = self.values.iter_mut().zip(rhs.0);
        for (a, b) in cells {
            *a += b as f64;
        }
        self.count += 1;
    }
}

impl FromIterator<Outcome> for Accumulator {
    fn from_iter<T: IntoIterator<Item = Outcome>>(iter: T) -> Self {
        let mut acc = Accumulator::default();
        iter.into_iter().for_each(|outcome| acc += outcome);
        acc
    }
}

#[derive(Clone, Copy, Debug)]
struct Outcome([u8; 6]);

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [n1, n2, n3, n4, n5, n6] = self.0;
        write!(f, "{n1}, {n2}, {n3}, {n4}, {n5}, {n6}")
    }
}

impl FromIterator<u8> for Outcome {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut arr = [0; 6];
        let pairs = arr.iter_mut().zip(iter);

        for (cell, value) in pairs {
            *cell = value;
        }

        Outcome(arr)
    }
}

fn main() {
    run(&Args::parse());
}

fn run(args: &Args) {
    if let Some(iterations) = args.average {
        print_average(args.strategy(), iterations);
        return;
    }

    let mut rng = SquirrelRng::new();
    let mut provider = args.strategy().get_provider(&mut rng);
    let results = provider();

    println!("{results}");
}

fn print_average(strategy: &Command, i: usize) {
    let mut rng = SquirrelRng::new();
    let mut provider = strategy.get_provider(&mut rng);

    let results: Accumulator = (0..i).map(|_| provider()).collect();

    println!("{results}");
}
