pub fn fibonacci_recursive(n: u32) -> u32 {
    if n == 1{
        0
    } else if n == 2 {
        1
    } else {
        fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
    }
}

pub fn fibonacci_iterative(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 1..n {
        let c = a + b;
        a = b;
        b = c;
    }
    a
}

