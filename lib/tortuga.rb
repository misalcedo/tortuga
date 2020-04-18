require "tortuga/version"
require 'tortuga/io/file_reader'
require 'tortuga/lexical/lexer'

module Tortuga
  class Error < StandardError; end

  def self.interpret(path, encoding)
    reader = Io::FileReader.new(path, encoding)
    lexer = Lexical::Lexer.new(reader)

    puts lexer.to_a
  end
end
