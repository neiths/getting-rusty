struct Solution;

impl Solution {
    pub fn move_zeros(nums: &mut Vec<i32>) {
        let mut slow = 0;

        for fast in 0..nums.len() {
            if nums[fast] != 0 {
                nums.swap(slow, fast);
                slow += 1;
            }
        }
    }
}

fn main() {
    let mut test1 = vec![0, 1, 0, 3, 12];
    Solution::move_zeros(&mut test1);
    println!("Test 1: {:?}", test1); // [1, 3, 12, 0, 0]

    let mut test2 = vec![0];
    Solution::move_zeros(&mut test2);
    println!("Test 2: {:?}", test2); // [0]
}
