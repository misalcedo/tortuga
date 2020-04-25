require "tortuga/version"
require 'tortuga/io/file_reader'
require 'tortuga/syntax/parser'
require 'tortuga/runtime/interpreter'

module Tortuga
  class Error < StandardError; end

  def self.interpret(path, encoding)
    ast = self.parse(path, encoding)
    interpreter = Runtime::Interpreter.new(ast)

    interpreter.interpret()
  end

  # Parse a file in the given encoding into an abstract syntax tree.
  def self.parse(path, encoding)
    reader = Io::FileReader.new(path, encoding)
    lexer = Lexical::Lexer.new(reader)
    parser = Grammar::Parser.new(lexer)

    parser.parse()
  end
end
