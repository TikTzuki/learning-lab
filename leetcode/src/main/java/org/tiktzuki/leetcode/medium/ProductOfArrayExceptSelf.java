package org.tiktzuki.leetcode.medium;

import java.util.Arrays;

public class ProductOfArrayExceptSelf {
    public int[] productExceptSelf(int[] nums) {
        int[] prods = new int[nums.length];
        for (int i = 0; i < nums.length; i++) {
            prods[i] = 1;
        }
        int temp = 1;
        for (int i = 0; i < nums.length; i++) {
            prods[i] = temp;
            temp *= nums[i];
        }
        temp = 1;
        for (int j = nums.length - 1; j >= 0; j--) {
           prods[j] *= temp;
           temp *= nums[j];
        }
        return prods;
    }

    public static void main(String[] args) {
        var solution = new ProductOfArrayExceptSelf();
        var nums = new int[]{1, 2, 3, 4};
        var prods = solution.productExceptSelf(nums);
    }
}
