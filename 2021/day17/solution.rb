# frozen_string_literal: true

reg = /^target area: x=([\d-]+)..([\d-]+), y=([\d-]+)..([\d-]+)/
input =~ reg

x1 = Regexp.last_match(1).to_i
x2 = Regexp.last_match(2).to_i
y1 = Regexp.last_match(3).to_i
y2 = Regexp.last_match(4).to_i

xs = (0..x2).map do |vx|
  ys = (y1..1000).map do |vy|
    vvx = vx
    vvy = vy
    x = 0
    y = 0
    1000.times.map do
      x += vvx
      y += vvy
      vvy -= 1
      if vvx.positive?
        vvx -= 1
      elsif vvx.negative?
        vvx += 1
      end
      [x, y]
    end.then do |track|
      track.any? { |x, y| (x1..x2).include?(x) && (y1..y2).include?(y) } ? track : nil
    end
  end.compact
end

p1 xs.flatten(2).max_by { _2 }.last

p2 xs.flatten(1).size

__END__
target area: x=20..30, y=-10..-5
