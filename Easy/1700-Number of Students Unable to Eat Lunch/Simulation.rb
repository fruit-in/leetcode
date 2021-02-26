# @param {Integer[]} students
# @param {Integer[]} sandwiches
# @return {Integer}
def count_students(students, sandwiches)
  students = Containers::Queue.new(students)
  count = [0, 0]
  i = 0

  students.each do |prefer|
    count[prefer] += 1
  end

  until students.empty?
    prefer = students.pop
    if prefer == sandwiches[i]
      i += 1
      count[prefer] -= 1
    elsif count[0] == 0 || count[1] == 0
      students.push(prefer)
      break
    else
      students.push(prefer)
    end
  end

  students.size
end
