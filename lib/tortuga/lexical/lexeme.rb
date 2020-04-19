require 'stringio'

module Tortuga
  module Lexical
    class Lexeme
      attr_reader :kind, :line, :column

      def initialize(kind, line, column)
        @kind = kind
        @line = line
        @column = column
        @contents = StringIO.new
      end

      def contents
        @contents.string
      end

      def <<(character)
        @contents.putc(character)
      end
    end
  end
end
