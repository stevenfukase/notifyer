#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0 {
        return Err(DivisionError::DivideByZero);
    }
    let result = a / b;
    if a % b != 0 {
        return Err(DivisionError::NotDivisible(NotDivisibleError {
            dividend: a,
            divisor: b,
        }));
    }

    Ok(result)
}

// Complete the function and return a value of the correct type so the test passes.
// Desired output: Ok([1, 11, 1426, 3])
fn result_with_list() -> Result<Vec<i32>, DivisionError> {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers
        .into_iter()
        .map(|n| divide(n, 27).unwrap())
        .collect();

    Ok(division_results)
}

// Complete the function and return a value of the correct type so the test passes.
// Desired output: [Ok(1), Ok(11), Ok(1426), Ok(3)]
fn list_of_results() -> Vec<Result<i32, DivisionError>> {
    let numbers = vec![27, 297, 38502, 81];
    numbers.into_iter().map(|n| divide(n, 27)).collect()
}

fn main() {
    let res = list_of_results();
    println!("{:?}", res);
}
