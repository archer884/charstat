use rand::{
    distributions::{DistIter, Uniform},
    prelude::*,
    Rng,
};

pub trait Dice: Rng + Sized {
    fn roll_d6(&mut self) -> IterD6<Self>;
}

impl<T: Rng> Dice for T {
    fn roll_d6(&mut self) -> IterD6<Self> {
        IterD6 {
            source: Uniform::new_inclusive(1, 6).sample_iter(self),
        }
    }
}

pub struct IterD6<'a, T: Rng> {
    source: DistIter<Uniform<u8>, &'a mut T, u8>,
}

impl<'a, T: Rng> Iterator for IterD6<'a, T> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.next()
    }
}
