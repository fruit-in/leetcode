# @param {Integer[]} a
# @param {Integer[]} b
# @return {Integer[]}
def advantage_count(a, b)
  i = 0
  j = b.length - 1
  indices = (0...b.length).to_a
  ret = Array.new(b.length)

  a.sort!
  indices.sort_by! { |k| -b[k] }

  indices.each do |k|
    if b[k] < a[j]
      ret[k] = a[j]
      j -= 1
    else
      ret[k] = a[i]
      i += 1
    end
  end

  ret
end
