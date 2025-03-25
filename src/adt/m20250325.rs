use proconio::input;
use itertools::{Itertools,sorted};
use std::collections::{HashMap, HashSet, BTreeSet};
use std::cmp::min;
use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;

pub fn e() {
    input! {
        N: i64,
      }
      let mut set = BTreeSet::new();
      set.insert(0);
      let mut index = 1 as i64;
      for i in 0..=60 {
        if N & index != 0 {
          let mut new_set = BTreeSet::new();
          for v in set.iter() {
            new_set.insert(*v);
            new_set.insert(*v + index);
          }
          set = new_set;
        }
        index *= 2;
      }
      for i in set.iter() {
        println!("{}", i);
      }
 }

pub fn f() {
    input! {
        N: usize,
        M: usize,
        A: [i64; N],
        B: [i64; M],
    }
    let mut count = 1 as usize;
    let mut n_index = 0;
    let mut m_index = 0;
    let mut Ai = vec![];
    let mut Bi = vec![];
    while n_index < N || m_index < M {
        if n_index == N {
            Bi.push(count);
            m_index += 1;
        } else if m_index == M {
            Ai.push(count);
            n_index += 1;
        } else if A[n_index] > B[m_index] {
            Bi.push(count);
            m_index += 1;
        } else {
            Ai.push(count);
            n_index += 1;
        }
        count += 1;
    }
    for i in 0..N {
        if i == 0 {
            print!("{}", Ai[i]);
        } else {
            print!(" {}", Ai[i]);
        }
    }
    println!("");
    
    for i in 0..M {
        if i == 0 {
            print!("{}", Bi[i]);
        } else {
            print!(" {}", Bi[i]);
        }
    }
    println!("");   
}
