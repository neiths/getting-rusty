struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;

        while left < right {
            let current_sum = numbers[left] + numbers[right];

            if current_sum == target {
                // LeetCode 167 requires 1-based indexing
                return vec![(left + 1) as i32, (right + 1) as i32];
            } else if current_sum > target {
                right -= 1;
            } else {
                left += 1;
            }
        }

        vec![]
    }
}

fn main() {
    // Test Case 1
    let numbers1 = vec![2, 7, 11, 15];
    let target1 = 9;
    let result1 = Solution::two_sum(numbers1, target1);
    println!("Test case 1 Output: {:?}", result1);

    // Test Case 2
    let numbers2 = vec![2, 3, 4];
    let target2 = 6;
    let result2 = Solution::two_sum(numbers2, target2);
    println!("Test case 2 Output: {:?}", result2);

    // Test Case 3
    let numbers3 = vec![-1, 0];
    let target3 = -1;
    let result3 = Solution::two_sum(numbers3, target3);
    println!("Test case 3 Output: {:?}", result3);
}
