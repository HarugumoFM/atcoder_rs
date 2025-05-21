use proconio::input;
use itertools::{Itertools,sorted};
use std::collections::{HashMap, HashSet, BTreeSet};
use std::cmp::min;
use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;

pub fn e() {
    input! {
        k: usize,
        s: String,
        t: String,
    }
    if(s == t) {
        println!("Yes");
        return;
    }
    let mut s = s.chars().collect::<Vec<_>>();
    let mut t = t.chars().collect::<Vec<_>>();
    let s_len = s.len() as i32;
    let t_len = t.len() as i32;
    if s_len - t_len == 1 {
        let mut s_index = -1;
        let mut t_index = t_len - 1;
        for i in 0..t_len {
            if s[i as usize] == t[i as usize] {
            s_index = i;
            } else {
            break;
            }
        }
        for i in 0..t_len {
            if s[(s_len - 1 - i) as usize] == t[(t_len - 1 - i) as usize] {
            t_index = t_len - 1 - i;
            } else {
            break;
            }
        }
        if t_index - s_index <= 1 {
            println!("Yes");
        } else {
            println!("No");
        }

    } else if s_len - t_len == -1 {
        let mut s_index = -1;
        let mut t_index = s_len - 1;
        for i in 0..s_len {
            if s[i as usize] == t[i as usize] {
            s_index = i;
            } else {
            break;
            }
        }
        for i in 0..s_len {
            if s[(s_len - 1 - i) as usize] == t[(t_len - 1 - i) as usize] {
            t_index = s_len - 1 - i;
            } else {
            break;
            }
        }
        //println!("s_index: {}, t_index: {}", s_index, t_index);
        if t_index - s_index <= 1 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else if s_len == t_len {
        let mut count = 0 as i32;
        for i in 0..s_len {
            if s[i as usize] != t[i as usize] {
            count += 1;
            }
        }
        if count == 1 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        println!("No");
    }
}
    
pub fn f() {
    input! {
        n: usize,
        t: usize,
        a: [i64; t],
      }
    let mut width = vec![0; n];
    let mut height = vec![0; n];
    let mut cross = 0;
    let mut cross2 = 0;
    for i in 0..t {
    let h = (a[i] - 1)/(n as i64);
    let w = (a[i] - 1) % (n as i64);
    height[h as usize] += 1;
    width[w as usize] += 1;
    if h == w {
        cross += 1;
    }
    if h + w == n as i64 - 1 {
        cross2 += 1;
    }
    //println!("{} {} {} {} {}", i, h, w, cross, cross2);
    if(height[h as usize] == n || width[w as usize] == n || cross == n || cross2 == n) {
        println!("{}", i + 1);
        return;
    }
    }
    println!("-1");
}


pub fn g() {
    input! {
        a: f64,
        b: f64,
      }
    let mut x = (a/2.0/b).powf(2.0/3.0)-1.0;
    let mut rounded = x.round();
    let mut ceil = x.ceil();
    let mut r1 = b*rounded + a/(1.0+rounded).powf(1.0/2.0);
    let mut r2 = b*ceil + a/(1.0+ceil).powf(1.0/2.0);
    let mut ans = if r1<=r2 { r1 } else { r2};
    println!("{}", ans);
}