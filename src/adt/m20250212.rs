#![allow(non_snake_case)]
use proconio::input;
use itertools::{Itertools,sorted};
use std::collections::{HashMap, HashSet, BTreeSet};
use std::cmp::min;
use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;

pub fn e() {
    input! {
        N:usize,
        M:usize,
    }
    input! {
        mut A:[i64;N],
        mut B:[i64;M],
    }
    let min: i64 = *A.iter().min().unwrap();
    let max: i64 = *B.iter().max().unwrap();
    let mut eats = vec![-1; (max +1) as usize];
    let mut index = max;
    let mut index2 = 1;
    for i in A.iter(){
        if(*i <= index) {
            for j in *i..=index {
                eats[j as usize] = index2;
            }
            //println!("{} {}", *i, index);
            index = *i-1;
        }
        index2+=1;
    }
    for x in B.iter() {
        println!("{}", eats[*x as usize]);
    }
}

pub fn f() {
    input! {
        N:usize,
    }
    input! {
        mut A:[i64;N],
    }
    let mut visited = vec![false;N];
    let mut queue:Vec<usize> = Vec::new();
    let mut set:HashSet<usize> = HashSet::new();
    for i in 0..N {
        if(!visited[i]) {
            let res = search(i+1, &mut visited, &mut queue, &mut set,&mut A);
            set.clear();
            queue.clear();
            if res {
                break;
            }
        }
    }
    fn search(x:usize, mut V:&mut Vec<bool>,mut Q:&mut Vec<usize>, mut S:&mut HashSet<usize> ,mut R:&mut [i64]) -> bool {
        if(S.contains(&x)) {
            let mut queue2 = vec![];
            let mut flag = false;
            for i in Q.iter() {
                if *i == x {
                    flag = true;
                }
                if flag {
                    queue2.push(i);
                }
            }
            println!("{}",queue2.len());
            let output = queue2.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" ");
            println!("{}",output);
            return true;
        } else {
            if V[x-1] {
                return false;
            }
            //println!("{}",x);
            V[x-1] = true;
            S.insert(x as usize);
            Q.push(x as usize);
        }
        
        let res = search(R[x-1] as usize, &mut V, &mut Q, &mut S, &mut R);
        return res || false;
    }
}