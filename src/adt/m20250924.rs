use proconio::input;
use itertools::{Itertools,sorted};
use std::collections::{HashMap, HashSet, BTreeSet};
use std::cmp::min;
use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;

pub fn e() {
  input! {
    mut x1: i64,
    mut y1: i64,
    mut x2: i64,
    mut y2: i64,
  }
  let mut result = false;
  let mut set_couple = HashSet::new();
  for a in 1..=2 {
    let b = 3 - a;
    set_couple.insert((x1 + a, y1 + b));
    set_couple.insert((x1 - a, y1 + b));
    set_couple.insert((x1 + a, y1 - b));
    set_couple.insert((x1 - a, y1 - b));
  }
  for a in 1..=2 {
    let b = 3 - a;
    if set_couple.contains(&(x2 + a, y2 + b)) || set_couple.contains(&(x2 - a, y2 + b)) || set_couple.contains(&(x2 + a, y2 - b)) || set_couple.contains(&(x2 - a, y2 - b)) {
      result = true;
    }
  }

  if result {
    println!("Yes");
  } else {
    println!("No");
  }
}

fn f() {
  input! {
    n: usize,
    a: [i64; n],
    mut x: i64
  }
  let mut sums = vec![0; n+1];
  let mut sum = 0;
  for i in 0..n {
    sum += a[i];
    sums[i+1] = sum;
  }
  let mut sub = x/sum;
  x -= sub*sum;
  //sumsでx以上の最小値を二分探索
  let mut left = 0;
  let mut right = n;
  while left < right {
    let mid = (left + right) / 2;
    if sums[mid] <= x {
      left = mid + 1;
    } else {
      right = mid;
    }
  }
  // leftがx以上の最小値のインデックス
  let mut ans = sub*n as i64 + right as i64;
  println!("{}", ans);

}

pub fn g() {
  let N = 1048576 as i64;
  input! {
    q: usize
  }
  let mut map = HashMap::new();
  let mut set = BTreeSet::new();
  for i in 0..N {
      set.insert(i);
  }
  for _ in 0..q {
    input! {
      t: i64,
      x: i64
    }
    
    if t == 1 {
      let mut h = x % N;
      if !set.contains(&h) {
        // hより大きいkeyのうち最小のもの
        if let Some(&key) = set.range(h..).next() {
          h = key;
        } else {
          // hより大きいkeyがなければ最小のkey
          h = *set.iter().next().unwrap();
        }
      }
      set.remove(&h);
      map.insert(h, x);
    } else {
      let h = x % N;
      if map.contains_key(&h) {
        println!("{}", map.get(&h).unwrap());
      } else {
        println!("-1");
      }
    }
  }
}
