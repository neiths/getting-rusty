class Solution:
    def isPalindrome(self, s: str) -> bool:
        left, right = 0, len(s) - 1

        while left < right:
            while left < right and not s[left].isalnum():
                left += 1

            while left < right and not s[right].isalnum():
                right -= 1

            if s[left].lower() != s[right].lower():
                return False

            left += 1
            right -= 1

        return True


if __name__ == "__main__":
    sol = Solution()

    # Test Case 1
    s1 = "A man, a plan, a canal: Panama"
    print(f"Test 1 Output: {sol.isPalindrome(s1)}")  # Expected: True

    # Test Case 2
    s2 = "race a car"
    print(f"Test 2 Output: {sol.isPalindrome(s2)}")  # Expected: False

    # Test Case 3
    s3 = " "
    print(f"Test 3 Output: {sol.isPalindrome(s3)}")  # Expected: True
