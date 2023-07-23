# @param {String} s
# @return {Integer}
def max_depth(s)
  left_count = 0
  ret = 0

  s.chars.each do |ch|
    if ch == '('
      left_count += 1
      ret = left_count if left_count > ret
    elsif ch == ')'
      left_count -= 1
    end
  end

  ret
end
