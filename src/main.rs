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
    S: String,
  }
  let S = S.chars().collect::<Vec<char>>();
  let mut res = 1;
  for i in 0..N {
    //println!("{} {} {} {}", i, count1, count2, mode);
    if S[i] == '/' {
      let mut count = 0;
      let mut sub = 1;
      while(i>=sub && i+sub <N) {
        if(S[i-sub] == '1' && S[i+sub] == '2') {
          count += 1;
        } else {
          break;
        }
        sub+=1;
      }
      res = max(res, count*2+1);
    }
  }
  println!("{}", res);
}