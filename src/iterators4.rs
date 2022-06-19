pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    // 6! = 6 * 5 * 4 * 3 * 2 * 1;
    let nums_iter = (1..=num).collect::<Vec<u64>>().iter().fold(1, |acc, x| acc * x);
    println!("{:?}", nums_iter);
    todo!();
}

fn main() {
    println!("{:?}", factorial(6));
    println!("{:?}", factorial(0));
}
