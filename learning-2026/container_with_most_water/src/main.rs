struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_water = 0;

        while left < right {
            let width = (right - left) as i32;
            let current_height = height[left].min(height[right]);
            let current_area = width * current_height;

            max_water = max_water.max(current_area);

            // Move the pointer of the shorter line inward
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_water
    }
}

fn main() {
    // Test case 1
    let h1 = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    println!("Test 1 Output: {}", Solution::max_area(h1)); // Expected 49

    // Test case 2
    let h2 = vec![1, 1];
    println!("Test 2 Output: {}", Solution::max_area(h2)); // Expected 1
}
