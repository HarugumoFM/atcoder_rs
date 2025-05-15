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
        m: usize,
    }
    let mut a = vec![false; n];
    let mut route: HashMap<usize, HashSet<usize>> = HashMap::new();
    for i in 0..m {
        input! {
          x: usize,
          y: usize,
        }
        if(!route.contains_key(&(x-1))) {
            route.insert(x-1, HashSet::new());
        } 
        if(!route.contains_key(&(y-1))) {
            route.insert(y-1, HashSet::new());
        }
        route.get_mut(&(x-1)).unwrap().insert(y-1);
        route.get_mut(&(y-1)).unwrap().insert(x-1);
    }
    let mut count = 0;
    for i in 0..n {
        if !a[i] {
            dfs(&route, &mut a, i);
            count += 1;
        }
    }
    println!("{}", count);
}
    
pub fn dfs(route: &HashMap<usize, HashSet<usize>>, a: &mut Vec<bool>, x: usize) {
    a[x] = true;
    if !route.contains_key(&x) {
      return;
    }
    for i in route.get(&x).unwrap() {
      if !a[*i] {
            dfs(route, a, *i);
        }
    }
}

pub fn f() {
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
    for i in 0..n {
        if set.contains(&(a[i] - x)) {
            res = true;
            break;
        }else if set.contains(&(x + a[i])) {
            res = true;
            break;
        }
    }
    if res {
        println!("Yes");
    }else {
        println!("No");
    }
}


pub fn g() {
    input! {
        n: usize,
        m: usize,
        p: i64,
        mut a: [i64; n],
        mut b: [i64; m],
    }
    a.sort();
    b.sort();
    let mut sums = vec![0; m+1];
    let mut sum = 0;
    for i in 0..m {
        sum += b[i];
        sums[i+1] = sum;
    }
    let mut ans = 0;
    let mut index = (m-1) as i64;
    for i in 0..n {
        while index>=0 && a[i] + b[index as usize] > p {
          index -= 1;
        }
        //println!("{} {} {}", a[i], index, sums[index as usize+1]);
        if index <0 {
          ans += p* m as i64;
        } else {
          ans += p*(m as i64 -index-1) + sums[index as usize+1] + a[i] * (index+1) as i64;
        }
    }
    println!("{}", ans);
}