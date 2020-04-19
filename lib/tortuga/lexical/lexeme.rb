require 'stringio'

module Tortuga
  module Lexical
    UnsupportedKindError = Class.new(ArgumentError)

    class Lexeme
      attr_reader :kind, :line, :column

      def initialize(kind, line, column)
        raise UnsupportedKindError, "Unsupported kind #{kind} of lexeme." unless self.class.types.include?(kind)

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

      def self.types
        Set[:identifier, :integer].freeze
      end
    end
  end
end
