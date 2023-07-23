# @param {Integer[][]} nums
# @return {Integer[]}
def find_diagonal_order(nums)
    num_row_col = []

    for row in 0...nums.length
        for col in 0...nums[row].length
            num_row_col.push([nums[row][col], row, col])
        end
    end

    num_row_col.sort_by! {|nrc| nrc[2]}
    num_row_col.sort_by! {|nrc| nrc[1] + nrc[2]}

    return num_row_col.map {|nrc| nrc[0]}
end
