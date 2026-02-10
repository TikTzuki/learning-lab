package org.tiktzuki.leetcode.medium;

import java.util.Arrays;
import java.util.Comparator;

public class MaximumLengthOfPairChain {
    public int findLongestChain(int[][] pairs) {
        Arrays.sort(pairs, Comparator.comparingInt(a -> a[1]));
        int chainCount = 0;
        int lastNumber = Integer.MIN_VALUE;
        for (int[] currentPair : pairs) {
            if (currentPair[0] > lastNumber) {
                chainCount++;
                lastNumber = currentPair[1];
            }
        }
        return chainCount;
    }

    public static void main(String[] args) {
        MaximumLengthOfPairChain solution = new MaximumLengthOfPairChain();
        int[][] input = new int[][]{{7,8}, {5,6}, {1,2}, {3,5}, {4,5}, {2,3}};
        int result = solution.findLongestChain(input);
        System.out.println(result);
    }
}
