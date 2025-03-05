package org.tiktzuki.leetcode.easy;

public class Pangram {
    public boolean checkIfPangram(String sentence) {
        int[] characters = new int[26];
        int count = 0;
        for (char c : sentence.toLowerCase().toCharArray()) {
            if (97 <= c && c <= 122 && characters[c - 97] == 0) {
                characters[c - 97] = 1;
                count++;
            }
        }
        return count == 26;
    }


    public static void main(String[] args) {
        String input = "Cwm fjord bank glyphs vext quiz";
        boolean result = new Pangram().checkIfPangram(input);
        assert result;
    }
}
