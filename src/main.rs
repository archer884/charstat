use std::{fmt, ops};

mod dice;

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
}

impl Command {
    fn get_provider<'a>(&self, rng: &'a mut impl Rng) -> impl FnMut() -> Outcome + 'a {
        let mut provider = rng.roll_d6();
        match self {
            Command::Traditional => move || traditional(&mut provider),
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

fn traditional(rng: &mut impl Iterator<Item = u8>) -> Outcome {
    let sets = chunks(rng);
    let values = sets.map(|set: [u8; 4]| {
        let min = set.iter().copied().min().unwrap_or(0);
        let sum: u8 = set.iter().copied().sum();
        sum - min
    });

    let mut outcome: Outcome = values.collect();
    outcome.0.sort();
    outcome
}

struct ChunksIter<I, const N: usize> {
    source: I,
}

impl<I, const N: usize> Iterator for ChunksIter<I, N>
where
    I: Iterator,
    I::Item: Copy + Default,
{
    type Item = [I::Item; N];

    fn next(&mut self) -> Option<Self::Item> {
        let mut arr = [Default::default(); N];
        let pairs = self.source.by_ref().take(N).zip(arr.iter_mut());

        for (value, dest) in pairs {
            *dest = value;
        }

        Some(arr)
    }
}

fn chunks<const N: usize, I>(iter: I) -> ChunksIter<I, N>
where
    I: Iterator,
    I::Item: Copy + Default,
{
    ChunksIter { source: iter }
}
