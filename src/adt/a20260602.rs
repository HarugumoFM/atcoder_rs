use proconio::input;
use itertools::{Itertools,sorted};
use core::num;
use std::collections::{HashMap, HashSet, BTreeSet, BTreeMap, BinaryHeap};
use std::io::{self, BufRead};
use std::vec;
use std::cmp::{max, min};
use std::cmp::Reverse;
use std::collections::VecDeque;
 
pub fn e() {
    input!{
        n:usize,
        k:usize,
    }
    let mut vec = VecDeque::new();
    let mut vec2 = Vec::new();
    let mut can_vec = VecDeque::new();
    for i in 0..n {
        let mut sum = 0 as i64;
        input! {
            a:[i64;3]
        }
        for j in 0..3 {
            sum += a[j];
        }
        vec.push_back(sum);
        vec2.push(sum);
        //println!("{}", sum);
        can_vec.push_back(false);
    }
    vec2.sort();
    vec2.reverse();
    let border = vec2[k-1];
    for i in 0..n {
        //println!("{} {}", vec[i], border);
        if vec[i] + 300 >= border {
            println!("Yes");
        } else {
            println!("No");
        }
    }

}
 
pub fn f() {
     input!{
        n:usize,
        q:usize,
    }
    let mut vec = VecDeque::new();
    for i in 1..=n {
        vec.push_back((i as i64, 0 as i64))
    }
    for j in 0..q {
        input!{
            _q:usize,
        }
        if _q == 1 {
            input!{
                c: char
            }
            let mut now = vec[0].clone();
            vec.pop_back();
            if c == 'U' {
                now.1 += 1;
            } else if c == 'D' {
                now.1 -=1;
            } else if c == 'R' {
                now.0 += 1;
            } else {
                now.0 -= 1;
            }
            vec.push_front(now);
        } else {
            input!{
                p: usize
            }
            println!("{} {}", vec[p-1].0, vec[p-1].1);
        }

    }

}
 

pub fn g() {
    input!{
        n:usize,
    }
    let mut vec = VecDeque::new();
    vec.push_front(1);
    for i in 0..n {
        println!("{}", vec.iter().join(" "));
        let mut vec2 = VecDeque::new();
        if i>0 {
            for j in 0..i {
                vec2.push_back(vec[j] + vec[j+1]);
            }
        }
        vec2.push_front(1);
        vec2.push_back(1);
        vec = vec2.clone();
    }

}
 
