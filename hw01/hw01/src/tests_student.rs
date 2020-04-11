#![cfg(test)]

use crate::problem1::{dedupe, filter, sum};
use crate::problem2::mat_mult;
use crate::problem3::sieve;

// Part 1

#[test]
fn test_sum_small() {
    let array = [1, 2, 3, 4, 5];
    assert_eq!(sum(&array), 15);
}

// Part 2

#[test]
fn test_dedup_small() {
    let vs = vec![1, 2, 2, 3, 4, 1];
    assert_eq!(dedupe(&vs), vec![1, 2, 3, 4]);
}

// Part 3

fn even_predicate(x: i32) -> bool {
    (x % 2) == 0
}

#[test]
fn test_filter_small() {
    let vs = vec![1, 2, 3, 4, 5];
    assert_eq!(filter(&vs, &even_predicate), vec![2, 4]);
}

//
// Problem 2
//

#[test]
fn test_mat_mult_identity() {
    let mut mat1 = vec![vec![0.; 3]; 3];
    for i in 0..mat1.len() {
        mat1[i][i] = 1.;
    }
    let mat2 = vec![vec![5.; 3]; 3];
    let result = mat_mult(&mat1, &mat2);
    for i in 0..result.len() {
        for j in 0..result[i].len() {
            assert_eq!(result[i][j], mat2[i][j]);
        }
    }
}

//
// Problem 3
//

#[test]
fn test_sieve_basic() {
    assert_eq!(vec![2, 3, 5, 7, 11], sieve(12));
}
