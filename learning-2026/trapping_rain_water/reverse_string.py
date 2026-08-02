from typing import List


class Solution:
    def reverseString(self, s: List[str]) -> None:
        left, right = 0, len(s) - 1

        while left < right:
            s[left], s[right] = s[right], s[left]

            left += 1
            right -= 1


if __name__ == "__main__":
    sol = Solution()

    test1 = ["h", "e", "l", "l", "o"]
    sol.reverseString(test1)
    print("Test 1:", test1)  # ["o", "l", "l", "e", "h"]

    test2 = ["H", "a", "n", "n", "a", "h"]
    sol.reverseString(test2)
    print("Test 2:", test2)  # ["h", "a", "n", "n", "a", "H"]
