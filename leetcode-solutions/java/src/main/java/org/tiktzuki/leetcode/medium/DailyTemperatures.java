package org.tiktzuki.leetcode.medium;

import java.util.Arrays;

public class DailyTemperatures {
    public int[] dailyTemperatures(int[] temperatures) {
        int n = temperatures.length;
        int[] rs = new int[n];
        for (int i = 0; i < n; i++) {
            for (int j = i + 1; j < n; j++) {
                if (temperatures[j] > temperatures[i]) {
                    rs[i] = j - i;
                    break;
                }
            }
        }
        return rs;
    }

    public static void main(String[] args) {
        int[] rs= new DailyTemperatures().dailyTemperatures(new int[]{73,74,75,71,69,72,76,73});
        Arrays.stream(rs).forEach(System.out::println);
    }
}