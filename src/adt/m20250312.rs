use proconio::input;
use itertools::{Itertools,sorted};
use std::collections::{HashMap, HashSet, BTreeSet};
use std::cmp::min;
use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;

pub fn e() {
    input!{
     N:usize,
     A:[i64;N],
    }
    let mut count:i64 = 0;
    let mut count2:i64 = 0;
    let mut set = HashSet::new();
    for i in 0..N {
         if(i as i64 == A[i] -1){
             count += 1;
         } else {
             if(A[A[i] as usize -1] == i as i64 +1  && !set.contains(&A[i])) {
                 set.insert(A[A[i] as usize -1]);
                 set.insert(A[i]);
                 count2+=1;
             }
         }
    }
    //println!("{}",count);
    println!("{}",count*(count-1)/2+count2);
    
 }

pub fn f() {
    input!{
        x1: i64,
        y1:i64,
        x2:i64,
        y2:i64,
       }
       let mut map = HashMap::new();
       distance(x1,y1,&mut map);
       let res = distance(x2,y2,&mut map);
       if res{
           println!("Yes");
       }else{
           println!("No");
       }
}


fn distance(x:i64, y:i64, set:&mut HashMap<i64, HashSet<i64>>)->bool {
    let mut x0:i64 = 0;
    let mut y0:i64 = 0;
    
    let mut insert = |x:i64,y:i64|->bool{
        if set.contains_key(&x){
            if set.get(&x).unwrap().contains(&y){
                return true;
            } else {
                set.get_mut(&x).unwrap().insert(y);
            }
        } else {
            let mut set2 = HashSet::new();
            set2.insert(y);
            set.insert(x,set2);
        }
        return false;
    };
    x0 = x+1;
    y0 = y+2;
    if insert(x0,y0){
        return true;
    }
    x0 = x+1;
    y0 = y-2;
    if insert(x0,y0){
        return true;
    }
    x0 = x-1;
    y0 = y+2;
    if insert(x0,y0){
        return true;
    }
    x0 = x-1;
    y0 = y-2;
    if insert(x0,y0){
        return true;
    }
    x0 = x+2;
    y0 = y+1;
    if insert(x0,y0){
        return true;
    }
    x0 = x+2;
    y0 = y-1;
    if insert(x0,y0){
        return true;
    }
    x0 = x-2;
    y0 = y+1;
    if insert(x0,y0){
        return true;
    }
    x0 = x-2;
    y0 = y-1;
    if insert(x0,y0){
        return true;
    }
    return false;
}