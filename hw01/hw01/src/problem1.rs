use std::collections::HashSet;

/// Computes the sum of all elements in the input i32 slice named `slice`
pub fn sum(slice: &[i32]) -> i32 {
    let mut s = 0;
    for i in slice {
        s = s + i
    }
    s
}

/// Deduplicates items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinct element of `vs`, preserving the
/// original order.
pub fn dedupe(vs: &Vec<i32>) -> Vec<i32> {
    let mut seen = HashSet::new();
    let mut out = Vec::<i32>::new();
    for v in vs {
        if !seen.contains(v) {
            seen.insert(v);
            out.push(*v)
        }
    }
    out
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    let mut out = Vec::<i32>::new();
    for v in vs {
        if pred(*v) {
            out.push(*v)
        }
    }
    out
}
