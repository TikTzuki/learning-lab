package org.tiktzuki.leetcode.hard;

import lombok.extern.slf4j.Slf4j;

import java.util.HashMap;
import java.util.Map;

@Slf4j
public class BestPokerHand {

    public String bestHand(int[] ranks, char[] suits) {
        int isFlush = suits[0];
        boolean isThree = false;
        boolean isPair = false;
        int[] rankCounter = new int[14];
        for (int i = 0; i < ranks.length; i++) {
            if (isFlush != suits[i])
                isFlush = 0;

            int rankCount = rankCounter[ranks[i]] + 1;
            if (rankCount == 2)
                isPair = true;
            if (rankCount == 3) {
                isThree = true;
                break;
            }
            rankCounter[ranks[i]] = rankCount;

        }

        if (isFlush != 0)
            return "Flush";
        if (isThree)
            return "Three of a Kind";
        if (isPair)
            return "Pair";
        return "High Card";
    }

    public static void main(String[] args) {
        int[] ranks = {13, 2, 3, 1, 9};
        char[] suits = {'a', 'a', 'a', 'a', 'a'};
        var solution = new BestPokerHand();
        log.info(solution.bestHand(ranks, suits));
    }
}
