# @param {Integer[][]} is_connected
# @return {Integer}
def find_circle_num(is_connected)
  seen = [false] * is_connected.size
  stack = []
  ret = 0

  (0...is_connected.size).each do |i|
    next if seen[i]

    ret += 1
    stack.push(i)

    until stack.empty?
      c = stack.pop
      seen[c] = true
      (i + 1...is_connected.size).each do |j|
        stack.push(j) if !seen[j] && is_connected[c][j] == 1
      end
    end
  end

  ret
end
