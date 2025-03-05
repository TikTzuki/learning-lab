package org.tiktzuki.leetcode.medium;

import java.util.HashSet;
import java.util.Set;

public class CardFlippingGame {

    public int flipgame(int[] fronts, int[] backs) {
        int min_good_number = Integer.MAX_VALUE;
        Set<Integer> s = new HashSet<>();
        for (int i = 0; i < fronts.length; i++) {
            if(fronts[i] == backs[i])
                s.add(fronts[i]);
        }

        for (int i = 0; i < fronts.length; i++) {
            int currentFront = fronts[i];
            int currentBack = backs[i];
            if (!s.contains(currentFront) && currentFront < min_good_number)
                min_good_number = currentFront;
            if (!s.contains(currentBack) && currentBack < min_good_number)
                min_good_number = currentBack;
        }
        return min_good_number % Integer.MAX_VALUE;
    }

    public static void main(String[] args) {
        int[] front = {1, 2, 4, 4, 7};
        int[] back = {1, 3, 4, 1, 3};
        var solution = new CardFlippingGame();
        System.out.println(solution.flipgame(front, back));
    }
}
