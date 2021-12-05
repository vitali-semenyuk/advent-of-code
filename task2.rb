input = File.read 'input2.txt'

commands = input.lines.map(&:strip)

x = 0
y = 0

commands.each do |c|
  action, value = c.split(' ')
  case action
  when 'forward'
    x += value.to_i
  when 'up'
    y -= value.to_i
  when 'down'
    y += value.to_i
  else
    raise 'Error'
  end
end

puts "X: #{x}; Y: #{y}"
puts x * y

# require 'pry'
# binding.pry

x = 0
y = 0
aim = 0

commands.each do |c|
  action, value = c.split(' ')
  case action
  when 'forward'
    x += value.to_i
    y += aim * value.to_i
  when 'up'
    aim -= value.to_i
  when 'down'
    aim += value.to_i
  else
    raise 'Error'
  end
end

puts "X: #{x}; Y: #{y}"
puts x * y

