# @param {String} s
# @return {Integer}
def max_length_between_equal_characters(s)
  pairs = Array.new(26) { |_| [s.length, 0] }

  s.chars.each_with_index do |c, i|
    k = c.ord - 97

    pairs[k][0] = i if pairs[k][0] > i
    pairs[k][1] = i if pairs[k][1] < i
  end

  pairs.map { |pair| pair[1] - pair[0] - 1 }.max
end
