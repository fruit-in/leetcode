# @param {Integer[]} release_times
# @param {String} keys_pressed
# @return {Character}
def slowest_key(release_times, keys_pressed)
  max_duration = release_times[0]
  ret = keys_pressed[0]

  (1...keys_pressed.length).each do |i|
    duration = release_times[i] - release_times[i - 1]

    if duration > max_duration || (duration == max_duration && keys_pressed[i] > ret)
      max_duration = duration
      ret = keys_pressed[i]
    end
  end

  ret
end
