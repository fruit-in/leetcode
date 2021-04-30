# @param {String} s1
# @param {String} s2
# @return {Boolean}
def check_inclusion(s1, s2)
  return false if s1.size > s2.size

  count1 = [0] * 26
  count2 = [0] * 26

  (0...s1.size).each do |i|
    count1[s1[i].ord - 97] += 1
    count2[s2[i].ord - 97] += 1
  end

  (0..s2.size - s1.size).each do |i|
    return true if count1 == count2

    if i + s1.size < s2.size
      count2[s2[i].ord - 97] -= 1
      count2[s2[i + s1.size].ord - 97] += 1
    end
  end

  false
end
