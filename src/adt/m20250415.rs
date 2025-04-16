use proconio::input;
use itertools::{Itertools,sorted};
use std::collections::{HashMap, HashSet, BTreeSet};
use std::cmp::min;
use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;

pub fn e() {
    input! {
        n: usize,
        x: i64,
        y: i64,
    }
    let mut a = vec![];
    a.push([0,1]);
    for i in 2..=n {
        let mut v = [0,0];
        v[1] = a[i-2][0] + y*a[i-2][1];
        v[0] = a[i-2][0] + x*v[1];
        //println!("{:?}", v);
        a.push(v);
    }
    println!("{}", a[n-1][0]);
}

pub fn f() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut ans = 0;
    let mut sum = 0;
    for i in 0..n {
        sum += a[i];
        ans = max(ans, sum);
        if sum < 0 {
            sum = 0;
        }
    }
    println!("{}", ans);

}

struct Item{
    pub score: i64,
    pub s: String,
}
