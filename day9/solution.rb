# frozen_string_literal: true

MAP = input.split.map { _1.chars.map(&:to_i) }
h = MAP.size
w = MAP.first.size
ADJ_P = ->(x, y) { [[x, y + 1], [x, y - 1], [x - 1, y], [x + 1, y]].filter { _1 >= 0 && _2 >= 0 && _1 < w && _2 < h } }
ADJ = ->(x, y) { ADJ_P.call(x, y).map { MAP[_2][_1] } }

lower_points = []
MAP.each.with_index do |row, y|
  row.each.with_index do |cell, x|
    lower_points << [x, y] if ADJ.call(x, y).all? { _1 > cell }
  end
end

risk = lower_points.reduce(0) { |acc, (x, y)| acc + MAP[y][x] + 1 }
p1 risk

def dfs(point, visited = [])
  visited << point

  res = 1

  ADJ_P.call(point.first, point.last).each do |pnt|
    res += dfs(pnt, visited) if !visited.include?(pnt) && MAP[pnt.last][pnt.first] != 9
  end

  res
end

p2 lower_points.map { dfs(_1) }.sort.last(3).reduce(:*)

__END__
2199943210
3987894921
9856789892
8767896789
9899965678
