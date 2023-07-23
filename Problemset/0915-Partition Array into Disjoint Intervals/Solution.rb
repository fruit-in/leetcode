# @param {Integer[]} a
# @return {Integer}
def partition_disjoint(a)
  max_left = a[0]
  max = a[0]
  length = 1

  (1...a.length).each do |i|
    if a[i] < max_left
      length = i + 1
      max_left = max
    elsif a[i] > max
      max = a[i]
    end
  end

  length
end
