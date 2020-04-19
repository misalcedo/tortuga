module Tortuga
  module Lexical
    class Lexer
      include Enumerable

      def initialize(characters)
        @characters = characters
        @lexeme = nil
      end

      def each
        line, column = 1, 0

        @characters.each do |character|
          column += 1

          case @lexeme&.kind
          when :identifier
          when :integer
          when :message_delimiter

          when :concurrency_delimiter
            line += 1
            column = 0
          when nil
            # This is the first character.
            kind = Lexeme.determine_kind(character)
          end
        end
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
