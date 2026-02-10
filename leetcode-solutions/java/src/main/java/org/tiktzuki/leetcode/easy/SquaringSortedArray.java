package org.tiktzuki.leetcode.easy;

public class SquaringSortedArray {
    public static int[] makeSquares(int[] arr) {
        int n = arr.length;
        int[] squares = new int[n];

        int biggestNumberIndex = arr.length - 1;
        for (int i = 0; i < n; i++) {
            int right = arr[biggestNumberIndex] * arr[biggestNumberIndex];
            int left = arr[i] * arr[i];
            if (right >= left) {
                squares[biggestNumberIndex] = right;
            }
            biggestNumberIndex--;
        }
        return squares;
    }

    public static void main(String[] args) {
        int[] result = makeSquares(new int[] { -2, -1, 0, 2, 3 });
        for (int i : result) {
            System.out.print(i + " ");
        }
    }
}
