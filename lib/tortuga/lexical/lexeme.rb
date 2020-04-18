module Tortuga
  module Lexical
    class Lexeme
      attr_reader :type, :line, :column

      def initialize(type, line, column)
        @type = type
        @line = line
        @column = column
      end

      def self.types
        [].freeze
      end
    end
  end
end
