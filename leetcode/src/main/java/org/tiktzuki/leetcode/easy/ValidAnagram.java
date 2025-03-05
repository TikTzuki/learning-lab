package org.tiktzuki.leetcode.easy;

public class ValidAnagram {
    int[] count = new int[26];

    public boolean isAnagram(String s, String t) {
        for (char c : s.toCharArray()) {
            count[c - 97]++;
        }
        for (char c : t.toCharArray()) {
            count[c - 97]--;
        }

        for (int j : count) {
            if (j != 0)
                return false;
        }
        return true;
    }

    public static void main(String[] args) {
        String s = "racecar";
        String t = "racecars";
        var solution = new ValidAnagram();
        boolean rs = solution.isAnagram(s, t);
        System.out.println(rs);
    }
}
