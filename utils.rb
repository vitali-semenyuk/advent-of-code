# frozen_string_literal: true

require 'pry'

def real_input
  @real_input ||= File.read('input.txt')
end

def test_input
  @test_input ||= File.read('solution.rb').split("__END__\n", 2).last
end

def input
  $env == 'test' ? test_input : real_input
end

def lines
  input.lines.map(&:strip)
end

def ints
  lines.map(&:to_i)
end

def p1(arg)
  puts 'First task:'
  puts arg
end

def p2(arg)
  puts 'Second task:'
  puts arg
end
