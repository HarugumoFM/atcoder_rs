use proconio::input;
use itertools::{Itertools,sorted};
use std::collections::{HashMap, HashSet, BTreeSet};
use std::cmp::min;
use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;

pub fn e() {
    input! {
        N: usize,
        Q: usize,
        S: String,
    }
    let mut S: Vec<char> = S.chars().collect();

    let mut start = 0 as usize;
    for i in 0..Q {
        input! {
            t: usize,
            x: usize
        }
        if t==1 {
            if(x>start) {
                start = start + N - x;
            } else {
                start = start -x;
            }
        } else {
            let mut x = (start + x -1) % N;
            println!("{}", S[x]);
        }
    }
}

pub fn f() {
    input! {
        H: usize,
        W: usize,
    }
    let mut A =vec![];
    for _ in 0..H {
        input! {
            a: String,
        }
        let mut a = a.chars().collect::<Vec<char>>();
        A.push(a);
    }
    let mut x:usize = 0;
    let mut y:usize = 0;
    let mut visited = vec![vec![false; W]; H];

    while(true) {
        if(visited[y][x]) {
            println!("-1");
            return;
        }
        visited[y][x] = true;
        if A[y][x] == 'R' {
            if(x < W-1) {
                x += 1;
            } else {
                println!("{} {}", y+1, x+1);
                break;
            }
        } else if A[y][x] == 'U' {
            if y > 0 {
                y -= 1;
            } else {
                println!("{} {}", y+1, x+1);
                break;
            }
            
        } else if A[y][x] == 'D' {
            if y < H-1 {
                y += 1;
            } else {
                println!("{} {}", y+1, x+1);
                break;
            }
        } else {
            if x > 0 {
                x -= 1;
            } else {
                println!("{} {}", y+1, x+1);
                break;
            }
        }
    }
}

