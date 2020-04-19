require 'stringio'

module Tortuga
  module Lexical
    class Lexeme
      attr_reader :kind, :line, :column

      def initialize(kind, line, column, string="")
        @kind = kind
        @line = line
        @column = column
        @content = StringIO.new(string.dup)
      end

      def content
        @content.string
      end

      def <<(character)
        @content.putc(character)
      end

      def ==(other)
        @kind == other&.kind &&@line == other&.line && @column == other&.column && content == other&.content
      end
    end
  end
end
