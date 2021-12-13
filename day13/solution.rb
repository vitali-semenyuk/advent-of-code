# frozen_string_literal: true

def make_fold(paper, dir, point)
  if dir == 'y'
    y = point
    paper[...y].each.with_index do |_row, i|
      paper[i] = paper[i].map.with_index { _1 == '#' ? _1 : paper[point * 2 - i][_2] }
    end
  else
    x = point
    paper.each.with_index do |row, i|
      paper[i] = row[...x].map.with_index { _1 == '#' ? _1 : row[x * 2 - _2] }
    end
  end

  paper[...y]
end

dots, folds = input.split("\n\n")
dots = dots.lines.map { _1.split(',').map(&:to_i) }
folds = folds.lines.map { _1.split.last.split('=') }

N = dots.map(&:max).max + 1

paper = Array.new(N) { Array.new(N, '.') }

dots.each do |dot|
  paper[dot.last][dot.first] = '#'
end

folds.each.with_index do |fold, i|
  paper = make_fold(paper, fold.first, fold.last.to_i)

  p1 paper.sum { _1.count('#') } if i.zero?
end

p2 paper.map(&:join).join("\n")

__END__
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
