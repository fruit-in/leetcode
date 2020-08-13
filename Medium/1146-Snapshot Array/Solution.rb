class SnapshotArray

=begin
    :type length: Integer
=end
    def initialize(length)
        @arrays = Array.new(length) {Array[Array[0, 0]]}
        @snap_id = 0
    end


=begin
    :type index: Integer
    :type val: Integer
    :rtype: Void
=end
    def set(index, val)
        if @snap_id > @arrays[index][-1][0]
            @arrays[index].push(Array[@snap_id, val])
        else
            @arrays[index][-1][1] = val
        end
    end


=begin
    :rtype: Integer
=end
    def snap()
        @snap_id += 1

        return @snap_id - 1
    end


=begin
    :type index: Integer
    :type snap_id: Integer
    :rtype: Integer
=end
    def get(index, snap_id)
        l, r = 0, @arrays[index].length - 1

        while l <= r
            m = (l + r) / 2

            if @arrays[index][m][0] == snap_id
                return @arrays[index][m][1]
            elsif @arrays[index][m][0] < snap_id
                l = m + 1
            else
                r = m - 1
            end
        end

        return @arrays[index][r][1]
    end


end

# Your SnapshotArray object will be instantiated and called as such:
# obj = SnapshotArray.new(length)
# obj.set(index, val)
# param_2 = obj.snap()
# param_3 = obj.get(index, snap_id)
