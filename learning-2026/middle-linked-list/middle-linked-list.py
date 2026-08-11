from typing import Optional

# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def middleNode(self, head: Optional[ListNode]) -> Optional[ListNode]:
        slow = head
        fast = head

        while fast and fast.next:
            slow = slow.next
            fast = fast.next.next

        return slow


# Helper function to create linked list from list
def create_linked_list(arr):
    if not arr:
        return None
    head = ListNode(arr[0])
    curr = head
    for val in arr[1:]:
        curr.next = ListNode(val)
        curr = curr.next
    return head


# Helper function to convert linked list to python list
def to_list(head):
    res = []
    curr = head
    while curr:
        res.append(curr.val)
        curr = curr.next
    return res


if __name__ == "__main__":
    sol = Solution()

    # Test Case 1: Odd length [1, 2, 3, 4, 5]
    head1 = create_linked_list([1, 2, 3, 4, 5])
    mid1 = sol.middleNode(head1)
    print("Test 1 Output:", to_list(mid1))  # Expected: [3, 4, 5]

    # Test Case 2: Even length [1, 2, 3, 4, 5, 6]
    head2 = create_linked_list([1, 2, 3, 4, 5, 6])
    mid2 = sol.middleNode(head2)
    print("Test 2 Output:", to_list(mid2))  # Expected: [4, 5, 6]