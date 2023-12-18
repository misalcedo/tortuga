#!/usr/bin/env ruby

puts "Content-type: text/html\n\n"

require "json"

puts "Environment:"
puts JSON.pretty_generate(ENV.to_h)

puts "Arguments:"
p ARGV

puts "STDIN:"
p STDIN.read

STDERR.puts "Done"
