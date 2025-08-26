use proconio::input;
use itertools::{Itertools,sorted};
use std::collections::{HashMap, HashSet, BTreeSet};
use std::cmp::min;
use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;

pub fn e() {
  input! {
    N: usize,
    mut W: i64,
  }
  let mut vec = Vec::new();
  for i in 0..N {
    input! {
      _a: i64,
      _b: i64
    }
    vec.push(cheese { A: _a, B: _b });
  }
  vec.sort_by(|a, b| b.A.cmp(&a.A));
  let mut sum = 0 as i64;
  for cheese in vec {
    if W <= 0 {
      break;
    }
    let take = min(W, cheese.B);
    sum += take * cheese.A;
    W -= take;
  }
  println!("{}", sum);
}
struct cheese {
  pub A: i64,
  pub B: i64,
}






fn f() {
  input! {
    N: usize,
    M: usize,
    s: String,
    a: [usize; N]
  }
  let mut indexes = vec![0; N+1];
  let _s = s.chars().collect::<Vec<_>>();
  let mut map: HashMap<usize, Vec<&char>> = HashMap::new();
  for i in 0..N {
    if map.contains_key(&a[i]) {
      map.get_mut(&a[i]).unwrap().push(&_s[i]);
    } else {
      let mut chars = Vec::new();
      chars.push(&_s[i]);
      map.insert(a[i], chars);
    }
  }
  for (key, mut value) in &mut map {
    if value.len() == 0{
      continue;
    }
    let c = value.pop();
    value.insert(0,c.unwrap());
  }
  for i in 0..N {
    let index:usize = a[i];
    print!("{}", map.get(&index).unwrap()[indexes[index]]);
    indexes[index] += 1;
  }
  println!();

}
