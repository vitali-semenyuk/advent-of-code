# frozen_string_literal: true

inp = lines

lines = inp.map do |line|
  line.split(' -> ').map { _1.split(',').then { |x, y| { x: x.to_i, y: y.to_i } } }
end

hv_lines, diagonales = lines.partition { _1[:x] == _2[:x] || _1[:y] == _2[:y] }
field_side = 1000
field = Array.new(field_side**2) { 0 }

hv_lines.each do |line|
  if line.first[:x] == line.last[:x]
    x = line.first[:x]
    Range.new(*[line.first[:y], line.last[:y]].sort).each { |y| field[(y * field_side) + x] += 1 }
  else
    y = line.first[:y]
    Range.new(*[line.first[:x], line.last[:x]].sort).each { |x| field[(y * field_side) + x] += 1 }
  end
end

p1 field.filter { _1 >= 2 }.size

diagonales.each do |line|
  x0 = line.first[:x]
  y0 = line.first[:y]
  x1 = line.last[:x]
  y1 = line.last[:y]

  xs = Range.new(x0, x1).step(x0 > x1 ? -1 : 1).to_a
  ys = Range.new(y0, y1).step(y0 > y1 ? -1 : 1).to_a
  xs.zip(ys).each { |x, y| field[(y * field_side) + x] += 1 }
end

p2 field.filter { _1 >= 2 }.size

__END__
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
