package org.tiktzuki.leetcode.medium;

import java.util.Map;

public class ValidSudoku {
    static final double SUPER_PRIME = 223092870;
    static final Map<Character, Integer> map = Map.of(
            '1', 2,
            '2', 3,
            '3', 5,
            '4', 7,
            '5', 11,
            '6', 13,
            '7', 17,
            '8', 19,
            '9', 23
    );

    public static boolean isValidSet(char[] set) {
        double current = SUPER_PRIME;
        for (char c : set) {
            if (c == '.')
                continue;
            int primeDivider = map.get(c);
            double modulo = current % primeDivider;
            current = current / primeDivider;
            if (modulo != 0)
                return false;
        }
        return true;
    }

    public boolean isValidSudoku(char[][] board) {
        for (int i = 0; i < 9; i++) {
            // scan rows
            boolean isValid = isValidSet(board[i]);
            if (!isValid)
                return false;

            // scan columns
            isValid = isValidSet(new char[]{
                    board[0][i],
                    board[1][i],
                    board[2][i],
                    board[3][i],
                    board[4][i],
                    board[5][i],
                    board[6][i],
                    board[7][i],
                    board[8][i]
            });
            if (!isValid)
                return false;
            // scan sub-boxes
            if (i == 0 || i == 3 || i == 6) {
                char[] set = {
                        board[i][0],
                        board[i][1],
                        board[i][2],
                        board[i + 1][0],
                        board[i + 1][1],
                        board[i + 1][2],
                        board[i + 2][0],
                        board[i + 2][1],
                        board[i + 2][2],
                };
                isValid = isValidSet(set);
                if (!isValid)
                    return false;

                char[] set2 = {
                        board[i][3],
                        board[i][4],
                        board[i][5],
                        board[i + 1][3],
                        board[i + 1][4],
                        board[i + 1][5],
                        board[i + 2][3],
                        board[i + 2][4],
                        board[i + 2][5],
                };
                isValid = isValidSet(set2);
                if (!isValid)
                    return false;
                char[] set3 = {
                        board[i][6],
                        board[i][7],
                        board[i][8],
                        board[i + 1][6],
                        board[i + 1][7],
                        board[i + 1][8],
                        board[i + 2][6],
                        board[i + 2][7],
                        board[i + 2][8],
                };
                isValid = isValidSet(set3);
                if (!isValid)
                    return false;
            }
        }
        return true;
    }

    public static void main(String[] args) {
        char[][] input = {
                {'.', '.', '4', '.', '.', '.', '6', '3', '.'},
                {'.', '.', '.', '.', '.', '.', '.', '.', '.'},
                {'5', '.', '.', '.', '.', '.', '.', '9', '.'},

                {'.', '.', '.', '5', '6', '.', '.', '.', '.'},
                {'4', '.', '3', '.', '.', '.', '.', '.', '1'},
                {'.', '.', '.', '7', '.', '.', '.', '.', '.'},

                {'.', '.', '.', '5', '.', '.', '.', '.', '.'},
                {'.', '.', '.', '.', '.', '.', '.', '.', '.'},
                {'.', '.', '.', '.', '.', '.', '.', '.', '.'}
        };
        System.out.println(1 << '.');
        var solution = new ValidSudoku();
        var result = solution.isValidSudoku(input);
        System.out.println(result);
    }
}
