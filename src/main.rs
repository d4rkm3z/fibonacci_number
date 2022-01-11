use std::{io, process};

fn gen_fibonacci(limit: usize) -> Option<u128> {
    let mut a: Vec<u128> = vec![0, 1, 1];

    if limit > 2 {
        for n in 3..limit + 1 {
            let n1 = a.get(n - 1).cloned().unwrap();
            let n2 = a.get(n - 2).cloned().unwrap();
            let sum = n1 + n2;

            a.push(sum);
        }
    }

    return a.get(limit).copied();
}

fn main() {
    let mut limit = String::new();

    println!("Enter below your number:");

    io::stdin()
        .read_line(&mut limit)
        .unwrap();


    let limit = limit.trim().parse().unwrap();
    let result = gen_fibonacci(limit).unwrap();

    println!("For {} fibonacci sum will {}", limit, result);
}