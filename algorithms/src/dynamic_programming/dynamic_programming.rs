/// # Examples
///
/// ```
/// use algorithms::dynamic_programming::fibonacci::fib;
///
/// let result = fib(5);
/// assert_eq!(result, 5);
/// ```

#[doc = "Returns the nth fibonacci number"]
pub(crate) fn fib(n: u32) -> u32 {
    // Generate a vector of fibonacci numbers
    let mut fibs: Vec<u32> = vec![0, 1];

    // Generate fibonacci numbers up to n
    for i in 2..=n {
        fibs.push(fibs[(i - 1) as usize] + fibs[(i - 2) as usize]);
    }

    // Return the nth fibonacci number
    fibs[n as usize]
}
