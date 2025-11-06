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
    m: i64,
  }
  let mut map = HashMap::new();
  for i in 1..=n {
    map.insert(i, HashSet::new());
  }

  for _ in 0..m {
    input! {
      a: usize,
      b: usize,
    }
    map.get_mut(&a).unwrap().insert(b);
    map.get_mut(&b).unwrap().insert(a);
  }

  let mut visited = HashSet::new();
  let mut count = 0 as i64;
  for i in 1..=n {
    if !visited.contains(&i) {
      count += dfs(i, &map, &mut visited) - 1;
    }
  }
  println!("{}", m- count);
}
fn dfs (
  v: usize,
  map: &HashMap<usize, HashSet<usize>>,
  visited: &mut HashSet<usize>,
)-> i64 {
  let mut count = 0 as i64;
  visited.insert(v);
  count += 1;
  for &next in map.get(&v).unwrap() {
    if !visited.contains(&next) {
      count += dfs(next, map, visited);
    }
  }
  return count;
}


fn f() {
  input! {
    n: usize,
    s: String
  }
  let s_chars: Vec<char> = s.chars().collect();
  let mut max = 0;
  for i in 0..n {
    //println!("i: {}", s_chars[i]);
    if s_chars[i] == '/' {
      let mut count = 1;
      let mut index = 0;
      
      while index + i + 1 < n && i >= index+1 {
        index += 1;
        //println!("  before: {}, after: {}", s_chars[i - index], s_chars[i + index]);
        if s_chars[i - index] == '1' && s_chars[i + index] == '2' {
          count += 2;
        } else {
          break;
        }
      }
      
      if max < count {
        max = count;
      }

    }
  }
  println!("{}", max);
}


pub fn g() {
  input! {
    mut a: i64,
    mut b:i64,
    c: i64,
  }
  let mut set = HashSet::new();
  let mut mask = 1;
  let mut count = 0 as i64;
  for _ in 0..64 {    
    if (c & mask) != 0 {
      set.insert(mask);
      count += 1;
    }
    mask = mask << 1;
  }
  let mut k = a+b - count;

  if k >=0 && k %2 ==0 {
    let mut a_x = 0 as i64;
    let mut b_x = 0 as i64;
    if k == 0 {
      for v in &set {
        if a >0{
          a_x += *v;
          a -= 1;
        } else {
          b_x += *v;
          b -= 1;
        }
      }
      println!("{} {}", a_x, b_x);
    } else {
      let mut mask = 1;
      k = k /2;
      a = a-k;
      b = b-k;
      if a <0 || b <0 {
        println!("-1");
        return;
      }
      let mut a_x = 0 as i64;
      let mut b_x = 0 as i64;
      if(k+count) > 60 {
        println!("-1");
        return;
      }
      for _ in 0..64 {
        if (c & mask) == 0 {
          if k >0 {
            a_x += mask;
            b_x += mask;
          } else {
            break;
          }
          k -= 1;
        } 
        mask = mask << 1;
      }
      for v in &set {
        if a >0{
          a_x += *v;
          a -= 1;
        } else {
          b_x += *v;
          b -= 1;
        }
      }
      println!("{} {}", a_x, b_x);
    }
  }  else {
    println!("-1");
  }
}
