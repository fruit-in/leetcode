# @param {String} coordinates
# @return {Boolean}
def square_is_white(coordinates)
  coordinates[0].ord % 2 != coordinates[1].ord % 2
end
