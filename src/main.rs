use proconio::input;
use itertools::{Itertools,sorted};
use core::num;
use std::collections::{HashMap, HashSet, BTreeSet};
use std::cmp::min;
use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;

fn main() {
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
