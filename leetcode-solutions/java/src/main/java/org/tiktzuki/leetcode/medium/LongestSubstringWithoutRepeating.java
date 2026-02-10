package org.tiktzuki.leetcode.medium;

import java.util.ArrayList;
import java.util.List;

public class LongestSubstringWithoutRepeating {
    public int lengthOfLongestSubstring(String s) {
        if(s.isEmpty())
            return 0;
        StringBuilder stringBuilder = new StringBuilder();
        List<Integer> result = new ArrayList<>();
        for (int i = 0; i < s.length(); i++) {
            char currentChar = s.charAt(i);
            int currentCharIndex = stringBuilder.indexOf(String.valueOf(currentChar));
            if (currentCharIndex != -1) {
                result.add(stringBuilder.length());
                stringBuilder.delete(0, currentCharIndex+1);
                stringBuilder = new StringBuilder(stringBuilder.toString());
            }
            stringBuilder.append(currentChar);
        }
        result.add(stringBuilder.length());
        return result.stream().max(Integer::compareTo).get();
    }

    public static void main(String[] args) {
        String s = "dvdf";
        System.out.println(new LongestSubstringWithoutRepeating().lengthOfLongestSubstring(s));
    }
}
