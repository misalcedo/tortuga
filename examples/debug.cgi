#!/usr/bin/env ruby

puts "Content-type: text/html\r\n\r"

require "json"

puts "Environment:"
puts JSON.pretty_generate(ENV.to_h)

puts "Arguments:"
p ARGV

puts "STDIN:"
p STDIN.read

STDERR.puts "Done"
