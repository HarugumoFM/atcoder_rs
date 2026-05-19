use proconio::input;
use itertools::{Itertools,sorted};
use core::num;
use std::collections::{HashMap, HashSet, BTreeSet, BTreeMap, BinaryHeap};
use std::io::{self, BufRead};
use std::vec;
use std::cmp::{max, min};
use std::cmp::Reverse;
use std::collections::VecDeque;

pub fn d() {
    input!{
        x: usize,
        y: i64
    }
    let mut sum = 0;
    for i in 1..=6 {
        for j in 1..=6 {
            let mut sub = (i as i64 - j as i64).abs();
            //println!("{} {} {}", i, j, sub);
            if (i+j) >= x || sub >= y {
                sum += 1;
            }
        }
    }
    println!("{}", sum as f64/ 36.0);

}
 
pub fn e() {
    input!{
        n: usize,
        a:[usize;n],
        b:[i32;n],
    }
    let mut dic = HashMap::new();
    for i in 0..n {
        if !dic.contains_key(&a[i]) {
            dic.insert(a[i], Vec::new());
        }
        dic.get_mut(&a[i]).unwrap().push(b[i]);
    }
    let mut sum = 0 as i32;
    for (key, value) in &mut dic {
        let num = value.len();
        if num >1 {
            value.sort();
            for i in 0..num-1 {
                sum += value[i];
            }
        }
    }
    println!("{}", sum);
}

pub fn f() {
    input!{
        s:String,
        t:String
    }
    let mut len = s.len();
    let _s:Vec<char> = s.chars().collect();
    let _t:Vec<char> = t.chars().collect();
    
    let mut front_index = -1 as i32;
    let mut back_index = len as i32;
    
    for i in 0..len {
        let next_index = (front_index + 1) as usize;
        if _s[next_index] == _t[next_index] {
            front_index += 1;
        } else {
            break;
        }
    }

    for j in 0..len{
        let next_index = (back_index - 1) as usize;
        if _s[next_index] == _t[next_index+1] {
            back_index -= 1;
        } else {
            break;
        }
    }
    //println!("{} {}", front_index, back_index);
    if front_index == -1 {
        println!("{}", back_index+1);
    } else {
        println!("{}", front_index+2);
    }
}

pub fn g() {
     input!{
        n: usize,
        k: i64,
    }
    let mut res = Vec::new();
    let mut len = 1 as i64;
    res.push(k);
    let mut res1 = 0;
    for i in 0..n {
        let mut res2 = Vec::new();
        for i in &res {
            if i % 2 == 0 {
                res2.push(i/2);
                res2.push(i/2);
            } else {
                res1 = 1;
                res2.push(i/2);
                res2.push(i/2 + 1);
            }
        }
        res = res2;
    
    }
    println!("{}", res1);
    println!("{}", res.iter().join(" "));
}
