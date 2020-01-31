use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::read_to_string;
use std::marker::Copy;

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
    T: Copy + std::cmp::PartialOrd + std::fmt::Debug,
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
}
