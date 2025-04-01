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
        M: usize,
    }
    let mut map = HashMap::new();
    let mut visited = HashSet::new();
    for i in 0..M {
        input! {
            a: usize,
            b: usize,
        }
        if !map.contains_key(&(a-1)) {
            map.insert(a-1, HashSet::new());
        }
        if(!map.contains_key(&(b-1))) {
            map.insert(b-1, HashSet::new());
        }
        map.get_mut(&(a-1)).unwrap().insert(b-1);
        map.get_mut(&(b-1)).unwrap().insert(a-1);
    }
    let mut count = 0;
    for i in 0..N {
        if !visited.contains(&i) {
            count+=1;
            let res = search(&map, &mut visited, i);
            //println!("{} {}", i, res);
        }
    }

    println!("{}", count);
}

pub fn f() {
    input!{
        X:i64,
        A:i64,
        D:i64,
        N:i64,
    }
    if(D == 0) {
        println!("{}", (X - A).abs());
    } else if(D>0) {
        let last = A + D * (N - 1);
        if(X < A) {
            println!("{}", (X - A).abs());
        } else if(X > last) {
            println!("{}", (X - last).abs());
        } else {
            let mut left = 0;
            let mut right = N - 1;
            while left < right {
                let mid = (left + right) / 2;
                if A + D * mid < X {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            let ans1 = (X - (A + D * left)).abs();
            let ans2 = (X - (A + D * (left - 1))).abs();
            println!("{}", min(ans1, ans2));
        }
    } else {
        let last = A + D * (N - 1);
        if(X > A) {
            println!("{}", (X - A).abs());
        } else if(X < last) {
            println!("{}", (X - last).abs());
        } else {
            let mut left = 0;
            let mut right = N - 1;
            while left < right {
                let mid = (left + right) / 2;
                if A + D * mid > X {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            let ans1 = (X - (A + D * left)).abs();
            let ans2 = (X - (A + D * (left - 1))).abs();
            println!("{}", min(ans1, ans2));
        }
    }
}

fn search(
    map: &HashMap<usize, HashSet<usize>>,
    visited: &mut HashSet<usize>,
    current: usize,
) -> usize {
    if visited.contains(&current) {
        return 0;
    }
    visited.insert(current);
    let mut count = 1;
    if let Some(neighbors) = map.get(&current) {
        for &neighbor in neighbors {
            count += search(map, visited, neighbor);
        }
    }
    count
}
