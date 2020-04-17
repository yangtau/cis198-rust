// Part 3.
// Lifetimes and move semantics.

// Problem 1.
// What went wrong? Copy strings properly.
#[test]
fn copy_string_test() {
    let str1 = String::from("foo");
    let str2 = str1.to_string();
    assert_eq!(str1, str2);
}

// Problem 2.
// Question 2: How come it works fine here?
#[test]
fn copy_int_test() {
    let i1 = 1;
    let i2 = i1;
    assert_eq!(i1, i2);
}

// Problem 3.
// These two don't work either. Fix by changing the type of "string" in the function
// copy_me ONLY, and by adjusting the parameter to "copy_me" where it's called.
#[test]
fn eat_me_test() {
    let str1 = String::from("foo");
    assert_eq!(str1, copy_me(&str1));
}

#[test]
fn eat_me_test2() {
    let str1 = String::from("foo");
    let str2 = copy_me(&str1);
    assert_eq!(str1, str2);
}

fn copy_me(string: &String) -> String {
    string.clone()
}

// Problem 4.
// Can you implement this function?
// Add a lifetime specifier to the return type if you think it will help.
// If not, why not.
// fn new_ref_string() -> &String {
//     // cannot return a refrence to data owned by the current function
// }

// fn new_ref_string() -> &'static String {
//     // cannot return a refrence to data owned by the current function
//     &String::from("hello")
// }

// Problem 5.
// Can you implement this function?
// Add a lifetime specifier to the return type if you think it will help.
// If not, why not.
fn new_ref_str() -> &'static str {
    "hello world"
}

#[test]
fn new_ref_str_tests() -> () {
    println!("{:?}", new_ref_str());
}

// Problem 6.
// Now we know how to implement this type of function. Implement it and write a test
// pick_longest_tests2() which passes all tests.
fn pick_longest2<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

#[test]
fn pick_longest_tests2() -> () {
    let s1: &str = "hello world";
    let s2: &str = "hello hello hello";
    assert_eq!(s2, pick_longest2(&&&&s1, s2));
}

// Problem 7.
// Write a function with a type signature which type checks the following test:
// and passes the test.
// This function compares it's second argument againsts all elements in it's first
// argument, returning those that are less than (<).
fn find_lesser_values<'a>(v: &Vec<&'a str>, s: &str) -> Vec<&'a str> {
    v.iter()
        .filter(|t| **t < s)
        .map(|x| *x)
        .collect::<Vec<&'a str>>()
}

#[test]
fn find_lesser_values_test() {
    assert_eq!(
        find_lesser_values(&vec!["foo", "bar", "foobar"], "zzzzzzzz"),
        vec!["foo", "bar", "foobar"]
    );
    assert_eq!(
        find_lesser_values(&vec!["foo", "bar", "foobar"], "bars"),
        vec!["bar"]
    );
    // Add more tests.
}

// Problem 8
// Move semantics present intersting API design choices not found in other languages.
// HashMap is an example of such a API.
// https://doc.rust-lang.org/std/collections/struct.HashMap.html

// Specifically, the Entry API:
// https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html

use std::collections::HashMap;

// Implement the following function which converts pairs from a vector,
// into key-value pairs in a hashmap.

fn vector_to_hashmap(v: &Vec<(i32, String)>) -> HashMap<i32, String> {
    let mut map = HashMap::new();
    for (ref i, ref s) in v {
        map.insert(*i, String::from(s));
    }
    map
}

#[test]
fn vector_to_hashmap_tests() {
    let v = vec![(32, String::from("hello")), (3, String::from("world"))];
    let mut h = HashMap::new();
    h.insert(32, String::from("hello"));
    h.insert(3, String::from("world"));
    assert_eq!(vector_to_hashmap(&v), h);
}

// Rust prevents us from deleting entries while iterating... Rewrite
// this function to delete all entries in hashmap where the keys
// are negative.
fn delete_negative_keys(h: &mut HashMap<i32, i32>) {
    // This fails, uncomment to see error.
    // for k in h.keys() {
    //     if *k < 0 {
    //         h.remove(k);
    //     }
    // }
    let v: Vec<i32> = h.keys().filter(|k| **k < 0).map(|x| *x).collect();
    for k in &v {
        h.remove(k);
    }
}

#[test]
fn delete_negative_keys_tests() {
    let mut h: HashMap<i32, i32> = HashMap::new();
    h.insert(3, 4);
    h.insert(2, 5);
    h.insert(1, 6);
    h.insert(-1, 6);
    h.insert(-5, 6);
    delete_negative_keys(&mut h);
    assert!(!h.contains_key(&-1));
    assert!(!h.contains_key(&-5));
    assert!(h.contains_key(&1));
    assert!(h.contains_key(&2));
    assert!(h.contains_key(&3));
}

// For all entries in `add`: (k, v)
// If `k` exists in `merged`, append `v` to the value of `merged[k]`.
// If that `k` doesn't exist in `merged`, add the (k, v) to `merged`.

// Use the Entry API:
// https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html
// Use `or_insert` and `and_modify`.
fn merge_maps(merged: &mut HashMap<String, String>, add: HashMap<String, String>) {
    for (k, v) in add {
        merged.entry(k).and_modify(|x| x.push_str(&v)).or_insert(v);
    }
}

#[test]
fn merge_maps_tests() {
    fn v2h(v: Vec<(String, String)>) -> HashMap<String, String> {
        let mut h = HashMap::new();
        for (t, s) in v {
            h.insert(t, s);
        }
        h
    }
    let mut h1 = v2h(vec![
        (String::from("hello"), String::from("hello")),
        (String::from("Yes"), String::from("No")),
    ]);
    let h2 = v2h(vec![
        (String::from("Yes"), String::from("No")),
        (String::from("hello"), String::from(" world")),
        (String::from("hi"), String::from("world")),
    ]);
    let h3 = v2h(vec![
        (String::from("Yes"), String::from("NoNo")),
        (String::from("hello"), String::from("hello world")),
        (String::from("hi"), String::from("world")),
    ]);
    merge_maps(&mut h1, h2);
    assert_eq!(h1, h3);
}
