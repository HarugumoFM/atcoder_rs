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
    m: usize,
    p: i64,
    mut a: [i64; n],
    mut b: [i64; m],
  }
  a.sort();
  b.sort();
  let mut sums = vec![0; m+1];
  let mut sum = 0;
  for i in 0..m {
    sum += b[i];
    sums[i+1] = sum;
  }
  let mut ans = 0;
  let mut index = (m-1) as i64;
  for i in 0..n {
    while index>=0 && a[i] + b[index as usize] > p {
      index -= 1;
    }
    //println!("{} {} {}", a[i], index, sums[index as usize+1]);
    if index <0 {
      ans += p* m as i64;
    } else {
      ans += p*(m as i64 -index-1) + sums[index as usize+1] + a[i] * (index+1) as i64;
    }
  }
  println!("{}", ans);

}