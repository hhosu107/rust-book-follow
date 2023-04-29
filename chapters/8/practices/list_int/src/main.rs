use std::io;
use std::collections::HashMap;

fn main() {
    // List of int.
    // Find medium, Find most occurred value.

    let mut vec: Vec<i32> = Vec::new();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.split_whitespace();
    for i in input {
        vec.push(i.parse::<i32>().unwrap());
    }
    vec.sort();
    let len: usize = vec.len();
    let mid_idx: usize = len / 2;
    let mid: i32 = *vec.get(mid_idx).unwrap();
    let mut hashmap = HashMap::new();
    for i in &vec {
        let count = hashmap.entry(i).or_insert(0);
        *count += 1;
    }
    let mut mode_cnt = 0;
    let mut mode = -1;
    for (k, v) in &hashmap {
        if *v > mode_cnt {
            mode_cnt = *v;
            mode = **k;
        }
    }
    println!("Mid: {mid}, Mode: {mode}");
}
