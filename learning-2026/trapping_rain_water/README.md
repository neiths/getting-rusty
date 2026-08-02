Write a function that reverses a string. The input string is given as an array of characters `s`.

You must do this by modifying the input array **in-place** with $\mathcal{O}(1)$ extra memory.

### Examples

**Example 1:**

- **Input:** `s = ["h","e","l","l","o"]`
- **Output:** `["o","l","l","e","h"]`

**Example 2:**

- **Input:** `s = ["H","a","n","n","a","h"]`
- **Output:** `["h","a","n","n","a","H"]`

---

## 2. Intuition & Key Pattern

Reversing an array in-place is the fundamental exercise for two pointers moving inward:

1. **Initialize Pointers:** Place `left = 0` at the start and `right = len(s) - 1` at the end.
2. **Swap Elements:** Swap the character at `s[left]` with the character at `s[right]`.
3. **Move Inward:** Increment `left` (`left += 1`) and decrement `right` (`right -= 1`).
4. **Termination:** Continue swapping while `left < right`. When `left >= right`, all elements have met in the middle, and the array is reversed.

---
