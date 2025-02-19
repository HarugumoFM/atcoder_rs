use proconio::input;
use itertools::{Itertools,sorted};
use std::collections::{HashMap, HashSet, BTreeSet};
use std::cmp::min;
use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;


pub fn e() {
    input! {
        mut N:usize,
    }
    let mut res:String = String::new();
    N= N-1;
    if( N == 0) {
        print!("0");
        return;
    }
    while(N != 0) {
        let sub = N % 5;
        res = ((sub * 2).to_string()) + &res;
        N = N / 5;
    }
    print!("{}", res);
}

pub fn f() {
    input! {
        mut N:usize,
    }
    let mut A:HashMap<i64, HashSet<i64>> = HashMap::new();
    let mut B = vec![0;10];
    for i in 0..10 {
        A.insert(i,HashSet::new());
    }
    for i in 0..N {
        input! {
            mut S:String,
        }
        let chars = S.chars().collect::<Vec<char>>();
        for j in 0..10 {
            let num = chars[j].to_digit(10).unwrap() as i64;
            let mut index = j as i64;
            while (A.get_mut(&num).unwrap().contains(&index)) {
                index+=10;
            } 
            A.get_mut(&num).unwrap().insert(index);
            if(index > B[num as usize]) {
                B[num as usize] = index;
            }
        }
    }
    println!("{:?}", B.iter().min().unwrap());
}