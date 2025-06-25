use proconio::input;
use itertools::{Itertools,sorted};
use std::collections::{HashMap, HashSet, BTreeSet};
use std::cmp::min;
use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;

pub fn e() {
   input!{
    n: usize,
    mut a: [i64; n],
    mut b: [i64; n-1],
  }
  a.sort();
  b.sort();
  let mut vec = vec![];
  let mut index = (n-2) as i64;
  let mut index2 = (n-1) as i64; 
  while index>=0 && index2>=0 {
    if b[index as usize] >= a[index2 as usize] {
      index -= 1;
      index2 -=1;
    } else {
      vec.push(a[index2 as usize]);
      index2 -= 1;
    }
  }
  if index == -1{
    if vec.len() == 0 {
      println!("{}", a[0]);
    } else if vec.len() == 1 {
      println!("{}", vec[0]);
    } else {
      println!("-1");
    }
  } else {
    println!("-1");
  }
}

pub fn f() {
    input! {
    mut n: i64
  }
  let mut ans = vec![];
  while n != 0 {
    if n%2 == 0 {
      ans.push('B');
      n /= 2;
    } else {
      ans.push('A');
      n -= 1;
    }
  }
  ans.reverse();
  for c in ans {
    print!("{}", c);
  }
  println!();
}


pub fn g() {
   input! {
    mut n: usize,
    mut a: [i64; n]
  }
  let mut vec = vec![];
  let mut count = 0;
  let mut set_count = 0;
  for i in 0..n {
    if count == 0 {
      vec.push(Ball { num: a[i], count: 1 });
      count += 1;
      set_count += 1;
    } else if vec[set_count -1].num == a[i] {
      if vec[set_count -1].count == a[i] as i64 -1 {
        count -= a[i] as i64 -1;
        vec.pop();
        set_count -= 1;
      } else {
        count += 1;
        vec[set_count -1].count += 1;
      }
    } else {
      vec.push(Ball { num: a[i], count: 1 });
      count += 1;
      set_count += 1;
    }
    println!("{}", count);
  }
}

struct Ball {
  pub num: i64,
  pub count: i64
}
