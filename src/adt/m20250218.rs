#![allow(non_snake_case)]
use proconio::input;
use itertools::{Itertools,sorted};
use std::collections::{HashMap, HashSet, BTreeSet};
use std::cmp::min;
use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;


pub fn e() {
    let mut vec =  vec![];
    input! {
        N:usize,
        M:usize,
    }
    for i in 0..N {
        input! {
            s:String
        }
        vec.push(s);
    }
    let permutations = vec.iter().permutations(N); 
    let mut res = false;
    for perm in permutations {
        //println!("{:?}", perm);
        let mut flag = true;
        for i in 0..N-1 {
            let a = perm[i].chars().collect::<Vec<char>>();
            let b = perm[i+1].chars().collect::<Vec<char>>();
            let mut count = 0;
            for j in 0..M {
                if a[j] != b[j] {
                    count += 1;
                }
            }
            if count != 1 {
                flag = false;
                break;
            }
        }
        if flag {
            res = true;
            break;
        }
    }

    if res {
        println!("Yes");
    } else {
        println!("No");
    }

}

pub fn f() {
    input! {
        N:usize,
    }

    input! {
        mut P:[i64;N],
    }
    let mut Q = vec![0;N];
    for i in 0..N {
        Q[P[i] as usize -1] = i as i64 +1;
    }

    println!("{}", Q.iter().map(|x| x.to_string()).join(" "));
}