use proconio::input;
use itertools::{Itertools,sorted};
use std::collections::{HashMap, HashSet, BTreeSet};
use std::cmp::min;
use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;



pub struct M20250204;

impl M20250204 {
    pub fn f() {
        input!{
            N:i32,
            M:i32,
        }
        let mut visited = vec![false;N as usize];
        let mut routes = HashMap::<i32,HashSet<i32>>::new();
        let mut S = 0;
        for _ in 0..M {
            input!{
                a:i32,
                b:i32,
            }
            if !routes.contains_key(&a) {
                let mut set = HashSet::new();
                routes.insert(a,set);
            }
            if !routes.contains_key(&b) {
                let mut set = HashSet::new();
                routes.insert(b,set);
            }
            routes.get_mut(&a).unwrap().insert(b);
            routes.get_mut(&b).unwrap().insert(a);
        }
        for i in 0..N {
            if search(i+1, &mut visited, &routes) {
                S += 1;
            }
        }
    
        fn search(x:i32, visited: &mut Vec<bool>, routes: &HashMap<i32, HashSet<i32>>)->bool {
            if(visited[(x-1) as usize]) {
                return false;
            }
            visited[(x-1) as usize] = true;
            if routes.contains_key(&x) {
                for i in routes.get(&x).unwrap() {
                    search(*i, visited, routes);
                }
            }
            return true;
        }
       
        println!("{}", max(M- N + S ,0));
    
    }
}