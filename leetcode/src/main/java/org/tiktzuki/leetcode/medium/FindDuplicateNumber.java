package org.tiktzuki.leetcode.medium;

public class FindDuplicateNumber {
    public int findDuplicate(int[] nums) {
        int[] counter = new int[nums.length + 1];
        for (int i : nums) {
            if (counter[i] > 0) {
                return i;
            }
            counter[i]++;
        }
        return -1;
    }
}
