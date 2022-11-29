# frozen_string_literal: true

fishes = input.split(',')
              .map(&:to_i)
              .each_with_object(Array.new(9, 0)) do |lifetime, fishes|
  fishes[lifetime] += 1
end

def solve(fishes, n)
  n.times do
    fishes.rotate!
    fishes[6] += fishes.last
  end
  fishes.sum
end

p1 solve(fishes.dup, 80)
p2 solve(fishes.dup, 256)

__END__
3,4,3,1,2
