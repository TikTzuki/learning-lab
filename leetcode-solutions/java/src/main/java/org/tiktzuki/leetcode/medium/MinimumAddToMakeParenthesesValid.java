package org.tiktzuki.leetcode.medium;

import java.util.Stack;

public class MinimumAddToMakeParenthesesValid {
    public boolean opposite(char a, char b) {
        return a == '(' && b == ')';
    }

    public int minAddToMakeValid(String s) {
        Stack<Character> stack = new Stack<>();
        for (char c : s.toCharArray()) {
            if (stack.isEmpty() || !opposite(stack.peek(), c)) {
                stack.push(c);
            } else {
                stack.pop();
            }
        }
        return stack.size();
    }

    public static void main(String[] args) {
        MinimumAddToMakeParenthesesValid solution = new MinimumAddToMakeParenthesesValid();
        String input = "()))((";
        int result = solution.minAddToMakeValid(input);
        System.out.println(result);
    }
}
