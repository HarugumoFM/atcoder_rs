use proconio::input;
use itertools::{Itertools,sorted};
use std::collections::{HashMap, HashSet, BTreeSet};
use std::cmp::min;
use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;

pub fn e() {
    input!{
        S: String,
        T: String,
    }
    let Sline = S.chars().collect::<Vec<char>>();
    let Tline = T.chars().collect::<Vec<char>>();
    let Slen = Sline.len();
    let Tlen = Tline.len();
    let mut index = -1 as i32;
    let mut index2 = (Slen-1) as i32;
    for i in 0..Slen{
        if(Sline[i] == Tline[i]){
        index = i as i32;
        
        } else {
        break;
        }
    }
    for i in 0..Slen{
        if(Sline[Slen-1-i] == Tline[Tlen-1-i]){
        index2 = (Slen-1-i) as i32;
        } else {
        break;
        }
    }
    //println!("{} {}", index, index2);
    if(index == -1) {
        println!("1");
    } else if(index2 == (Slen-1) as i32){
        println!("{}", Tlen);
    } else {
        println!("{}", index +2);
    }
 }

pub fn f() {
    input!{
        N:i64
    }
    let mut ans = N as i128;
    for i in 0..N {
        let mut Y = N - i*i;

        if(Y<0) {
        break;
        }
        let mut X = (Y as f64).sqrt() as i64;
        if(X<i) {
        break;
        }
        let _x = X as i128;
        let _d = N as i128;
        let _i = i as i128;
        ans = min(ans, (_i*_i + _x*_x - _d).abs());
        ans = min(ans, (_i*_i+(_x+1)*(_x+1) - _d).abs());
        if(_x>0) {
        ans = min(ans, (_i*_i + (_x-1)*(_x-1) - _d).abs());
        }
    }
    println!("{}", ans);
       
}
