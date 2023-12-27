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

unless input.empty?
  unless ENV.include?("CONTENT_LENGTH")
    abort("Missing content length despite the request having a body.")
  end

  length = ENV["CONTENT_LENGTH"].to_i

  unless input.size == length
    abort("Input stream length (#{input.size}) did not match the content length (#{length}).")
  end

  unless ENV.include?("CONTENT_TYPE")
    abort("If the request includes a body, the CONTENT_TYPE must be set to the Media Type of the body.")
  end
end

unless ENV["GATEWAY_INTERFACE"] == "CGI/1.1"
  abort("'#{ENV["GATEWAY_INTERFACE"]}' is an invalid gateway interface.")
end

if ENV.include?("PATH_INFO")
  require "cgi"

    unless ENV["PATH_INFO"] == CGI::unescape(ENV["PATH_INFO"])
      abort("Path info '#{ENV["PATH_INFO"]}' must not be URL-escaped.")
    end

    unless ENV["PATH_TRANSLATED"] == File.join(Dir.pwd, ENV["PATH_INFO"])
      abort("Path translated '#{ENV["PATH_TRANSLATED"]}' must be resolved by the document root (i.e. #{File.join(Dir.pwd, ENV["PATH_INFO"])}).")
    end
end

STDOUT.write("\r\n")
STDOUT.write(input)
