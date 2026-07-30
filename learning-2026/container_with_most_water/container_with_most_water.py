from typing import List


class Solution:
    def maxArea(self, height: List[int]) -> int:
        left, right = 0, len(height) - 1
        max_water = 0

        while left < right:
            # width between the two lines
            width = right - left

            # the bottleneck height is the shorter of the two lines
            current_height = min(height[left], height[right])

            current_area = width * current_height

            max_water = max(max_water, current_area)

            # Move the pointer pointing to the shorter line
            if height[left] < height[right]:
                left += 1
            else:
                right -= 1

        return max_water


# --- Runnable Test Cases ---
if __name__ == "__main__":
    sol = Solution()

    # Test Case 1
    h1 = [1, 8, 6, 2, 5, 4, 8, 3, 7]
    print(f"Test 1 Output: {sol.maxArea(h1)}")  # Expected: 49

    # Test Case 2
    h2 = [1, 1]
    print(f"Test 2 Output: {sol.maxArea(h2)}")  # Expected: 1
