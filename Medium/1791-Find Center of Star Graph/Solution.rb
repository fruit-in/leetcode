# @param {Integer[][]} edges
# @return {Integer}
def find_center(edges)
  edges[0].include?(edges[1][0]) ? edges[1][0] : edges[1][1]
end
