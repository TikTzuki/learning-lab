package org.tiktzuki.leetcode.easy;

public class Palindrome {
    public boolean isAlphabetic(char c) {
        return (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9');
    }
    public boolean isPalindrome(String s) {
        char[] chars = s.toLowerCase().toCharArray();
        int i = 0, j = chars.length - 1;
        while (i < j) {
            if (!isAlphabetic(chars[i])) {
                i++;
                continue;
            }
            if (!isAlphabetic(chars[j])) {
                j--;
                continue;
            }
            if (chars[i] != chars[j]) {
                return false;
            }
            i++;
            j--;
        }
        return true;
    }

    public static void main(String[] args) {
        Palindrome palindrome = new Palindrome();
        String input = "0P";
        boolean result = palindrome.isPalindrome(input);
        System.out.println(result);
    }
}
