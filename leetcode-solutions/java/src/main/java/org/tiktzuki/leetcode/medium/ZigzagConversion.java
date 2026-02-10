package org.tiktzuki.leetcode.medium;

public class ZigzagConversion {
    public String convert(String s, int n) {
        int i = 0;
        StringBuilder sb = new StringBuilder();
        char[] str = s.toCharArray();
        int previousRow = i;
        while (i < str.length) {
            if (str[i] != '0')
                sb.append(str[i]);
            str[i] = '0';

            if (isNotFirstOrLast(i, n)) {
                int zagIndex = i + (n - previousRow) + (n - previousRow - 2);
                if (zagIndex < str.length) {
                    sb.append(str[zagIndex]);
                    str[zagIndex] = '0';
                }
                if(isZagIsLastCharOfRow(zagIndex, n, str.length, previousRow)){
                    i = previousRow + 1;
                    previousRow = i;
                    continue;
                }
            }

            if (!isLastCharOfRow(i, n, str.length)){
                i = nextColumn(i, n);
            }
            else{
                i = previousRow + 1;
                previousRow = i;
            }
        }
        return sb.toString();
    }

    boolean isLastCharOfRow(int i, int n, int length) {
        int lastNumberRow = length % (n + n / 2);
        int limitOfLastColumn = length - lastNumberRow;
        return i >= limitOfLastColumn;
    }
    boolean isZagIsLastCharOfRow(int zagIndex, int n, int length, int previousRow){
        if(zagIndex + (n-previousRow) + (n - previousRow -2) >= length)
            return true;
        return false;
    }

    int nextColumn(int i, int n) {
        return i + (n + (int) n / 2);
    }

    boolean isNotFirstOrLast(int i, int n) {
        int mod = i % (n + n / 2);
        return mod > 0 && mod < n - 1;
    }

    public static void main(String[] args) {
        var str = "PAYPALISHIRING";
        var solution = new ZigzagConversion();
        var result = solution.convert(str, 4);
        System.out.println(result);
    }
}
