use proconio::input;
use itertools::{Itertools,sorted};
use std::collections::{HashMap, HashSet, BTreeSet};
use std::cmp::min;
use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;

pub fn e() {
  let mut cheeses = vec![];
  input! {
    n: usize,
    mut w: i64
  } 
  for i in 0..n {
    input! {
      a: i64,
      b: i64,
    }
    cheeses.push(Cheese { tasty: a, weight: b });
  }
  cheeses.sort_by(|a, b| b.tasty.cmp(&a.tasty));
  let mut sum = 0 as i64;
  for cheese in cheeses {
    if w >= cheese.weight {
      w -= cheese.weight;
      sum += cheese.tasty * cheese.weight;
    } else  if w < cheese.weight {
      sum += cheese.tasty * w;
      break;
    }
  }
  println!("{}", sum);
}

struct Cheese {
   pub tasty: i64,
   pub weight: i64
}
 
pub fn f() {
  input! {
    n: i64
  } 
  let mut index = (n as f64).powf(1.0/3.0) as i64;
  //println!("{}", index);
  for i in 0..=index {
    let num = (index+1 - i) * (index+1 - i) * (index+1 - i);
    let reverse_num = num.to_string().chars().rev().collect::<String>();
    let reverse_num: i64 = reverse_num.parse().unwrap();
    if num == reverse_num && num <= n {
      println!("{}", num);
      return
    }
  }
}


pub fn g() {
  input! {
    n: usize
  } 
  for i in 0..n {
    input! {
      a: i64,
      mut s: i64
    }
    s = s -a - a;
    let mut count = 0;
    let mut mask = 1;
    let mut res = true;
    if s <0 {
      println!("No");
      continue;
    }
    for i in 0..60 {
      if s & mask != 0 {
        count += 1;
        s = s - mask;
        if mask & a != 0 {
          res = false;
          break;
        }
      }
      mask *= 2;
    }
    if s == 0 && res {
      println!("Yes");
    } else {
      println!("No");
    }
  }
}