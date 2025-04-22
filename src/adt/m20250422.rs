use proconio::input;
use itertools::{Itertools,sorted};
use std::collections::{HashMap, HashSet, BTreeSet};
use std::cmp::min;
use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;

pub fn e() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    }
    let mut routes = HashMap::new();
    for i in 0..n-1 {
        input! {
            a: usize,
            b: usize,
        }
        routes.entry(a).or_insert(vec![]).push(b);
        routes.entry(b).or_insert(vec![]).push(a);
    }
    let mut visited = HashSet::new();
    search(x,y,&mut vec![], &mut routes, &mut visited);

}

pub fn search(x: usize, y: usize , stack: &mut Vec<usize>, route: &mut HashMap<usize, Vec<usize>>, visited: &mut HashSet<usize>) {
    if visited.contains(&x) {
        return;
    }
    stack.push(x);
    visited.insert(x);
    
    if x == y {
        println!("{}", stack.into_iter().join(" "));
        return;
    } else {
        let neighbors = route.get(&x).unwrap();
        let mut neighbors = neighbors.clone();
        for i in neighbors {
            if !visited.contains(&i) {
                search(i, y, stack, route, visited);
            }
        }
    }
    

    stack.pop();
}




pub fn f() {
    input! {
        n: usize,
        mut w: i64,
    }
    let mut cheeses = vec![];
    for _ in 0..n {
        input! {
            a: i64,
            b: i64,
        }
        cheeses.push(Cheese { a, b });
    }
    cheeses.sort_by(|a, b| b.a.cmp(&a.a));
    
    let mut ans = 0 as i64;
    for c in &cheeses {
        //println!("cheese:{} {} {}", c.a, c.b, w);
        if c.b > w {
            ans += c.a * w;
            break;
        } else {
            ans += c.a * c.b;
            w -= c.b;
        }
    }
    println!("{}", ans);
}

struct Cheese {
    pub a: i64,
    pub b: i64,
}

pub fn g() {
    let mask = 998244353 as i64;
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }
    let mut last_start = 0 as i64;
    let mut last_vec = vec![];
    let mut last_end = 0 as i64;
    last_vec.push(1);
    for i in 0..n {
        let mut new_vec = vec![0 as i64; (b[i] - a[i] + 1) as usize];
        for j in a[i]..=b[i] {
            //println!("{} {}", i, j);
            if(j>= last_end) {
                new_vec[(j-a[i]) as usize] = last_vec[(last_end-last_start) as usize]%mask;
            } else if (j >= last_start) {
                new_vec[(j-a[i]) as usize] = last_vec[(j-last_start) as usize]%mask;
            } else {
                new_vec[(j-a[i]) as usize] = 0;
            }
            //println!("{}", new_vec[(j-a[i]) as usize]);
            new_vec[(j-a[i]) as usize] = (new_vec[(j-a[i]) as usize])%mask;
            if(j-a[i] > 0) {
                new_vec[(j-a[i]) as usize] += new_vec[(j-a[i]-1) as usize];
                new_vec[(j-a[i]) as usize] = (new_vec[(j-a[i]) as usize])%mask;
            }
            //println!("{} {} {}", i, j, new_vec[(j-a[i]) as usize]);
        }
        last_vec = new_vec;
        last_start = a[i];
        last_end = b[i];
    }
    println!("{}", last_vec[(last_end-last_start) as usize]);
}

