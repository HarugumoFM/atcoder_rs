use proconio::input;
use itertools::{Itertools,sorted};
use std::collections::{HashMap, HashSet, BTreeSet};
use std::cmp::min;
use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;

pub fn e() {
    input!{
        n:usize,
        k:i64,
        a:[i64;n]
    };
    let mut set = HashSet::new();
    let mut sum = ((1+k) * k) / 2;
    for x in a {
        set.insert(x);
    }
    for i in set {
        if i <=k {
            sum -= i;
        }
    }
    
    println!("{}", sum);
}

pub fn f() {
    input!{
        s: String
    };
    let _s:Vec<char> = s.chars().collect();
    let len = _s.len();
    let mut num_list = Vec::<(i32,i64)>::new();
    let mut last_char = _s[0];
    let mut last_len = 0 as i64;
    for i in 0..len {
        if last_char == _s[i] {
            last_len += 1;
        } else {
            let num = last_char as i32 - '0' as i32;
            num_list.push((num, last_len));
            last_char = _s[i];
            last_len = 1;
        }
    }
    num_list.push((last_char as i32 - '0' as i32, last_len));
    let list_len = num_list.len();
    if list_len == 1 {
        println!("0");
        return
    }
    let mut res = 0 as i64;
    for i in 1..num_list.len() {
        if num_list[i-1].0 + 1 == num_list[i].0 {
            res += min(num_list[i-1].1, num_list[i].1);
        }
    }
    println!("{}", res);
}


// timeout
pub fn g() {
    input!{
        n: usize,
        a:[i32;n],
    };
    let mut dic = HashMap::new();
    for i in 0..n-1 {
        input!{
            a:usize,
            b:usize,
        }
        if !dic.contains_key(&a) {
            dic.insert(a, HashSet::new());
        }
        if !dic.contains_key(&b) {
            dic.insert(b, HashSet::new());
        }
        dic.get_mut(&a).unwrap().insert(b);
        dic.get_mut(&b).unwrap().insert(a);
    }
    let mut result = vec![false;n];
    let mut visited = vec![false;n];
    let mut stack = HashSet::new();
    search(1, &mut result, &a, &mut stack, &dic, &mut visited, false);
    for i in 0..n {
        if result[i] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
fn search(n:usize, res:&mut Vec<bool>, a:&Vec<i32>, stack:&mut HashSet<i32>, dic:&HashMap<usize, HashSet<usize>>, visited:&mut Vec<bool>, is_success:bool) {
    let is_contain = stack.contains(&a[n-1]);
    res[n-1] = is_contain || is_success;
    //println!("{} {}", n, res[n-1]);
    visited[n-1] = true;
    if !is_contain {
        stack.insert(a[n-1]);
    }
    if dic.contains_key(&n) {
        let success = res[n-1];
        for i in dic.get(&n).unwrap().clone() {
            if !visited[i-1] {
                search(i, res, a, stack, dic, visited, success);
            }
        }
    }
    if !is_contain {
        stack.remove(&a[n-1]);
    }
}