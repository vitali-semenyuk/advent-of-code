#!/usr/bin/env ruby
# frozen_string_literal: true

require_relative './utils'

def error(msg)
  puts msg
  exit(1)
end

day = begin
  Integer(ARGV[0])
rescue StandardError
  nil
end
error 'Please specify a day' unless day

$env = %w[test t].include?(ARGV[1]) ? 'test' : 'run'

Dir.chdir("day#{day}")
load 'solution.rb'
