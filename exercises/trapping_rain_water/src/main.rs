struct Solution;

impl Solution {
    pub fn reserve_string(s: &mut Vec<char>) {
        if s.is_empty() {
            return;
        }

        let mut left = 0;
        let mut right = s.len() - 1;

        while left < right {
            s.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}

fn main() {
    let mut test1 = vec!['h', 'e', 'l', 'l', 'e'];
    Solution::reserve_string(&mut test1);
    println!("Test 1: {:?}", test1);

    let mut test2 = vec!['H', 'a', 'n', 'n', 'a', 'h'];
    Solution::reserve_string(&mut test2);
    println!("Test 2: {:?}", test2);
}
