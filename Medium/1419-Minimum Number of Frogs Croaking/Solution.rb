# @param {String} croak_of_frogs
# @return {Integer}
def min_number_of_frogs(croak_of_frogs)
  counter = { 'c' => 0, 'r' => 0, 'o' => 0, 'a' => 0 }
  prev = { 'r' => 'c', 'o' => 'r', 'a' => 'o' }
  frogs = 0
  ret = 0

  croak_of_frogs.each_char do |c|
    if c == 'c'
      counter['c'] += 1
      frogs += 1
      ret = [ret, frogs].max
    elsif c == 'k'
      return -1 if counter['a'] == 0

      counter['a'] -= 1
      frogs -= 1
    else
      return -1 if counter[prev[c]] == 0

      counter[prev[c]] -= 1
      counter[c] += 1
    end
  end

  counter.values.all? { |v| v == 0 } ? ret : -1
end
