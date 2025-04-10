use proconio::input;
use itertools::{Itertools,sorted};
use core::num;
use std::collections::{HashMap, HashSet, BTreeSet};
use std::cmp::min;
use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;


fn main() {
    input! {
      n: usize,
    }
    let mut x = vec![vec![0 as i64; 2];n];
    let mut mins = HashMap::new();
    let mut m = 0 as i64;
    for i in 0..n {
        input! {
            a: i64,
            b: i64,
        }
        x[i][0] = a;
        x[i][1] = b;
        if mins.contains_key(&b) {
            let mut v = mins.get_mut(&b).unwrap();
            *v = min(*v, a);
        } else {
            mins.insert(b, a);
        }

    }
    let mut ans = 0;
    for i in mins {
       ans = max(ans, i.1);
    }
    println!("{}", ans);
}


