package org.tiktzuki.leetcode.medium;

public class RemovingMinimumAndMaximumFromArray {
    public static int minMoves(int[] nums) {
        int minIndex = -1;
        int min = Integer.MAX_VALUE;
        int maxIndex = -1;
        int max = Integer.MIN_VALUE;
        for (int i = 0; i < nums.length; i++) {
            if (nums[i] < min) {
                min = nums[i];
                minIndex = i;
            }
            if (nums[i] > max) {
                max = nums[i];
                maxIndex = i;
            }
        }
        int firstIndex = Math.min(minIndex, maxIndex);
        int secondIndex = Math.max(minIndex, maxIndex);
        int fromStartAndEndDistance = firstIndex + 1 + nums.length - secondIndex;
        return Math.min(Math.min(nums.length - firstIndex, secondIndex + 1), fromStartAndEndDistance);
    }

    public static void main(String[] args) {
        int[] nums = {5, 3, 2, 4, 1};
        System.out.println(minMoves(nums));
    }
}
