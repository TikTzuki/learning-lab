package org.tiktzuki.leetcode.easy;

public class ReverseVowels {
    final String vowel = "AEIOUaeiou";

    protected boolean isVowel(char c) {
        return vowel.indexOf(c) != -1;
    }

    public String reverseVowels(String s) {
        int i = 0, j = s.length() - 1;
        char[] array = s.toCharArray();
        while(i < j) {
            while (i < j && !isVowel(s.charAt(i))) {
                i++;
            }
            while (i < j && !isVowel(s.charAt(j))) {
                j--;
            }
            char temp = array[i];
            array[i] = array[j];
            array[j] = temp;
            i++;
            j--;
        }
        return new String(array);
    }

    public static void main(String[] args) {
        String input = "IceCreAm";
        String result = new ReverseVowels().reverseVowels(input);
        System.out.println(result);
        // raca e car
    }
}
