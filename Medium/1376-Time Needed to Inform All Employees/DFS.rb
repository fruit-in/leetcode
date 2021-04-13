# @param {Integer} n
# @param {Integer} head_id
# @param {Integer[]} manager
# @param {Integer[]} inform_time
# @return {Integer}
def num_of_minutes(_n, head_id, manager, inform_time)
  subordinates = {}

  (0...manager.size).each do |i|
    subordinates[manager[i]] = [] unless subordinates.member?(manager[i])
    subordinates[manager[i]].push(i)
  end

  dfs(head_id, subordinates, inform_time)
end

# @param {Integer} head_id
# @param {Hash} subordinates
# @param {Integer[]} inform_time
# @return {Integer}
def dfs(head_id, subordinates, inform_time)
  return 0 unless subordinates.member?(head_id)

  inform_time[head_id] +
    subordinates[head_id].map { |id| dfs(id, subordinates, inform_time) }.max
end
