# @param {Integer[]} arr
# @param {Integer[][]} pieces
# @return {Boolean}
def can_form_array(arr, pieces)
  indices = arr.map.with_index.to_h
  indices.default = 0

  arr == pieces.sort_by { |piece| indices[piece[0]] }.flatten(1)
end
