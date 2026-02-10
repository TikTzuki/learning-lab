package org.tiktzuki.leetcode.easy;

import java.util.HashMap;
import java.util.Map;
import java.util.Objects;
import java.util.Stack;

public class ValidParentheses {
    Map<Character, Character> map = new HashMap<>() {{
        put('(', ')');
        put('[', ']');
        put('{', '}');
    }};

    public boolean isValid(String s) {
        Stack<Character> charStack = new Stack<>();
        int i = 0;
        while (i < s.length()) {
            if (!charStack.empty() && Objects.isNull(map.get(charStack.peek()))) {
                return false;
            }
            if (!charStack.empty() && map.get(charStack.peek()).equals(s.charAt(i))) {
                charStack.pop();
            } else {
                charStack.push(s.charAt(i));
            }
            i++;
        }
        return charStack.empty();
    }

    public static void main(String[] args) {
        var result = new ValidParentheses().isValid("()");
        System.out.println(result);
    }
}
