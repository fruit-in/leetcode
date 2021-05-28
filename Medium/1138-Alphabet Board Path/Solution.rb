# @param {String} target
# @return {String}
def alphabet_board_path(target)
  s = [0, 0]
  ret = ''

  target.each_byte do |c|
    t = [(c - 97) / 5, (c - 97) % 5]
    ret += path_to(s, t) + '!'
    s = t
  end

  ret
end

# @param {Integer[]} start
# @param {Integer[]} target
# @return {String}
def path_to(start, target)
  if start == target
    ''
  elsif start == [5, 0]
    'U' + path_to([4, 0], target)
  elsif target == [5, 0]
    path_to(start, [4, 0]) + 'D'
  else
    v_dir = start[0] > target[0] ? 'U' : 'D'
    h_dir = start[1] > target[1] ? 'L' : 'R'
    v_dir * (start[0] - target[0]).abs + h_dir * (start[1] - target[1]).abs
  end
end
