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

if ENV.include?("QUERY_STRING")
  # TODO
end

unless ENV.include?("REMOTE_ADDR")
  require "ipaddr"
  address = IPAddr.new(ENV["REMOTE_ADDR"]) rescue nil

  if address.nil?
    abort("Remote address '#{ENV["REMOTE_ADDR"]}' must be a valid IP address.")
  end
end

unless %w[GET HEAD POST PUT DELETE CONNECT OPTIONS TRACE PATCH].include?(ENV["REQUEST_METHOD"])
  abort("Invalid HTTP request method: #{ENV["REQUEST_METHOD"]}.")
end

unless ENV["SCRIPT_NAME"] == "/cgi-bin/#{__FILE__}"
  abort("'#{ENV["SCRIPT_NAME"]}' script name must be a valid URI path prefix (i.e. #{"/cgi-bin/#{__FILE__}"})")
end

if ENV["SERVER_NAME"]
  # TODO
end

unless ENV.include?("SERVER_ADDR")
  require "ipaddr"
  address = IPAddr.new(ENV["SERVER_ADDR"]) rescue nil

  if address.nil?
    abort("Server address '#{ENV["SERVER_ADDR"]}' must be a valid IP address.")
  end
end

unless (0...2**16).include?(ENV["SERVER_PORT"].to_i)
  abort("Server port '#{ENV["SERVER_PORT"]}' must be be a non-negative number smaller than 2^16.")
end

unless %w[HTTP/1.1].include?(ENV["SERVER_PROTOCOL"])
  abort("Server protocol '#{ENV["SERVER_PROTOCOL"]}' must be either HTTP or HTTPS.")
end

if ENV["SERVER_SOFTWARE"]
  program, version = ENV["SERVER_SOFTWARE"].split("/")

  unless program.downcase == "tortuga"
    abort("Server software '#{ENV["SERVER_SOFTWARE"]}' must have 'tortuga' as the program name.")
  end

  version = Gem::Version.new(version) rescue nil

  if version.nil?
    abort("Server software '#{ENV["SERVER_SOFTWARE"]}' does not contain a valid version.")
  end
end

STDOUT.write("\r\n")
STDOUT.write(input)
