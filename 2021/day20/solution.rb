# frozen_string_literal: true

alg, image = input.split("\n\n")
alg = alg.split("\n").join
image = image.split("\n").map(&:chars)

def adj(image, x, y, base)
  [[x - 1, y - 1], [x, y - 1], [x + 1, y - 1], [x - 1, y], [x, y], [x + 1, y], [x - 1, y + 1],
   [x, y + 1], [x + 1, y + 1]].map do |x, y|
    (image[y] || [])[x] || base
  end
end

def expand(img, chr)
  ([[chr] * (img.first.size + 4)] * 2) +
    img.map { ([chr] * 2) + _1 + ([chr] * 2) } +
    ([[chr] * (img.first.size + 4)] * 2)
end

def narrow(img)
  img[1...-1].map { _1[1...-1] }
end

def apply(img, alg, base)
  copy = []
  img.each.with_index do |row, y|
    copy << []
    row.each.with_index do |_pix, x|
      copy[y] << adj(img, x, y, base).map { _1 == '#' ? '1' : '0' }.join.to_i(2)
    end
  end
  copy.map { |row| row.map { alg[_1] } }
end

def solve(img, alg, iters)
  image = expand(img, '.')

  iters.times do
    image = expand(image, image[0][0])
    image = apply(image, alg, image[0][0])
  end

  image = narrow(image)
  image.map { |r| r.select { _1 == '#' }.size }.sum
end

# puts image.map(&:join).join("\n")

p1 solve(image, alg, 2)

p2 solve(image, alg, 50)

__END__
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
