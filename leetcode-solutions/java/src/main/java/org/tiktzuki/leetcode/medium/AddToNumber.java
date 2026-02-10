package org.tiktzuki.leetcode.medium;

import java.util.List;


public class AddToNumber {
    public static ListNode initLinkedList(List<Integer> l) {
        ListNode l1 = null;
        ListNode tail = null;
        for (int j : l) {
            ListNode newNode = new ListNode(j);
            if (tail != null) {
                tail.next = newNode;
                tail = newNode;
            } else {
                tail = newNode;
                l1 = tail;
            }
        }
        return l1;
    }

    public static void main(String[] args) {
        ListNode l1 = initLinkedList(List.of(2, 4, 3));
        ListNode l2 = initLinkedList(List.of(5, 6, 4));
        var result = new Solution().addTwoNumbers(l1, l2);
        while (result != null) {
            System.out.println(result.val);
            result = result.next;
        }
    }

    static class ListNode {
        int val;
        ListNode next;

        ListNode() {
        }

        ListNode(int val) {
            this.val = val;
        }

        ListNode(int val, ListNode next) {
            this.val = val;
            this.next = next;
        }
    }

    static class Solution {
        public ListNode addTwoNumbers(ListNode l1, ListNode l2) {
            int additionNumber = 0;
            ListNode result = null;
            ListNode cursor = null;
            while (true) {
                if (l1 == null && l2 == null && additionNumber == 0)
                    return result;
                int sum = (l1 != null ? l1.val : 0) + (l2 != null ? l2.val : 0) + additionNumber;
                int currentVal = sum % 10;
                additionNumber = sum >= 10 ? 1 : 0;
                if (cursor != null) {
                    cursor.next = new ListNode(currentVal);
                    cursor = cursor.next;
                } else {
                    cursor = new ListNode(currentVal);
                    result = cursor;
                }
                l1 = l1 != null && l1.next != null ? l1.next : null;
                l2 = l2 != null && l2.next != null ? l2.next : null;
            }
        }
    }
}

