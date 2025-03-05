package org.tiktzuki.leetcode.medium;

import java.util.HashMap;
import java.util.HashSet;
import java.util.Set;
import java.util.Stack;

public class RemoveDuplicateLetters {
    public class ResultBuilder {
        Set<Character> usedChars = new HashSet<>();
        Stack<Character> result = new Stack<>();

        public char last() {
            return result.peek();
        }

        public boolean contains(Character c) {
            return usedChars.contains(c);
        }

        public void remove(Character c) {
            result.remove(c);
            usedChars.remove(c);
        }

        public void append(Character c) {
            result.push(c);
            usedChars.add(c);
        }

        public String toString() {
            StringBuilder sb = new StringBuilder();
            for (char c : result) {
                sb.append(c);
            }
            return sb.toString();
        }
    }

    public String removeDuplicateLetters(String s) {
        HashMap<Character, Integer> charCount = new HashMap<>();
        ResultBuilder resultBuilder = new ResultBuilder();
        for (char c : s.toCharArray()) {
            charCount.compute(c, (k, v) -> {
                if (v == null) return 1;
                return v + 1;
            });
        }
        for (char c : s.toCharArray()) {
            if (!resultBuilder.contains(c)) {
                while (!resultBuilder.result.isEmpty() && resultBuilder.last() > c && charCount.get(resultBuilder.last()) > 0) {
                    resultBuilder.remove(resultBuilder.last());
                }
                resultBuilder.append(c);
            }
            charCount.put(c, charCount.get(c) - 1);
        }
        return resultBuilder.toString();
    }

    public static void main(String[] args) {
        var sol = new RemoveDuplicateLetters();
        System.out.println(sol.removeDuplicateLetters("bcabc"));    // Output: "abc"
        System.out.println(sol.removeDuplicateLetters("cbacdcbc")); // Output: "zabcde"
        System.out.println(sol.removeDuplicateLetters("mnopmn"));
    }
}
