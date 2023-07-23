# @param {String[]} words
# @param {Integer} k
# @return {String[]}
def top_k_frequent(words, k)
    count = Hash.new(0)

    for word in words
        count[word] += 1
    end

    return count.keys.sort_by {|k| [-count[k], k]}[0...k]
end
