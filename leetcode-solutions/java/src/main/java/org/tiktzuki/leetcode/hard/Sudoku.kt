//package org.dyson.blog
//
//import java.io.InputStream
//
//val NUM_SET: CharArray = charArrayOf('1', '2', '3', '4', '5', '6', '7', '8', '9');
//val INDEXES = intArrayOf(0, 1, 2, 3, 4, 5, 6, 7, 8);
//val ZERO_CHAR = Char(0)
//
//class SodokuSolver {
//    var matrix: Array<CharArray>
//    private var targetIndexes: MutableList<Pair<Int, Int>> = mutableListOf()
//
//    constructor(filename: String) {
//        matrix = readCsv(readFileToStream(filename))
//        initTargetIndexes()
//    }
//
//    constructor(matrix: Array<CharArray>) {
//        for (y in matrix.indices) {
//            for (x in matrix[y].indices) {
//                if (matrix[y][x] == '.')
//                    targetIndexes.add(y to x)
//            }
//        }
//        this.matrix = matrix;
//    }
//
//    private fun initTargetIndexes() {
//        for (y in matrix.indices) {
//            for (x in matrix[y].indices) {
//                if (matrix[y][x] == ZERO_CHAR)
//                    targetIndexes.add(y to x)
//            }
//        }
//    }
//
//    fun getBoxIndex(index: Int): IntArray {
//        return when (index) {
//            0, 1, 2 -> intArrayOf(0, 1, 2)
//            3, 4, 5 -> intArrayOf(3, 4, 5)
//            6, 7, 8 -> intArrayOf(6, 7, 8)
//            else -> intArrayOf()
//        }
//    }
//
//    fun checkInBox(index: Pair<Int, Int>): Collection<Char> {
//        val yIndex = getBoxIndex(index.first)
//        val xIndex = getBoxIndex(index.second)
//        val numbers = NUM_SET.toMutableList();
//        for (y in yIndex) {
//            for (x in xIndex) {
//                val removeNumber = matrix.elementAt(y).elementAt(x)
//                if (removeNumber != ZERO_CHAR)
//                    numbers.remove(removeNumber)
//            }
//        }
//        return numbers;
//    }
//
//    fun checkInRow(index: Pair<Int, Int>): Collection<Char> {
//        val y = index.first
//        val numbers = NUM_SET.toMutableList();
//        for (x in INDEXES) {
//            val removeNumber = matrix.elementAt(y).elementAt(x)
//            if (removeNumber != ZERO_CHAR)
//                numbers.remove(removeNumber)
//        }
//        return numbers;
//    }
//
//    fun checkInColumn(index: Pair<Int, Int>): Collection<Char> {
//        val x = index.second
//        val numbers = NUM_SET.toMutableList();
//        for (y in INDEXES) {
//            val removeNumber = matrix.elementAt(y).elementAt(x)
//            if (removeNumber != ZERO_CHAR)
//                numbers.remove(removeNumber)
//        }
//        return numbers;
//    }
//
//    fun checkPossibleValue(index: Pair<Int, Int>): Char? {
//        val row = checkInRow(index)
//        val column = checkInColumn(index)
//        val box = checkInBox(index)
//        val rs = row intersect column intersect box
//        return if (rs.size == 1) rs.elementAt(0) else null;
//    }
//
//    fun solve(): Array<CharArray> {
//        while (targetIndexes.isNotEmpty()) {
//            var index = 0;
//            while (index < targetIndexes.size) {
//                val (y, x) = targetIndexes[index]
//                val value = checkPossibleValue(y to x)
//                if (value != null) {
//                    matrix[y][x] = value
//                    targetIndexes.removeAt(index)
//                } else {
//                    index++;
//                }
//            }
//        }
//        return matrix;
//    }
//
//    fun readCsv(inputStream: InputStream): Array<CharArray> {
//        val reader = inputStream.bufferedReader()
//        val result = Array(9) {
//            CharArray(9)
//        }
//        reader.lineSequence()
//            .forEachIndexed { y, rawString ->
//                rawString.split(',', ignoreCase = false, limit = 9)
//                    .forEachIndexed { x, chars ->
//                        if (chars.isNotEmpty())
//                            result[y][x] = chars.toCharArray()[0]
//                    }
//            };
//        return result;
//    }
//
//    fun readFileToStream(fileName: String): InputStream = this::class.java.getResourceAsStream(fileName)
//}
//
//
//class Solution {
//    fun solveSudoku(board: Array<CharArray>): Unit {
//        val result = SodokuSolver(board).solve();
//    }
//}
//
//fun main() {
//    val solver = SodokuSolver(
//        "/input.csv"
//    )
//    val result = solver.solve()
//    println(result)
//}
