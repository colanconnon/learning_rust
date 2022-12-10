use std::collections::HashMap;



fn fib(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    match memo.get(&n) {
        Some(&number) => number,
        None => {
            let fib = fib(n - 1, memo) + fib(n - 2, memo);
            memo.insert(n, fib);
            return fib;
        }
    }
}
fn main() {
    let mut memo: HashMap<u64, u64> = HashMap::from([(0, 0), (1, 1)]);
    println!("{}", fib(50, &mut memo));
}
