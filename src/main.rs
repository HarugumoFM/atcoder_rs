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
    a: f64,
    b: f64,
  }
  let mut x = (a/2.0/b).powf(2.0/3.0)-1.0;
  let mut rounded = x.round();
  let mut ceil = x.ceil();
  let mut r1 = b*rounded + a/(1.0+rounded).powf(1.0/2.0);
  let mut r2 = b*ceil + a/(1.0+ceil).powf(1.0/2.0);
  let mut ans = if r1<=r2 { r1 } else { r2};
  println!("{}", ans);
}

