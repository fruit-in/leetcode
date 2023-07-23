# @param {Integer[]} queries
# @param {Integer} m
# @return {Integer[]}
def process_queries(queries, m)
    p = Array(m.downto(1))
    ret = Array.new(queries.length)

    for i in 0...queries.length
        ret[i] = m - 1 - p.index(queries[i])
        p.delete(queries[i])
        p.push(queries[i])
    end

    return ret
end
