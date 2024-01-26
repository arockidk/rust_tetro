pub fn factorial(n: i64) -> i64{
    let mut result = 1;

    for i in 1..n { 
        result *= i;
    }
    return result;
}