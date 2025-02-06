use proconio::input;
use itertools::{Itertools,sorted};
use std::collections::{HashMap, HashSet, BTreeSet};
use std::cmp::min;
use std::vec;

pub fn e() {
    input! {
        x:usize,
    }
    let mut list:Vec<i64> = vec![];
    for i in 1..1<<10 {
        let mut res = String::from("");
        for j in (0..=9).rev() {
            if 1<<j &i != 0 {
                let ch = (j as u8 + b'0') as char;
                res.push(ch);
            } 
        }
        list.push(res.parse::<i64>().unwrap());
    }
    list.sort();

    println!("{}", list[x]);
}