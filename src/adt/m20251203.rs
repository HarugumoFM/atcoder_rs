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
    q: usize,
  }
  let mut dic = HashMap::new();
  let mut sum_index = 0;
  for _i in 0..n {
    dic.insert(_i, _i as i64 + 1);
  }
  for _ in 0..q {
    input! {
      t: usize,
    }
    if t == 1 {
      input! {
        p: usize,
        x: i64,
      }
      let index = (p + sum_index -1) % n;
      dic.entry(index).and_modify(|e| *e = x).or_insert(x);
    } else if t == 2 {
      input! {
        p: usize,
      }
      let index = (p + sum_index -1) % n;
      println!("{}", dic.get(&index).unwrap_or(&0));
    } else {
      input! {
        k: usize,
      }
      sum_index =  (sum_index+k%n) % n;
    }
  }
}




fn f() {
  input! {
    n: usize,
    a: [i64; n],
  }
  let mut odds = vec![];
  let mut evens = vec![];
  for &x in &a {
    if x % 2 == 0 {
      evens.push(x);
    } else {
      odds.push(x);
    }
  }
  odds.sort();
  evens.sort();
  let mut ans = -1 as i64;
  if odds.len() >= 2 {
    odds.reverse();
    ans = max(ans, odds[0] + odds[1]);
  }
  if evens.len() >= 2 {
    evens.reverse();
    ans = max(ans, evens[0] + evens[1]);
  }
  println!("{}", ans);
}

pub fn g() {
 input! {
    h: usize,
    w: usize,
  }
  let mut grid = vec![vec!['.';w];h];
  let mut visit = vec![vec![false;w];h];
  let mut _max = 0;
  for i in 0..h {
    let s: String;
    input! {
      s: String,
    }
    for (j,c) in s.chars().enumerate() {
      if c == '#' {
        grid[i][j] = '#';
        visit[i][j] = true; 
      }
    }
  }
  for i in 0..1 {
    for j in 0..1 {
      if grid[i][j] == '.' && !visit[i][j] {
        let res = dfs(i,j,h,w,&mut grid,&mut visit,0);
        _max = max(_max,res);
      }
    }
  }
  println!("{}",_max);
}
fn dfs(i:usize,j:usize,h:usize,w:usize,grid:&mut Vec<Vec<char>>,visit:&mut Vec<Vec<bool>>, count: usize) -> usize {
  let directions = vec![(0,1),(1,0)];
  if visit[i][j] {
    return count;
  }
  visit[i][j] = true;
  let mut _count = count + 1;
  let mut _res = _count;
  for (dx,dy) in directions.iter() {
    let ni = i as isize + dx;
    let nj = j as isize + dy;
    if ni >= 0 && ni < h as isize && nj >= 0 && nj < w as isize {
      let ni = ni as usize;
      let nj = nj as usize;
      if !visit[ni][nj] && grid[ni][nj] == '.' {
        _res = max(_res,dfs(ni,nj,h,w,grid,visit,_count));
      }
    }
  }
  return _res;
}