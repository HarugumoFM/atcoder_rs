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
    m: usize,
  }
  let mut list = vec![];
  for _ in 0..m {
    input! {
      l: usize,
      r: usize,
    }
    list.push(Walls { l, r });
  }
  list.sort_by(|a, b| a.l.cmp(&b.l));
  let mut index = 0;
  let mut set = HashMap::new();
  let mut min = 0;
  let mut count = 0;
  for i in 1..=n {
    while index < m && list[index].l <= i {
      if list[index].l == i {
        *set.entry(list[index].r).or_insert(0) += 1;
        count += 1;
      }
      index += 1;
    }
    if i== 1 {
      min = count;
    } else if count < min {
        min = count;
    }
    
    if let Some(v) = set.get_mut(&i) {
      count -= *v;
      set.remove(&i);
    }
  }
  println!("{}", min);
}

struct Walls {
  l: usize,
  r: usize,
}


pub fn g() {
  input! {
    q: usize,
  }
  let mut subs = vec![];
  let mut mask = 1 as i64;
  subs.push(1);
  let mut list = vec![];
  let mut index = 0;
  list.push(1);
  let mut sum = 1;
  let mut count = 1;
  for i in 0..q {
    input! {
      a: i64,
    }
    if a == 1 {
      input! {
        b: i64,
      }
      mask = (mask * 10) % 998244353;
      subs.push(mask);
      list.push(b);
      sum = (sum * 10 + b) % 998244353;
      count += 1;
    } else if a == 2 {
      sum = (sum - list[index] * subs[count - 1]) % 998244353;
      if sum < 0 {
        sum += 998244353;
      }
      index +=1;
      count -= 1;
      
    } else {
      println!("{}", sum);
    }

  }
}
