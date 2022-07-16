//! A library crate with the sole purpose of building out a solution 
//! to the 'string_to_integer, atoi' LeetCode problem. This was a 
//! particularly performant solution, though a bit messily coded.



/// An empty struct with implementation solving the my_atoi leet code 
/// problem.
/// 
/// # Example
/// ```
/// use string_to_integer::*;
/// let result = Solution::my_atoi("   -000000000001".to_string());
/// assert_eq!(result, -1);
/// ```
pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut first_nonzero = -1;
        let mut had_numeric: bool = false;
        let mut pos = true;
        let mut digits: String = String::new();
        let trimmed = s.trim_start();
        let mut itr = trimmed.chars();
        if let Some(x) = itr.next() {
            if x == '-' {
                pos = false;
            }
            else if x == '+' {

            }
            else {
                if x.is_numeric() {
                    if x != '0' && first_nonzero == -1 { first_nonzero = 0 }
                    digits.push(x);
                    had_numeric = true;
                }
                else {
                    return 0;
                }
            }
        }
        
        for c in itr {
            if c.is_numeric() {
                if c != '0' && first_nonzero == -1 { first_nonzero = digits.len() as i32 }
                digits.push(c);
                had_numeric = true;
            }
            else {
                if had_numeric {
                    break;
                }
                else {
                    return 0;
                }
            }
        }
        if digits.len() as i32 - first_nonzero > 20 {
            if pos == true {
                return i32::MAX;
            }
            else {
                return i32::MIN;
            }
        }
        let unsigned_result:i128 = match digits.parse() {
            Ok(x) => x,
            Err(_e) => 0,
        };
        let mut signed_result = unsigned_result;
        if pos == false {
            signed_result *= -1;
        }
        signed_result.clamp(i32::MIN as i128, i32::MAX as i128) as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = Solution::my_atoi("20000000000000000000".to_string());
        assert_eq!(result, i32::MAX);
    }
}
