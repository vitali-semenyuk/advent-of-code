# frozen_string_literal: true

require 'algorithms'

cave = input.lines.map { _1.strip.chars.map(&:to_i) }

def dfs(cave)
  w = cave.first.size
  h = cave.size
  n = w * h
  adj = lambda { |point|
    [[point.first, point.last + 1], [point.first, point.last - 1], [point.first - 1, point.last],
     [point.first + 1, point.last]].select { _1 >= 0 && _2 >= 0 && _1 < w && _2 < h }
  }

  d = Array.new(n, 99_999_999)
  p = Array.new(n)

  queue = Containers::PriorityQueue.new { |x, y| (x <=> y) == -1 }
  queue.push(0, 0)

  d[0] = 0
  p[0] = 0

  loop do
    break if queue.empty?

    cur = queue.pop

    y = cur / w
    x = cur - (y * w)

    adj.call([x, y]).each do |v|
      vv = (v.last * w) + v.first
      # next if !queue.include?(vv)

      alt = d[cur] + cave[v.last][v.first]
      next unless alt < d[vv]

      d[vv] = alt
      p[vv] = cur
      queue.push(vv, d[vv])
    end
  end

  d[(h * w) - 1]
end

p1 dfs(cave)

cave.each do |row|
  rs = row.size
  (rs * 4).times do |i|
    row[rs + i] = row[i] + 1
    row[rs + i] = 1 if row[rs + i] > 9
  end
end

ls = cave.size
(ls * 4).times do |i|
  cave[ls + i] = cave[i].map { _1 == 9 ? 1 : _1 + 1 }
end

p2 dfs(cave)

__END__
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
