package org.tiktzuki.leetcode.medium;

import java.util.*;

public class GroupAnagrams {
    Set<Map<Integer, Integer>> sets = new HashSet<>();

    public boolean isAnagram(String s, String t) {
        Map<Integer, Integer> m = new HashMap<>();
        s.chars().forEach(c -> m.merge(c, 1, Integer::sum));
        sets.add(m);

        Map<Integer, Integer> m2 = new HashMap<>();
        t.chars().forEach(c -> m2.merge(c, 1, Integer::sum));
        return sets.contains(m2);
    }

    public List<List<String>> groupAnagrams(String[] strs) {
        List<List<String>> anagrams = new ArrayList<>();
        for (int i = 0; i < strs.length; i++) {
            String first = strs[i];
            if (first == null)
                continue;
            List<String> anagram = new ArrayList<>();
            anagram.add(first);
            for (int j = i + 1; j < strs.length; j++) {
                String sec = strs[j];
                if (sec == null)
                    continue;
                if (isAnagram(first, sec)) {
                    strs[j] = null;
                    anagram.add(sec);
                }
            }
            anagrams.add(anagram);
        }
        return anagrams;
    }

    public static void main(String[] args) {
        var algorithm = new GroupAnagrams();
        var input = new String[]{"eat", "tea", "tan", "ate", "nat", "bat"};
        var output = algorithm.groupAnagrams(input);
        output.forEach(System.out::println);
    }
}
