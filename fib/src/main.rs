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

fn calc_fib(n: u64) -> u64 {
    let mut memo: HashMap<u64, u64> = HashMap::from([(0, 0), (1, 1)]);
    return fib(n, &mut memo);
} 
fn main() {
    println!("{}", calc_fib(50));
}


#[cfg(test)]
mod tests {
    use crate::calc_fib;

    #[test]
    fn it_calcs_10() {
        let result = calc_fib(10);
        assert_eq!(result, 55);
    }

    #[test]
    fn it_calcs_50() {
        let result = calc_fib(50);
        assert_eq!(result, 12586269025);
    }

    #[test]
    fn it_calcs_1() {
        let result = calc_fib(1);
        assert_eq!(result, 1);
    }
}

