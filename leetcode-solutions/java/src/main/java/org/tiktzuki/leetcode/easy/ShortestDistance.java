package org.tiktzuki.leetcode.easy;

public class ShortestDistance {

    public int shortestDistance(String[] words, String word1, String word2) {
        int position1 = -1;
        int position2 = -1;
        int distance = Integer.MAX_VALUE;
        for (int i = 0; i < words.length; i++) {
            String currentWord = words[i];
            if (currentWord.equals(word1)) {
                position1 = i;
            } else if (currentWord.equals(word2)) {
                position2 = i;
            }
            if(position1!= -1 && position2 != -1){
                distance = Math.min(distance, Math.abs(position1 - position2));
            }
        }
        return distance;
    }

    public static void main(String[] args) {
        var solution = new ShortestDistance();
        String[] words = new String[]{"the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"};
        int rs = solution.shortestDistance(words, "fox", "dog");
        System.out.println(rs);
    }
}
