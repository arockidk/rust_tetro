pub fn factorial(n: i64) -> i64{
    let mut result = 1;

    for i in 1..n + 1{ 
        result *= i;
    }
    return result;
}
pub fn usize_factorial(n: usize) -> usize {
    let mut result = 1;
    for i in 1..n + 1 {
        result *= i;
    }
    return result;
}