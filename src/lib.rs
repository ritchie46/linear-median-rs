use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::marker::Copy;
extern crate num;
use num::{Num, NumCast};

fn choose_random<T>(a: &[T]) -> Option<T>
where
    T: Copy,
{
    let mut rng = thread_rng();
    let smpl = *a.choose(&mut rng)?;
    Some(smpl)
}

fn quickselect<T>(a: &[T], k: usize, pivot_fn: &dyn Fn(&[T]) -> Option<T>) -> Option<T>
where
    T: Copy + PartialOrd,
{
    if a.len() == 1 {
        return match k {
            0 => Some(a[0].clone()),
            _ => None,
        };
    }
    let pivot = pivot_fn(a)?;
    let lows: Vec<T> = a.iter().map(|x| *x).filter(|x| *x < pivot).collect();
    let highs: Vec<T> = a.iter().map(|x| *x).filter(|x| *x > pivot).collect();
    let pivots: Vec<T> = a.iter().map(|x| *x).filter(|x| *x == pivot).collect();

    return if k < lows.len() {
        quickselect(&lows, k, pivot_fn)
    } else if k < (lows.len() + pivots.len()) {
        Some(pivots[0])
    } else {
        quickselect(&highs, k - lows.len() - pivots.len(), pivot_fn)
    };
}

fn nlogn_median<T>(a: &[T]) -> Option<T>
// https://users.rust-lang.org/t/generic-arithmetic-with-std-ops-add/6848
where
    T: Copy + PartialOrd + Num + NumCast,
{
    let mut a: Vec<T> = a.iter().cloned().collect();
    // Todo: unwrap with error handling
    a.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mid = a.len() / 2;
    if a.len() % 2 == 0 {
        let div = T::from(2)?;
        Some((a[mid - 1] + a[mid]) / div)
    } else {
        Some(a[mid])
    }
}

fn quickselect_median<T>(a: &[T], pivot_fn: &dyn Fn(&[T]) -> Option<T>) -> Option<T>
where
    T: Copy + PartialOrd + Num + NumCast,
{
    if a.len() % 2 == 1 {
        quickselect(a, a.len() / 2, pivot_fn)
    } else {
        // median is not exactly in the middle of the array when sorted
        let median_min1 = quickselect(a, a.len() / 2 - 1, pivot_fn)?;
        let median_plus1 = quickselect(a, a.len() / 2, pivot_fn)?;
        let div = T::from(2)?;
        Some((median_min1 + median_plus1) / div)
    }
}

fn pick_pivot<T>(a: &[T]) -> Option<T>
where
    T: Copy + PartialOrd + Num + NumCast,
{
    // Todo
    if a.len() == 0 {
        return None;
    } else if a.len() < 5 {
        return nlogn_median(a);
    };

    let medians: Vec<T> = a
        .chunks_exact(5) // [[chunk-5], [chunk-5]...[chunk-5]]
        .map(|x| {
            // Need a mutable vector to be able to sort.
            let mut a: Vec<T> = x.clone().to_vec();
            // Every chunk has size of 5. So after sorting the median
            // is at index 2.
            // Todo: unwrap with error handling
            a.sort_by(|a, b| a.partial_cmp(b).unwrap());
            // select median
            a[2]
        })
        .collect();
    quickselect_median(&medians, &pick_pivot)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let a = [1, 3, 4];
        let smpl = choose_random(&a).unwrap();
        assert!(a.contains(&smpl));
    }

    #[test]
    fn test_quickselect() {
        let a = [1];
        assert_eq!(quickselect(&a, 0, &choose_random).unwrap(), 1);

        // Selection k of empty slice should return None
        let a: [i32; 0] = [];
        assert_eq!(quickselect(&a, 0, &choose_random), None);

        // runs on floats
        let a: [f32; 4] = [1., 3., 5., 2.];
        quickselect(&a, 1, &choose_random);

        let a = [1, 2, 3, 4, 5];
        assert_eq!(quickselect(&a, 2, &choose_random).unwrap(), 3);
    }

    #[test]
    fn test_median() {
        let a = [1, 3, 4, 5, 6, 6, 8, 2, 3];
        assert_eq!(nlogn_median(&a), quickselect_median(&a, &choose_random));
        assert_eq!(nlogn_median(&a), quickselect_median(&a, &pick_pivot));
    }
}
