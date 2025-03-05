package org.tiktzuki.leetcode.easy;

public class FindNonDuplicateNumberInstances {
    public int moveElements(int[] arr) {
        int biggestNumberIndex = 0;
        for (int i = 0; i < arr.length; i++) {
            if (arr[i] > arr[biggestNumberIndex]) {
                biggestNumberIndex++;
                arr[biggestNumberIndex] = arr[i];
            }
        }
        return biggestNumberIndex + 1;
    }
}
