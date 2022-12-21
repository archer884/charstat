use crate::Outcome;

pub(crate) fn traditional(rng: &mut impl Iterator<Item = u8>) -> Outcome {
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

pub(crate) fn drop_twice(rng: &mut impl Iterator<Item = u8>) -> Outcome {
    let sets = chunks(rng);
    let mut values = sets.map(|set: [u8; 4]| {
        let min = set.iter().copied().min().unwrap_or(0);
        let sum: u8 = set.iter().copied().sum();
        sum - min
    });

    let mut values = [
        values.next().unwrap(),
        values.next().unwrap(),
        values.next().unwrap(),
        values.next().unwrap(),
        values.next().unwrap(),
        values.next().unwrap(),
        values.next().unwrap(),
    ];

    values.sort();
    values.into_iter().skip(1).collect()
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
