package org.tiktzuki.leetcode.medium;

import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.atomic.AtomicInteger;

class TopKFrequent {
    public int[] topKFrequent(int[] nums, int k) {
        Map<Integer, Integer> map = new HashMap<>();
        for (int n : nums) {
            map.put(n, map.getOrDefault(n, 0) + 1);
        }
        int[] rs = new int[k];
        AtomicInteger i = new AtomicInteger(0);
        map.entrySet()
            .stream()
            .sorted((entry1, entry2) -> entry1.getValue().compareTo(entry2.getValue())).limit(k)
            .forEach(entry -> {
                rs[i.get()] = entry.getKey();
                i.incrementAndGet();
            });
        return rs;
    }

    public static void main(String[] args) {
        new TopKFrequent().topKFrequent(new int[]{1, 1, 1, 2, 2, 3}, 2);
    }
}