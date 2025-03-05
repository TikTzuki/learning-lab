package org.tiktzuki.leetcode.medium;

import java.util.ArrayList;
import java.util.List;

public class LexicographicalOrder {
    public List<Integer> lexicalOrder(int n) {
        List<Integer> result = new ArrayList<>();
        dfs(1, n, result);
        return result;
    }

    public void dfs(int temp, int n, List<Integer> result) {
        if (temp > n)
            return;
        result.add(temp);
        dfs(temp * 10, n, result);
        if (temp % 10 != 9)
            dfs(temp + 1, n, result);
    }

    public static void main(String[] args) {
        var solution = new LexicographicalOrder();
        var result = solution.lexicalOrder(13);
        System.out.println(result);
    }
}
