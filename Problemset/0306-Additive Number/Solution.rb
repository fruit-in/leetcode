# @param {String} num
# @return {Boolean}
def is_additive_number(num)
  (1...(num.length + 1) / 2).each do |i|
    break if num[0] == '0' && i > 1

    (1..[(num.length - i) / 2, num.length - 2 * i].min).each do |j|
      break if num[i] == '0' && j > 1

      k = i + j
      x = num[0...i].to_i
      y = num[i...k].to_i
      l = k + (x + y).to_s.length

      while l <= num.length
        z = num[k...l].to_i

        break if x + y != z

        x = y
        y = z
        k = l
        l = k + (x + y).to_s.length

        return true if k == num.length
      end
    end
  end

  false
end
