// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: i32,
        mut input_vec: [i32; n],
    }
    let mut count: i32 = 0;
    let input_vec_size: usize = input_vec.iter().count();
    'outerloop: loop {
        for i in 0..input_vec_size {
            if input_vec[i]%2==1 || input_vec[i]==0 {
                break 'outerloop;
            }
            input_vec[i] >>= 1;
        }
        count+=1;
        continue;
    }
    println!("{}", count);
}
