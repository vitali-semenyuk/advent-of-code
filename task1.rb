input = File.read 'input1.txt'

depths = input.lines.map(&:to_i)

inc = 0
depths.each.with_index do |d, i|
  next if i == 0
  inc += 1 if depths[i-1] < d
end

puts 'First task:'
puts inc

temp = depths.map.with_index do |d, i|
  next if i == 0

  depths[i-1] + d + depths[i+1] if i + 1 < depths.size
end.compact

inc = 0
temp.each.with_index do |d, i|
  next if i == 0
  inc += 1 if temp[i-1] < d
end

puts 'Second task:'
puts inc
