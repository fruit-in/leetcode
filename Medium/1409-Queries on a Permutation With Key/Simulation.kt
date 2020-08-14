class Solution {
    fun processQueries(queries: IntArray, m: Int): IntArray {
        var p = ArrayList<Int>((1..m).toList().reversed())
        var ret = IntArray(queries.size)

        for (i in 0 until queries.size) {
            ret[i] = m - 1 - p.indexOf(queries[i])
            p.remove(queries[i])
            p.add(queries[i])
        }

        return ret
    }
}
