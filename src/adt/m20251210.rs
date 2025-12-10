use proconio::input;
use itertools::{Itertools,sorted};
use std::collections::{HashMap, HashSet, BTreeSet};
use std::cmp::min;
use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;

pub fn e() {
   input! {
    n: i64,
  }
  let mut _n = n;
  let mut mask = 1;
  let mut ans = vec![];
  for i in 0..64 {
    if _n & mask != 0 {
      ans.push(2);
      _n -= mask;
    } else {
      ans.push(0);
    }
    if _n == 0 {
      break;
    }
    mask = mask << 1;
  }
  ans.reverse();
  println!("{}", ans.iter().join(""));
}


pub fn g() {
   let mut res = true;
  input! {
    n: usize,
    m: usize,
  }
  let mut a = HashMap::new();
  let mut dic = HashMap::new();
  let mut count = HashMap::new();
  let mut count_route = HashMap::new();
  for i in 0..m {
    input! {
      x: usize,
      y: usize
    }
    if !dic.contains_key(&x) {
      dic.insert(x, HashSet::new());
    }
    if !dic.contains_key(&y) {
      dic.insert(y, HashSet::new());
    }
    dic.get_mut(&x).unwrap().insert(y);
    dic.get_mut(&y).unwrap().insert(x);
  }
  let mut start = 1;
  for i in 1..=n {
    if !a.contains_key(&i) {
      if !search(i, &mut dic, &mut a, &mut count, &mut count_route, start) {
        res = false;
        break;
      }
      start += 2;
    }
  }
  if res {
    let mut sum = 0;
    let mut ans = 0;
    for key in count.keys() {
      //println!("key: {}, count: {}", key, count[&key]);
      ans += count[&key] * sum;
      sum += count[&key];
    }
    println!("{}", ans-m);
  } else {
    println!("0");
  }
  
}
fn search (a:usize, route: &HashMap<usize, HashSet<usize>>, visited: &mut HashMap<usize, usize>, count: &mut HashMap<usize, usize>, count_route: &mut HashMap<usize, usize>, num:usize) -> bool {
  if visited.contains_key(&a)  {
    if visited[&a] != num {
      return false;
    } else {
      return true;
    }
  }
  //println!("visit: {}, num: {}", a, num);
  //カウント更新
  if count.contains_key(&num) {
    count.insert(num, count[&num]+1);
  } else {
    count.insert(num, 1);
  }
  visited.insert(a, num);
  let next_num = if num % 2 == 0 {num-1} else {num+1};
  if route.contains_key(&a) {
    for &to in &route[&a] {
      if !search(to, route, visited, count, count_route, next_num) {
        return false;
      }
    }
  }
  return true;
}
