# @param {Integer[]} a
# @return {Integer}
def number_of_arithmetic_slices(a)
  count = 1
  ret = 0

  (2...a.length).each do |i|
    if a[i] - a[i - 1] == a[i - 1] - a[i - 2]
      count += 1
    else
      ret += (count - 1) * count / 2
      count = 1
    end
  end

  ret + (count - 1) * count / 2
end
