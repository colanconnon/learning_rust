use fib;

#[test]
fn it_calcs_10() {
    assert_eq!(55, fib::calc_fib(10));
}

#[test]
fn it_calcs_50() {
    let result = fib::calc_fib(50);
    assert_eq!(result, 12586269025);
}

#[test]
fn it_calcs_1() {
    let result = fib::calc_fib(1);
    assert_eq!(result, 1);
}
