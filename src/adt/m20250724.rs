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
    m: usize
  }
  let mut a = vec![0 as i64;n+1];
  a[0] = 1;
  let mask = 1000000000 as i64;
  let mut sum2 = 0 as i64;
  for i in 1..=n {
    if i < m {
      a[i] = 1;
      sum2 = (sum2 + a[i]) % mask;
      continue;
    }
    if i>m {
      sum2 = (mask + sum2 + a[i-1] - a[i-m-1]) % mask;
    } else {
      sum2 = (sum2 + a[i-1]) % mask;
    }
    a[i] = sum2;
    //println!("N{}: {}", i, a[i]);
  }
  println!("{}", a[n]);
}

pub fn f() {
  input! {
    n: usize,
    a: [i64; n],
  }
  let mut b = vec![0 as i64; n-1];
  for i in 1..n {
    b[i-1] = a[i] - a[i-1];
  }
  let mut sum = 0 as i64;
  let mut count = 0 as i64;
  let mut last: i64 = 0;
  for i in 0..n-1 {
    if i == 0 {
      last = b[i];
      count += 1;
    } else if last == b[i] {
      count += 1;
    } else {
      //println!("{} {}", last, count);
      sum += count * (count + 1) / 2;
      count = 1;
      last = b[i];
    }
  }
  sum += count * (count + 1) / 2;
  println!("{}", sum+n as i64);
}


