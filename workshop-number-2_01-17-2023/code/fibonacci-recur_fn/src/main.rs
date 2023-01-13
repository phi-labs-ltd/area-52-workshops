use recur_fn::{recur_fn, RecurFn};

fn main() {
    let length: i32 = 45;
    let mut i: i32 = 1;

    let fib = recur_fn(|fib, n: i32| {
        if n <= 1 {
            n
        } else {
            fib(n - 1) + fib(n - 2)
        }
    });

    while i < length {
        let result = fib.call(i);
        i += 1;
        println!("{:?}: {:?}", i, result);
    }
}