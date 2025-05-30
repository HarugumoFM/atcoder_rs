use proconio::input;
use itertools::{Itertools,sorted};
use std::collections::{HashMap, HashSet, BTreeSet};
use std::cmp::min;
use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;

pub fn e() {
     input!{
    N: usize,
    Q: usize,
    A: [i64; N],
  }
  let mut map = HashMap::<i64, Vec<usize>>::new();
  for i in 0..N {
    if( !map.contains_key(&A[i]) ){
      map.insert(A[i], vec![]);
    }
    map.get_mut(&A[i]).unwrap().push(i+1);
  }
  for _ in 0..Q {
    input!{
      x: i64,
      k: usize,
    }
    if let Some(v) = map.get(&x) {
      if v.len() >= k {
        println!("{}", v[k-1]);
      } else {
        println!("-1");
      }
    } else {
      println!("-1");
    }
  }
}
    
pub fn f() {
    input!{
    N: usize,
    A: i64,
    B: i64,
    D: [i64; N],
  }
  let mut set = HashSet::new();
  for i in 0..N {
    set.insert(D[i]% (A+B));
  }
  let mut sort_by = set.into_iter().collect::<Vec<i64>>();
  sort_by.sort();
  if(sort_by.len() == 1 && sort_by[0] == 0) {
    println!("Yes");
    return;
  }
  let mut max = 0 as i64;
  for i in 1..sort_by.len() {
    if( sort_by[i] - sort_by[i-1] > max) {
      max = sort_by[i] - sort_by[i-1];
    }
  }
  if(A+B - sort_by[sort_by.len()-1] + sort_by[0] > max) {
    max = A + B - sort_by[sort_by.len()-1] + sort_by[0];
  }
  if(max > B) {
    println!("Yes");
  } else {
    println!("No");
  }
}


pub fn g() {
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