fn fib(n: i32) -> i32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

fn main() {
    let length: i32 = 45;
    let mut i: i32 = 0;
    while i < length {
        let result: i32 = fib(i);
        i += 1;
        println!("{:?}: {:?}", i, result);
    }
}