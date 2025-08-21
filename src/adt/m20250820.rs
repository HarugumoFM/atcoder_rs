use proconio::input;
use itertools::{Itertools,sorted};
use std::collections::{HashMap, HashSet, BTreeSet};
use std::cmp::min;
use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;

pub fn f() {
   input! {
    n:usize,
  }
  let mut vec = Vec::new();
  let mut nums = [0,2,4,6,8];
  if n == 1 {
    println!("0");
  } else {
    let mut num = n-1;
    while(true) {
      if num < 5 {
        vec.push(nums[num]);
        break;
      } else {
        vec.push(nums[num % 5]);
      }
      num /= 5;
    }
  }
  vec.reverse();
  for i in 0..vec.len() {
    print!("{}", vec[i]);
  }
  println!();
}


//this solution is timeout
fn g() {
  input! {
    n:usize,
    k:usize,
    p:[usize;n]
  }
  let mut vanishes = vec![-1 as i32; n];
  let mut decks = Vec::new();
  for i in 0..n {
    if decks.len() == 0{
      let mut vec = Vec::new();
      if(k == 1) {
        vanishes[p[i] -1] = (i+1) as i32;
      } else {
      vec.push(p[i]);
      decks.push(Deck {
        cards: vec,
        min: p[i],
        count: 1
      });
    }
    } else {
      let mut found = false;
      //2分探索でp[i]以上の最小のminを持つDeckを探す
      let mut low = 0;
      let mut high = decks.len();
      while low < high {
        let mid = (low + high) / 2;
        if decks[mid].min >= p[i] {
          high = mid;
        } else {
          low = mid + 1;
        }
      }
      if low < decks.len() {
        decks[low].cards.push(p[i]);
        decks[low].count += 1;
        decks[low].min = p[i];
        if decks[low].count == k {
          for card in &decks[low].cards {
            vanishes[*card -1 ] = (i + 1) as i32;
          }
          decks.remove(low);
        }
      } else {
        let mut vec = Vec::new();
        vec.push(p[i]);
        decks.push(Deck {
          cards: vec,
          min: p[i],
          count: 1
        });
      }

    }
    
  }
  for i in &vanishes {
      println!("{}", i);
    }
}

struct Deck {
  pub cards: Vec<usize>,
  pub min: usize,
  pub count: usize
}



