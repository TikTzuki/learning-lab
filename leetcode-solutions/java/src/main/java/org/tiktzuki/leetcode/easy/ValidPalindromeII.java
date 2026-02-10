package org.tiktzuki.leetcode.easy;

public class ValidPalindromeII {
    public static boolean isPalindrome(String s, int left, int right) {
        while (left < right) {
            if (s.charAt(left) != s.charAt(right))
                return false;
            left++;
            right--;
        }
        return true;
    }

    public static boolean isPalindromePossible(String str) {
        int i = 0, j = str.length() - 1;
        while (i < j) {
            if (str.charAt(i) != str.charAt(j)) {
                return isPalindrome(str, i + 1, j) || isPalindrome(str, i, j - 1);
            }
            i++;
            j--;
        }
        return true;
    }

    public static void main(String[] args) {
        String input = "ebcbbececabbacecbbcbe";
        boolean result = isPalindromePossible(input);
        System.out.println(result);
    }
}
