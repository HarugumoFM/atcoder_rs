use proconio::input;
use itertools::{Itertools,sorted};
use std::collections::{HashMap, HashSet, BTreeSet};
use std::cmp::min;
use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;

pub fn e() {
   input! {
      h: usize,
      w: usize,
      n: usize,
      s: String
    }
    let mut a = vec![];
    for i in 0..h {
        input! {
            x: String
        }
       a.push(x.chars().collect::<Vec<char>>());
    }
    let root = s.chars().collect::<Vec<char>>();
    let mut minx = 0 as i64;
    let mut maxx = 0 as i64;
    let mut miny = 0 as i64;
    let mut maxy = 0 as i64;
    let mut x = 0 as i64;
    let mut y = 0 as i64;
    for i in 0..n {
        if root[i] == 'U' {
            y -= 1;
        } else if root[i] == 'D' {
            y += 1;
        } else if root[i] == 'L' {
            x -= 1;
        } else if root[i] == 'R' {
            x += 1;
        }
        if x < minx {
            minx = x;
        }
        if x > maxx {
            maxx = x;
        }
        if y < miny {
            miny = y;
        }
        if y > maxy {
            maxy = y;
        }
    }
    //println!("minx: {}, maxx: {}, miny: {}, maxy: {}", minx, maxx, miny, maxy);
    let mut count = 0;
    for j in 0..h {
        for i in 0..w {
            if j as i64 + miny >= 0 && j as i64 + maxy < h as i64 && i as i64 + minx >= 0 && i as i64 + maxx < w as i64 {
                count +=search(i,j, h, w, &a, &root);
            }
        }
    }
    println!("{}", count);

}

fn search(x: usize, y: usize, h: usize, w: usize, a: &Vec<Vec<char>>, s: &Vec<char>) -> usize {
    let mut i = x;
    let mut j = y;
    for n in s {
        if(a[j][i] != '.') {
            return 0;
        }
        if *n == 'U' {
            j -= 1;
        } else if *n == 'D' {
            j += 1;
        } else if *n == 'L' {
            i -= 1;
        } else if *n == 'R' {
            i += 1;
        }
    }
    if(a[j][i] != '.') {
        return 0;
    }
    //println!("x: {}, y: {}, rx: {}, ry: {}", x, y, i, j);
    return 1;
}

pub fn f() {
   input! {
      n: usize,
    }
    let mut x = vec![vec![0 as i64; 2];n];
    let mut mins = HashMap::new();
    let mut m = 0 as i64;
    for i in 0..n {
        input! {
            a: i64,
            b: i64,
        }
        x[i][0] = a;
        x[i][1] = b;
        if mins.contains_key(&b) {
            let mut v = mins.get_mut(&b).unwrap();
            *v = min(*v, a);
        } else {
            mins.insert(b, a);
        }

    }
    let mut ans = 0;
    for i in mins {
       ans = max(ans, i.1);
    }
    println!("{}", ans);
}

