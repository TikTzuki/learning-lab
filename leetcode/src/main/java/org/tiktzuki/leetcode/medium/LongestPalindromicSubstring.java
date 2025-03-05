package org.tiktzuki.leetcode.medium;

import java.util.ArrayList;
import java.util.List;

public class LongestPalindromicSubstring {
    private StringBuilder createPalindrome(StringBuilder currentPalindromicString, int i, int j, String s) {
        while (i >= 0 && j < s.length()) {
            if (s.charAt(i) != s.charAt(j)) {
                break;
            }
            currentPalindromicString
                    .insert(0, s.charAt(i))
                    .append(s.charAt(j));
            i--;
            j++;
        }

        return currentPalindromicString;
    }

    public String longestPalindrome(String s) {
        if (s.length() == 1) {
            return s;
        }
        StringBuilder longestPalindrome = new StringBuilder();
        for (int k = 0; k < s.length(); k++) {
            // Odd palindromic string
            int i = k - 1;
            int j = k + 1;
            StringBuilder palindrome = createPalindrome(new StringBuilder()
                    .append(s.charAt(k)), i, j, s);
            if (palindrome.length() > longestPalindrome.length())
                longestPalindrome = palindrome;

            // Event palindromic string
            if (k < s.length() - 1 && s.charAt(k) == s.charAt(k + 1)) {
                i = k - 1;
                j = k + 2;
                palindrome = (createPalindrome(new StringBuilder()
                        .append(s.charAt(k)).append(s.charAt(k)), i, j, s));
                if (palindrome.length() > longestPalindrome.length())
                    longestPalindrome = palindrome;
            }
        }
        return longestPalindrome.toString();
    }


    public static void main(String[] args) {
        String s = "bbb";
        System.out.printf(new LongestPalindromicSubstring().longestPalindrome(s));
    }
}
