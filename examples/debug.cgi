#!/usr/bin/env ruby

require "json"

puts "Environment:"
puts JSON.pretty_generate(ENV.to_h)

puts "Arguments:"
p ARGV

puts "STDIN:"
p STDIN.read
