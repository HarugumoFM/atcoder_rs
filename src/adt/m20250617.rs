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
    x: i64,
    a: [i64; n],
   }
   let mut set = HashSet::new();
    for i in 0..n {
        set.insert(a[i]);
    }
    let mut res = false;
    for num in set.iter() {
        if set.contains(&(*num + x)) || set.contains(&(*num - x)) {
            res = true;
            break;
        }
    }
    if res {
        println!("Yes");
    } else {
        println!("No");
    }
}

pub fn f() {
   input! {
    n: usize,
    h: [i64; n],
   }
   let mut map1 = HashMap::new();
   for i in 0..n {
      if map1.contains_key(&h[i]) {
         let v = map1.get_mut(&h[i]).unwrap();
         *v += 1;
      } else {
         map1.insert(h[i], 1);
      }
   }
   let max = *map1.values().max().unwrap();
   //println!("max: {}", max);
   let mut max_count = 1;
   for i in 0..n-1 {
      for j in 1..n {
         if i+j >= n {
            break;
         }
         if h[i] == h[i+j] {
            let mut index = i+j;
            let mut count = 1; 
            while index < n {
               if h[index] == h[i] {
                count +=1;
               } else {
                  break;
               }
                index += j;
            }
            if count > max_count {
               max_count = count;
            }
            //println!("num: {} i: {}, j: {}, count: {}", h[i],i, j, count);
          }
      }
   }
    println!("{}", max_count);
}


pub fn g() {
  input! {
     n: usize,
     x: i64,
     y: i64,
     a: [i64; n],
   }
   let mut setX = HashSet::new();
   let mut setY = HashSet::new();
   setX.insert(0);
   setY.insert(0);
   for i in 0..n {
     if i == 0 {
      let mut newSetX = HashSet::new();
      newSetX.insert(a[i]);
      setX = newSetX;
     } else if i % 2 == 0 {
      let mut newSetX = HashSet::new();
      for &x in setX.iter() {
        newSetX.insert(x + a[i]);
        newSetX.insert(x - a[i]);
      }
      setX = newSetX;
     } else {
      let mut newSetY = HashSet::new();
      for &y in setY.iter() {
        newSetY.insert(y + a[i]);
        newSetY.insert(y - a[i]);
      }
      setY = newSetY;
     }
   }
   if setX.contains(&x) && setY.contains(&y) {
     println!("Yes");
   } else {
     println!("No");
   }

}