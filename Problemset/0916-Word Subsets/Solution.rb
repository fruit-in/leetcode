# @param {String[]} a
# @param {String[]} b
# @return {String[]}
def word_subsets(a, b)
  count = [0] * 26
  b.each do |sub|
    count_ = count_chars(sub)
    (0...26).each do |i|
      count[i] = [count[i], count_[i]].max
    end
  end

  a.filter { |word| count_chars(word).zip(count).all? { |c| c[0] >= c[1] } }
end

# @param {String} s
# @return {Integer[]}
def count_chars(s)
  count = [0] * 26
  s.each_byte { |c| count[c - 97] += 1 }

  count
end
