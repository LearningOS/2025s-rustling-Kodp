fn triangle(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..n {
        sum += i;
    }
    sum
}

use std::ffi::OsStr;
use std::path::Path;

fn main() {
    println!("{}", triangle(10));
    println!("There's");

    let v = vec!["antimony", "arsenic", "aluminum", "selenium"];

    // for ele in &v {
    //     println!("{}", ele);
    // }

    let mut iterator = (&v).into_iter();
    while let Some(ele) = iterator.next() {
        println!("{}", ele);
    }
    let v = vec![4, 20, 12, 8, 6];
    let mut iterator = v.iter();
    assert_eq!(iterator.next(), Some(&4));

    let path = Path::new("")
}