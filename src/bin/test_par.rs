use rayon::prelude::*;

fn main() {
    let arr = (1..1000).into_par_iter().find_any(|n| {
        println!("n {}", n);
        n % 11 == 0
    });
    println!("find any n: {}", arr.unwrap());

}