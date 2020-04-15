#![allow(dead_code)]

// Uncomment these to have Rust compile the other files as well.
mod lib2;
mod lib3;

// Part 1. Implementing Functions. Taken from Fall 2016's Rust class.
// Write unit tests for you functions.

// Problem 1
// Implement the sum function on slices. Do not use the predefined sum function.
fn sum(slice: &[i32]) -> i32 {
    let mut s = 0;
    for x in slice {
        s += x;
    }
    s
}
#[test]
fn sum_tests() {
    let a = [];
    assert_eq!(sum(&a), a.iter().sum::<i32>());

    let a = [1, 9, 2, 3, 4, 5, 5, 6, 7];
    assert_eq!(sum(&a), a.iter().sum::<i32>());

    let a = [9, 5, 7, 1, 2, 5, 8, 10];
    assert_eq!(sum(&a), a.iter().sum::<i32>());
}

// Problem 2.
// Make unique. Create a new vector which contains each item in the vector
// only once! Much like a set would.
// Please implement this using a for loop.
fn unique(vs: &Vec<i32>) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    for x in vs {
        if !v.contains(x) {
            v.push(*x);
        }
    }
    v
}

#[test]
fn unique_tests() {
    use std::collections::HashSet;
    use std::iter::FromIterator;
    let v: Vec<i32> = vec![1, 1, 2, 3, 1, 3, 7, 6, 4, 5, 6, 7];

    let hash: HashSet<&i32> = HashSet::from_iter(&v);
    let mut v1 = hash.iter().map(|x| **x).collect::<Vec<i32>>();
    let v1 = v1.sort();

    let mut v = unique(&v);
    let v = v.sort();

    assert_eq!(v, v1);
}

// Problem 3.
// return a new vector containing only elements that satisfy `pred`.
fn filter(vs: &Vec<i32>, pred: &dyn Fn(i32) -> bool) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    for x in vs {
        if pred(*x) {
            v.push(*x);
        }
    }
    v
}

#[test]
fn filter_tests() {
    assert_eq!(
        filter(&vec![1, 2, 3, 4, 5, 6], &|n| n % 2 == 0),
        vec![2, 4, 6]
    );
}

// Problem 4
// Given starting fibonacci numbers n1 and n2, compute a vector
// where v[i] is the ith fibonacci number.
fn fibonacci(n1: i32, n2: i32, how_many: usize) -> Vec<i32> {
    let (mut a, mut b) = (n1, n2);
    let mut v: Vec<i32> = Vec::new();

    for _ in 0..how_many {
        v.push(a);
        let t = b;
        b = a + b;
        a = t;
    }
    v
}

#[test]
fn fibonacci_tests() {
    let v = vec![0, 1, 1, 2, 3, 5, 8, 13, 21];
    assert_eq!(v, fibonacci(0, 1, v.len()));

    let v = vec![1, 1, 2, 3, 5, 8, 13, 21];
    assert_eq!(v, fibonacci(1, 1, v.len()));
}

// Problem 5
// Create a function which concats 2 strs and returns a String.
// You may use any standard library function you wish.
fn str_concat(s1: &str, s2: &str) -> String {
    let mut t = String::from(s1);
    t.push_str(s2);

    t
}

#[test]
fn str_concat_tests() {
    let (s1, s2) = ("hello ", "world");
    let s = str_concat(&s1, &s2);
    let t = format!("{}{}", s1, s2);
    assert!(s.eq(&t));
}

// Problem 6
// Create a function which concats 2 string and returns a String.
// You may use any standard library function you wish.
fn string_concat(s1: &String, s2: &String) -> String {
    let mut t = String::from(s1);
    t.push_str(&s2);
    t
}

#[test]
fn string_concat_tests() {
    let (s1, s2) = (String::from("hello "), String::from("world"));
    let s = string_concat(&s1, &s2);
    let t = format!("{}{}", s1, s2);
    assert!(s.eq(&t));
}

// Problem 7
// Convert a Vec<String> into a Vec<u64>. Assume all strings
// are correct numbers! We will do error handling later. Use
// `.expect("ignoring error")` to ignore Result from parse()
// See https://doc.rust-lang.org/std/primitive.str.html#method.parse
// Use turbo fish! Do not use type inference for parse()
fn concat_all(v: Vec<String>) -> Vec<u64> {
    let mut ve: Vec<u64> = Vec::new();
    for s in v {
        let x: u64 = s.parse::<u64>().expect("ignoring error");
        ve.push(x);
    }
    ve
}

#[test]
fn concat_all_tests() {
    let v = ["1234", "1343", "43242", "32412", "32417598"]
        .iter()
        .map(|x| String::from(*x))
        .collect::<Vec<String>>();
    assert_eq!(concat_all(v), vec![1234, 1343, 43242, 32412, 32417598])
}

// Implement concat_all using map, parse (with turbo fish), and collect()
// Check out how the lecture does something similar:
// https://github.com/upenn-cis198/lecture2/blob/f54753527c1dabbd5e55c2f48a19745768769beb/src/lib.rs#L362
fn concat_all_with_map(v: Vec<String>) -> Vec<u64> {
    v.iter()
        .map(|x| x.parse::<u64>().expect("Please type a number"))
        .collect::<Vec<u64>>()
}

#[test]
fn concat_all_with_map_tests() {
    let v = ["1234", "1343", "43242", "32412", "32417598"]
        .iter()
        .map(|x| String::from(*x))
        .collect::<Vec<String>>();

    assert_eq!(
        concat_all_with_map(v),
        vec![1234, 1343, 43242, 32412, 32417598]
    )
}
