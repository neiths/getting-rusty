# LeetCode 15: 3Sum

- **Pattern:** Two Pointers (Sorting + Opposite Ends)
- **Difficulty:** Medium
- **Time Complexity:** $\mathcal{O}(n^2)$
- **Space Complexity:** $\mathcal{O}(1)$ or $\mathcal{O}(n)$ (depending on sort implementation)

---

## 1. Problem Statement

Given an integer array `nums`, return all the triplets `[nums[i], nums[j], nums[k]]` such that `i != j`, `i != k`, and `j != k`, and `nums[i] + nums[j] + nums[k] == 0`.

Notice that the solution set **must not contain duplicate triplets**.

### Examples

**Example 1:**

- **Input:** `nums = [-1,0,1,2,-1,-4]`
- **Output:** `[[-1,-1,2],[-1,0,1]]`
- **Explanation:**
  `nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0`.
  `nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0`.
  `nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0`.
  The distinct triplets are `[-1,0,1]` and `[-1,-1,2]`.

**Example 2:**

- **Input:** `nums = [0,1,1]`
- **Output:** `[]`
- **Explanation:** The only possible triplet does not sum up to 0.

**Example 3:**

- **Input:** `nums = [0,0,0]`
- **Output:** `[[0,0,0]]`

---

## 2. Intuition & Key Pattern

`3Sum` reduces down to `Two Sum II` once we sort the array:

1. **Sort the array:** Sorting allows us to use two pointers and skip duplicates easily.
2. **Iterate with a fixed outer loop:** Fix `nums[i]` as the first element. Now, the problem becomes finding two numbers in the remaining subarray (`nums[i+1...]`) that sum to `-nums[i]`.
3. **Use Two Pointers:** Place `left = i + 1` and `right = len(nums) - 1`. Move them inward just like `Two Sum II`.
4. **Skip Duplicates:** To avoid identical triplets:
   - Skip `nums[i]` if `nums[i] == nums[i-1]`.
   - After finding a valid triplet, advance `left` and `right` past identical adjacent values.

---
