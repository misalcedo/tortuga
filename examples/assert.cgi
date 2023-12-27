#!/usr/bin/env ruby

if ARGV.any?
  abort("Invalid command-line arguments.")
end

if ENV["PWD"] != __dir__
  abort("Working directory (#{ENV["PWD"]}) must be the parent directory of the script (#{__dir__}).")
end

ENV.to_h.each do |key, value|
  if key.start_with?("HTTP") && (key.upcase != key || key.include?("-"))
    abort("Protocol meta-variables must be upper case and not contain any dashes (i.e. '-').")
  end
end

input = STDIN.read
length = ENV["HTTP_CONTENT_LENGTH"]&.to_i

if length != input.size
  abort("Input stream length (#{input.size}) did not match the content length (#{length}).")
end

STDOUT.write("\r\n")
STDOUT.write(input)
