class Solution:
    def moveZeros(self, nums: list[int]) -> None:
        """
        Do not return anything, modify nums in-place instead
        """

        slow = 0

        for fast in range(len(nums)):
            if nums[fast] != 0:
                nums[slow], nums[fast] = nums[fast], nums[slow]
                slow += 1


if __name__ == "__main__":
    sol = Solution()

    test1 = [0, 1, 0, 3, 12]
    sol.moveZeros(test1)
    print("Test 1:", test1)

    test2 = [0]
    sol.moveZeros(test2)
    print("Test 2:", test2)
