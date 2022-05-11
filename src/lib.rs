use std::cmp::Ordering::Equal;

// Trick from https://www.worthe-it.co.za/blog/2017-01-15-aliasing-traits-in-rust.html to alias traits
// Copy + Clone slows the sort algo, which is well, to be expected.
pub trait Numeric: PartialOrd + Copy + Clone {}

impl<T> Numeric for T
    where T: PartialOrd + Copy + Clone {}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Pair<T>(T, T)
where
    T: Numeric;

#[derive(Clone)]
enum PairOption<T: Numeric> {
    Empty,
    One(T),
    Pair(Pair<T>)
}

// Clones iter into a vec, sorts it, and returns the top 2. Very naive impl
pub fn sort_max2<I, T>(input: I) -> Pair<T>
where
    I: IntoIterator<Item = T>,
    T: Numeric,
{
    let mut sorted = input.into_iter().collect::<Vec<T>>();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
    let x = sorted.pop().unwrap();
    let y = sorted.pop().unwrap();
    Pair(x, y)
}

// Fold in Rust nomenclature is what I typically expect to be reduce.
pub fn fold_max2<I, T>(input: I) -> Pair<T>
where
    I: IntoIterator<Item = T>,
    T: Numeric,
{   
    let mut input = input.into_iter();
    let initial_value = {
        let x = input.next().unwrap();
        let y = input.next().unwrap();
        Pair(x, y)
    };
    input.fold(initial_value, |current, next| {
        if next > current.0 {
            return Pair(next, current.0)
        }
        if next > current.1 {
            return Pair(current.0, next)
        }
        current
    })
}

fn compare<T: Numeric>(current: PairOption<T>, next: T) -> PairOption<T> {
    match current {
        PairOption::Empty => { PairOption::One(next) },
        PairOption::One(x) => {
            if next > x {
                PairOption::Pair(Pair(next, x))
            } else {
                PairOption::Pair(Pair(x, next))
            }
        },
        PairOption::Pair(pair) => {
            if next > pair.0 {
                PairOption::Pair(Pair(next, pair.0))
            } else if next > pair.1 {
                PairOption::Pair(Pair(pair.0, next))
            } else {
                current
            }
        }
    }
}

// Pairs on both sides version of the above function. Bit dicey, and a shame that definitions need to be doubled up
fn compare_pair<T: Numeric>(current: PairOption<T>, next: PairOption<T>) -> PairOption<T> {
    match (current, next) {
        (PairOption::Empty, PairOption::Empty) => PairOption::Empty,
        (PairOption::Empty, PairOption::One(x)) => PairOption::One(x),
        (PairOption::One(x), PairOption::Empty) => PairOption::One(x),
        (PairOption::Empty, PairOption::Pair(pair)) => PairOption::Pair(pair),
        (PairOption::Pair(pair), PairOption::Empty) => PairOption::Pair(pair),
        (PairOption::One(x), PairOption::One(y)) => PairOption::Pair( if x > y { Pair(x, y) } else { Pair(y, x) } ),
        (PairOption::One(x), PairOption::Pair(pair)) => {
            if x > pair.0 {
                PairOption::Pair(Pair(x, pair.0))
            } else if x > pair.1 {
                PairOption::Pair(Pair(pair.0, x))
            } else {
                PairOption::Pair(pair)
            }
        },
        (PairOption::Pair(pair), PairOption::One(x)) => {
            if x > pair.0 {
                PairOption::Pair(Pair(x, pair.0))
            } else if x > pair.1 {
                PairOption::Pair(Pair(pair.0, x))
            } else {
                PairOption::Pair(pair)
            }
        },
        (PairOption::Pair(lhs), PairOption::Pair(rhs)) => {
            // Hey, look, we're using the sort option here, wasting the last two.
            // Imagine this is comparable considering the set is limited to 4.
            let mut nums = vec!(lhs.0, lhs.1, rhs.0, rhs.1);
            nums.sort_by(|a, b| b.partial_cmp(a).unwrap_or(Equal));
            PairOption::Pair(Pair(nums[0], nums[1]))
        },
    }
}

// So the previous is pretty slick, but at the end of the day, it's a for loop, right?
pub fn loop_max2<I, T>(input: I) -> Pair<T>
where
    I: IntoIterator<Item = T>,
    T: Numeric
{
    let mut current = PairOption::Empty;

    // Basically the same body in fold.
    for next in input.into_iter() {
        match current {
            PairOption::Empty => { current = PairOption::One(next); },
            PairOption::One(x) => {
                if next > x {
                    current = PairOption::Pair(Pair(next, x));
                } else {
                    current = PairOption::Pair(Pair(x, next));
                }
            },
            PairOption::Pair(pair) => {
                let pair = pair.clone();
                if next > pair.0 {
                    current = PairOption::Pair(Pair(next, pair.0));
                } else if next > pair.1 {
                    current = PairOption::Pair(Pair(pair.0, next));
                }
            }
        }
    }

    match current {
        PairOption::Empty => panic!("Empty input"),
        PairOption::One(_) => panic!("Insufficient inputs"),
        PairOption::Pair(pair) => pair
    }

}

// This is suppsoed to match the article's transform reduce, but that does not really exist in rust.
// Instead, using the lessons learned from the previous loop function to improve over the fold version.
pub fn transform_max2<I, T>(input: I) -> Pair<T>
where
    I: IntoIterator<Item = T>,
    T: Numeric,
{
    match input.into_iter().fold(PairOption::Empty, compare) {
        PairOption::Empty => panic!("Empty input"),
        PairOption::One(_) => panic!("Insufficient inputs"),
        PairOption::Pair(pair) => pair
    }
}

use rayon::prelude::*;

// Now, for the main attraction; parallel!
pub fn par_max2<I, T>(input: I) -> Pair<T>
where
    I: IntoParallelIterator<Item = T>,
    T: Numeric + Send + Sync,
{
    let result = input.into_par_iter()
        .fold_with(PairOption::Empty, compare)
        .reduce(|| PairOption::Empty, compare_pair);

    // Aha! This may have been the transform-reduce we've been looking for. The fold_with does a bit of processing, turning into pairs and reducing the set down.
    // When it's all pairs, we can then do a final reduce with that giga-function we have designed, which must be the equivalent of that function object they were
    // referring to from C++. However, where they would use function overloading, we can use pattern matching instead.

    match result {
        PairOption::Empty => panic!("Empty input"),
        PairOption::One(_) => panic!("Insufficient inputs"),
        PairOption::Pair(pair) => pair
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    const SAMPLE: [i32; 25] = [98, 20, 65, 10, 72, 25, 70, 60, 40, 77, 64, 46,  3, 42, 33, 50, 83, 36, 41, 32, 89, 88, 23, 21, 61];
    const RESULT: Pair<i32> = Pair(98, 89);

    #[test]
    fn test_sort() {
        assert_eq!(RESULT, sort_max2(SAMPLE));
    }

    #[test]
    fn test_fold() {
        assert_eq!(RESULT, fold_max2(SAMPLE));
    }

    #[test]
    fn test_loop() {
        assert_eq!(RESULT, loop_max2(SAMPLE));
    }

    #[test]
    fn test_transform() {
        assert_eq!(RESULT, transform_max2(SAMPLE));
    }

    #[test]
    fn test_par() {
        assert_eq!(RESULT, par_max2(SAMPLE));
    }
}