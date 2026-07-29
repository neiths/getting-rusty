struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        // Collect bytes or working slice to avoid repeated Unicode parsing overhead
        let bytes = s.as_bytes();
        if bytes.is_empty() {
            return true;
        }

        let mut left = 0;
        let mut right = bytes.len() - 1;

        while left < right {
            // Advance left pointer if current byte is not alphanumeric
            if !bytes[left].is_ascii_alphanumeric() {
                left += 1;
                continue;
            }

            // Decrement right pointer if current byte is not alphanumeric
            if !bytes[right].is_ascii_alphanumeric() {
                if right == 0 {
                    break;
                }
                right -= 1;
                continue;
            }

            if bytes[left].to_ascii_lowercase() != bytes[right].to_ascii_lowercase() {
                return false;
            }

            left += 1;
            if right == 0 {
                break;
            }
            right -= 1;
        }
        true
    }
}

fn main() {
    // Test case 1
    let s1 = String::from("A man, a plan, a canal: Panama");
    println!("Test 1 Output: {}", Solution::is_palindrome(s1)); // Expected true
    // Test case 2
    let s2 = String::from("race a car");
    println!("Test 2 Output: {}", Solution::is_palindrome(s2)); // Expected false 
    // Test case 3
    let s3 = String::from(" ");
    println!("Test 3 Output: {}", Solution::is_palindrome(s3)); // Expected true
}
