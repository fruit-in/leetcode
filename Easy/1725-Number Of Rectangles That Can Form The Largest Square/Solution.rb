# @param {Integer[][]} rectangles
# @return {Integer}
def count_good_rectangles(rectangles)
  max_len = 0
  ret = 0

  rectangles.each do |rec|
    if rec.min == max_len
      ret += 1
    elsif rec.min > max_len
      max_len = rec.min
      ret = 1
    end
  end

  ret
end
