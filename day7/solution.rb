# frozen_string_literal: true

values = input.split(',').map(&:to_i)

def cost(values, x)
  values.map { (x - _1).abs }.sum
end

def cost2(values, x)
  values.map { (1..(x - _1).abs).sum }.sum
end

res = (values.min..values.max).map do |target|
  [target, cost(values, target)]
end.sort_by { _2 }

answer = res.first.last
p1 answer

res = (values.min..values.max).map do |target|
  [target, cost2(values, target)]
end.sort_by { _2 }

answer = res.first.last
p2 answer

__END__
16,1,2,0,4,2,7,1,2,14
