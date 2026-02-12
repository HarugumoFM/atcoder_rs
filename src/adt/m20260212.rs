use proconio::input;
use itertools::{Itertools,sorted};
use std::collections::{HashMap, HashSet, BTreeSet};
use std::cmp::min;
use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;

pub fn e() {
  input! {
    s: String,
 }
 let _s = s.chars().collect::<Vec<char>>();
    let mut index = _s.len();
    let mut start = 0;
    let mut a_count = 0;
    let mut start_a_count = 0;
    for i in (0.._s.len()).rev() {
        if _s[i] == 'a' {
            index = i;
            a_count += 1;
        } else {
            break;
        }
    }
    for i in 0..index {
        if _s[i] == 'a' {
            start += 1;
            start_a_count += 1;
        } else {
            break;
        }
    }
    let mut is_same = true && (a_count >= start_a_count);
    for i in start..index {
        let mut _i = i-start;
        //println!("{} {}", _s[i], _s[index - 1 - _i]);
        if _s[i] != _s[index - 1 - _i] {
            is_same = false;
            break;
        }
    }
    if is_same {
        println!("Yes");
    } else {
        println!("No");
    }
}

// timeout
pub fn g() {
   input! {
    n: usize,
    m: usize,
  }
  let mut map = HashMap::<usize, usize>::new();
  let mut routes = HashMap::<usize, usize>::new();
  let mut groups = HashMap::<usize, usize>::new();
  
  for i in 0..m {
      input! {
          a: usize,
          b: usize,
      }
      if groups.contains_key(&a) {
          let group_id = groups.get(&a).unwrap().clone();
          if !groups.contains_key(&b) {
              *map.get_mut(&group_id).unwrap()+=1;
              groups.insert(b, group_id);
          }
          *routes.get_mut(&group_id).unwrap_or(&mut 0)+=1;
      } else if groups.contains_key(&b) {
          let group_id = groups.get(&b).unwrap().clone();
          groups.insert(a, group_id);
          *map.get_mut(&group_id).unwrap()+=1;
          *routes.get_mut(&group_id).unwrap_or(&mut 0)+=1;
      } else {
          if a == b {
              map.insert(a,1);
          } else {
              map.insert(a,2);
              groups.insert(b,a);
          }
          routes.insert(a,1);
          groups.insert(a,a);
      }
  }
  let mut related = true;
  for i in map.keys() {
      if map.get(i).unwrap() != routes.get(&i).unwrap() {
          related = false;
          break;
      }
  }
  if related  && groups.len() == n {
      println!("Yes");
  } else {
      println!("No");
  } 
  
}
