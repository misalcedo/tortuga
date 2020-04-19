require 'stringio'

module Tortuga
  module Lexical
    UnsupportedKindError = Class.new(ArgumentError)

    class Lexeme
      attr_reader :kind, :line, :column

      def initialize(kind, line, column)
        raise UnsupportedKindError, "Unsupported kind #{kind} of lexeme." unless self.class.kinds.include?(kind)

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

      def self.kinds
        Set[:message_delimiter, :identifier, :integer, :concurrency_delimiter].freeze
      end

      def self.determine_kind(character)
        case character
        when /[()]/
          :message_delimiter
        when /[\r\n]/
          :concurrency_delimiter
        when /[[:digit:]]/
          :integer
        when /[[:alpha:]]/
          :identifier
        else
          nil
        end
      end
    end
  end
end
