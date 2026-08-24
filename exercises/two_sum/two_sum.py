from typing import List


def twoSum(numbers: List[int], target: int) -> List[int]:
    left, right = 0, len(numbers) - 1

    while left < right:
        current_sum = numbers[left] + numbers[right]

        if current_sum == target:
            return [left + 1, right + 1]

        elif current_sum > target:
            right -= 1
        else:
            left += 1


if __name__ == "__main__":
    numbers1, target1 = [2, 7, 11, 15], 9
    numbers2, target2 = [2, 3, 4], 6
    numbers3, target3 = [-1, 0], -1

    print(f"Test Case 1 output: {twoSum(numbers1, target1)}")
    print(f"Test Case 2 output: {twoSum(numbers2, target2)}")
    print(f"Test Case 1 output: {twoSum(numbers3, target3)}")
