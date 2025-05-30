use proconio::input;
use itertools::{Itertools,sorted};
use core::num;
use std::collections::{HashMap, HashSet, BTreeSet};
use std::cmp::min;
use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;


fn main() {
  input!{
    N: usize,
    M: usize,
    D: i64,
    mut A: [i64; N],
    mut B: [i64; M],
  }
  A.sort();
  B.sort();
  let mut a_index = N -1;
  let mut b_index = M -1;
  let mut res = -1 as i64;
  while(a_index >= 0 && b_index >= 0) {
    if (A[a_index] - B[b_index]).abs() <= D {
      res = A[a_index]+B[b_index];
      break;
    } 
    if a_index == 0 && b_index == 0 {
      break;
    } else if a_index == 0 {
      b_index -= 1;
    } else if b_index == 0 {
      a_index -= 1;
    } else if A[a_index] - B[b_index] > D {
      a_index -= 1;
    } else {
      b_index -= 1;
    }
  }
  println!("{}", res);
}

