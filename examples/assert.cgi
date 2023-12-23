#!/usr/bin/env ruby

if ARGV.any?
  abort("Invalid command-line arguments.")
end

if ENV["PWD"] != __dir__
  abort("Working directory must be the parent directory of the script.")
end

ENV.to_h.each do |key, value|
  if key.start_with?("HTTP") && (key.upcase != key || key.include?("-"))
    abort("Protocol meta-variables must be upper case and not contain any dashes (i.e. '-').")
  end
end

STDOUT.write("\r\n")
STDOUT.write(STDIN.read)
