package org.tiktzuki.leetcode.medium;

import java.util.HashMap;
import java.util.Hashtable;
import java.util.Map;

public class LongestConsecutiveSequence {
    static class Node {
        int counter ;
        Node head = this;
        Node tail = this;
    }

    public int longestConsecutive(int[] nums) {
        Map<Integer, Node> map = new HashMap<>();
        int max = 0;
        for (int num : nums) {
            if (map.containsKey(num))
                continue;
            Node node = new Node();
            Node nextNode = map.get(num + 1);
            Node prevNode = map.get(num - 1);
            int sum = 1;
            if (nextNode != null) {
                Node headNode = nextNode.head;
                sum += headNode.counter;
                node.head = headNode;
                headNode.tail = prevNode != null ? prevNode.tail : node;
            }
            if (prevNode != null) {
                Node tailNode = prevNode.tail;
                sum += tailNode.counter;
                node.tail = tailNode;
                tailNode.head = node.head;
            }
            node.head.counter = sum;
            node.tail.counter = sum;
            map.put(num, node);
            if (sum > max)
                max = sum;
        }
        return max;
    }

    public static void main(String[] args) {
        var solution = new LongestConsecutiveSequence();
        var result = solution.longestConsecutive(new int[]{100, 4, 200, 1, 3, 2});
        System.out.println(result);
    }
}
