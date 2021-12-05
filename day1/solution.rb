# frozen_string_literal: true

depths = ints

inc = 0
depths.each.with_index do |d, i|
  next if i.zero?

  inc += 1 if depths[i - 1] < d
end

p1 inc

temp = depths.map.with_index do |d, i|
  next if i.zero?

  depths[i - 1] + d + depths[i + 1] if i + 1 < depths.size
end.compact

inc = 0
temp.each.with_index do |d, i|
  next if i.zero?

  inc += 1 if temp[i - 1] < d
end

p2 inc

__END__
199
200
208
210
200
207
240
269
260
263
