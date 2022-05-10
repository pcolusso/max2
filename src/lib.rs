#[derive(Debug, PartialEq)]
pub struct Pair<T>(T, T)
where
    T: Ord;

// Clones iter into a vec, sorts it, and returns the top 2. Very naive impl
pub fn sort_max2<I, T>(input: I) -> Pair<T>
where
    I: IntoIterator<Item = T>,
    T: Ord,
{
    let mut sorted = input.into_iter().collect::<Vec<T>>();
    sorted.sort();
    let x = sorted.pop().unwrap();
    let y = sorted.pop().unwrap();
    Pair(x, y)
}

pub fn fold_max2<I, T>(input: I) -> Pair<T>
where
    I: IntoIterator<Item = T>,
    T: Ord,
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

fn transform_max2<I, T>(input: I) -> Pair<T>
where
    I: IntoIterator<Item = T>,
    T: Ord,
{
    unimplemented!()
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
}