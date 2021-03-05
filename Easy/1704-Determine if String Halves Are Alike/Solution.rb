# @param {String} s
# @return {Boolean}
def halves_are_alike(s)
  count = 0

  (0...s.size).each do |i|
    count += i < s.size / 2 ? 1 : -1 if 'aeiouAEIOU'.include?(s[i])
  end

  count == 0
end
