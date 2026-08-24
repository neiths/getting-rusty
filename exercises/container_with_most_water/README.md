# LeetCode 11 - Container With Most Water

## Problem

You are given an integer array `height` of length `n`.

There are `n` vertical lines drawn such that the two endpoints of the `i`-th line are:

- `(i, 0)`
- `(i, height[i])`

Find two lines that, together with the x-axis, form a container that can store the maximum amount of water.

Return the maximum amount of water the container can hold.

### Example

**Input**

```text
height = [1,8,6,2,5,4,8,3,7]
```

**Output**

```text
49
```

**Explanation**

Choose:

- Left index = `1` (height = `8`)
- Right index = `8` (height = `7`)

```
width = 8 - 1 = 7
min_height = min(8, 7) = 7
area = 7 × 7 = 49
```

---

# 20-Minute Strategy

## Formula

```text
Area = width × min(height[left], height[right])
```

where

```text
width = right - left
```

---

## Goal

Maximize the container area.

---

## Key Observation

The area depends on two things:

- Width
- Shorter of the two heights

As the pointers move inward:

- Width always decreases.
- Therefore, the only way to obtain a larger area is to increase the limiting (shorter) height.

---

## Two-Pointer Algorithm

1. Start with the widest possible container.

```text
left = 0
right = n - 1
```

2. Compute the current area.

3. Update the maximum area.

4. Move **only the pointer with the smaller height**.

5. Repeat until `left == right`.

---

## Why Move the Shorter Pointer?

Suppose

```
height[left] < height[right]
```

Current area is

```
(right - left) × height[left]
```

If we move the **right** pointer:

- Width decreases.
- The limiting height is still `height[left]`.

So the new area can **never** be larger.

Only moving the **shorter** pointer gives a chance to find a taller line that increases the minimum height.

Therefore:

```text
Move the shorter pointer.
Never move the taller pointer.
```

---

# Example Walkthrough

```
height = [1,8,6,2,5,4,8,3,7]
```

| Left | Right | Heights | Width | Area | Move |
|------|-------|---------|------:|-----:|------|
|0|8|(1,7)|8|8|Left|
|1|8|(8,7)|7|49 ✅|Right|
|1|7|(8,3)|6|18|Right|
|1|6|(8,8)|5|40|Either|
|...|...|...|...|...|...|

Maximum area = **49**

---

# Complexity

- **Time:** `O(n)`
- **Space:** `O(1)`

Each pointer moves at most `n` times.

---

# Python Solution

```python
class Solution:
    def maxArea(self, height):
        left, right = 0, len(height) - 1
        ans = 0

        while left < right:
            width = right - left
            area = width * min(height[left], height[right])
            ans = max(ans, area)

            if height[left] < height[right]:
                left += 1
            else:
                right -= 1

        return ans
```

---

# Key Takeaways

- Start with the **maximum width**.
- The container height is limited by the **shorter line**.
- Width always decreases as pointers move.
- Moving the **taller** pointer cannot improve the result.
- Always move the **shorter** pointer.
- Overall complexity is **O(n)**.