
pub fn test_error() -> (bool, i32) {
    let good_result: Result<i32, i32> = Ok(10);
    let bad_result: Result<i32, i32> = Err(10);

    let good_result: Result<i32, i32> = good_result.map(|i| i + 1);
    let bad_result: Result<i32, i32> = bad_result.map(|i| i - 1);

    let good_result: Result<bool, i32> = good_result.and_then(|i| Ok(i == 11));

    let final_awesome_result: bool = good_result.unwrap();
    let final_error_result = bad_result.unwrap_or(3);

    return (final_awesome_result, final_error_result);
}