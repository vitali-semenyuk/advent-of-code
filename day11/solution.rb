# frozen_string_literal: true

SIZE = 10

def adj(point)
  x = point.first
  y = point.last

  [
    [x, y+1],
    [x, y-1],
    [x-1, y],
    [x+1, y],
    [x-1, y-1],
    [x-1, y+1],
    [x+1, y-1],
    [x+1, y+1],
  ].filter { _1 >= 0 && _2 >= 0 && _1 < SIZE && _2 < SIZE }
end

def bfs(matrix, queue)
  visited = Array.new(SIZE) { Array.new(SIZE, false) }

  loop do
    break if queue.empty?

    cur = queue.shift
    next if visited[cur.last][cur.first]

    visited[cur.last][cur.first] = true

    adj(cur).each do |a|
      matrix[a.last][a.first] += 1

      if !visited[a.last][a.first] && matrix[a.last][a.first] > 9
        queue << a
      end
    end
  end
end

octopus = lines.map { _1.split('').map(&:to_i) }

flashes = 0
sync = nil
1000.times do |i|
  targets = []

  octopus.each.with_index do |row, y|
    row.each.with_index do |oct, x|
      octopus[y][x] += 1
      targets << [x,y] if octopus[y][x] > 9
    end
  end

  bfs(octopus, targets)

  octopus.each.with_index do |row, y|
    row.each.with_index do |oct, x|
      if octopus[y][x] > 9
        octopus[y][x] = 0
        flashes += 1 if i < 100
      end
    end
  end

  if octopus.all? { |r| r.all?(&:zero?) }
  sync = i
  break
  end
end


p1 flashes

p2 sync + 1

__END__
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
