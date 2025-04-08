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
      n :usize,
      q :usize,
      s1 : String
    }
    let mut s = s1.chars().collect::<Vec<char>>();
    let mut sum = vec![0; n+1];
    let mut count = 0;
    for i in 1..n {
      if(s[i] == s[i-1]) {
         count += 1;
      }
      sum[i] = count;
      //println!("{} {}", i, sum[i]);
    }
    for i in 0..q {
      input! {
        l : usize,
        r : usize
      }
      let ans = sum[r-1] - sum[l-1];
      println!("{}", ans);
    }
}