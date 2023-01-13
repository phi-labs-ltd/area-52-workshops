use recur_fn::{recur_fn, RecurFn};

fn main() {
    let length: i32 = 45;
    let mut i: i32 = 0;

    let fib = recur_fn(|fib, n: i32| {
        if n <= 1 {
            n
        } else {
            fib(n - 1) + fib(n - 2)
        }
    });

    while i < length {
        i += 1;
        let result: i32 = fib.call(i);
        println!("{:?}: {:?}", i, result);
    }
}