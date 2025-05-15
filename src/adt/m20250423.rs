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
        p: [usize; n],
        q: [usize; n],
    }
    let mut ans = vec![0;n];
    for i in 0..n {
        let num = q[i];
        ans[num-1] = q[p[i] -1];
    }
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}

pub fn f() {
    input! {
        n: usize,
  }
  let mut x = HashMap::<i64, HashSet<i64>>::new();
  for i in 0..n {
      input! {
          a: i64,
          b: i64,
      }
      x.entry(a).or_insert(HashSet::new()).insert(b);
      x.entry(b).or_insert(HashSet::new()).insert(a);
  }
  let visited = HashSet::new();
  let max = dfs(1, &mut visited.clone(), &x);
  println!("{}", max);
}

fn dfs(x: i64, visited: &mut HashSet<i64>, x_map: &HashMap<i64, HashSet<i64>>) -> i64 {
   //println!("x: {}", x);
   if( visited.contains(&x)) {
       return x;
   }
   visited.insert(x);
   let mut ans = 0;
   if(x_map.get(&x).is_none()) {
       return x;
   }
   for &y in x_map.get(&x).unwrap() {
       let num = dfs(y, visited, x_map);
       ans = max(ans, num);
   }
   max(ans,x)
}