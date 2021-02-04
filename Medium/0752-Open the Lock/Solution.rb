# @param {String[]} deadends
# @param {String} target
# @return {Integer}
def open_lock(deadends, target)
  deadends = deadends.map { |deadend| deadend.to_i }.to_set
  target = target.to_i
  deadends.add(target)
  states = Containers::Queue.new([[target, 0]])

  until states.empty?
    state, i = states.pop

    return i if state == 0

    (0..3).each do |j|
      [-1, 1].each do |k|
        new_state = [state / 1000, state % 1000 / 100, state % 100 / 10, state % 10]
        new_state[j] = (new_state[j] + k) % 10
        new_state = new_state[0] * 1000 + new_state[1] * 100 + new_state[2] * 10 + new_state[3]

        unless deadends.member?(new_state)
          deadends.add(new_state)
          states.push([new_state, i + 1])
        end
      end
    end
  end

  -1
end
