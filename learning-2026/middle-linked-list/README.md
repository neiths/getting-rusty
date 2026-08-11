# LeetCode 876: Middle of the Linked List

- **Pattern:** Fast & Slow Pointers (Floyd's Tortoise and Hare)
- **Difficulty:** Easy
- **Time Complexity:** $\mathcal{O}(n)$
- **Space Complexity:** $\mathcal{O}(1)$

---

## 1. Problem Statement

Given the `head` of a singly linked list, return *the middle node of the linked list*.

If there are two middle nodes, return **the second middle node**.

### Examples

**Example 1:**
- **Input:** `head = [1,2,3,4,5]`
- **Output:** `[3,4,5]`
- **Explanation:** The middle node of the list is node 3.

**Example 2:**
- **Input:** `head = [1,2,3,4,5,6]`
- **Output:** `[4,5,6]`
- **Explanation:** Since the list has two middle nodes with values 3 and 4, we return the second one.

---

## 2. Intuition & Key Pattern

Instead of making two passes (one to count the total length $N$ and another to reach index $N/2$), we can solve this in a **single pass** using the **Fast & Slow Pointers** technique:

1. **Pointers Setup:** Initialize two pointers, `slow` and `fast`, both starting at `head`.
2. **Speed Difference:** 
   - `slow` moves forward **1 step** at a time (`slow = slow.next`).
   - `fast` moves forward **2 steps** at a time (`fast = fast.next.next`).
3. **Termination:** When `fast` reaches the end of the list (`None` / `null`) or `fast.next` is `None`, the `slow` pointer will be exactly at the middle node!

---