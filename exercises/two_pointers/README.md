# LeetCode 283: Move Zeroes

- **Pattern:** Two Pointers (Fast & Slow / Writer-Reader)
- **Difficulty:** Easy
- **Time Complexity:** $\mathcal{O}(n)$
- **Space Complexity:** $\mathcal{O}(1)$

---

## 1. Problem Statement

Given an integer array `nums`, move all `0`'s to the end of it while maintaining the relative order of the non-zero elements.

Note that you must do this **in-place** without making a copy of the array.

### Examples

**Example 1:**

- **Input:** `nums = [0,1,0,3,12]`
- **Output:** `[1,3,12,0,0]`

**Example 2:**

- **Input:** `nums = [0]`
- **Output:** `[0]`

---

## 2. Intuition & Key Pattern

Unlike opposite-end pointers (used for reversing or palindromes), this problem uses the **Fast & Slow Pointers** (also called Writer & Reader) pattern:

1. **Slow Pointer (`slow`):** Keeps track of the index where the next non-zero element should be written.
2. **Fast Pointer (`fast`):** Scans through the array to find non-zero elements.
3. **Execution:**
   - As `fast` iterates through the array, whenever it finds a non-zero element (`nums[fast] != 0`), it swaps that element with `nums[slow]`, and then `slow` increments by 1.
   - If `nums[fast]` is `0`, `fast` simply moves forward while `slow` stays put, waiting for the next non-zero element.

This approach guarantees that all non-zero elements are compressed at the front in their original relative order, while zeroes are swapped to the end—all in a single pass.
