module Tortuga
  module Lexical
    InvalidCharacterError = Class.new(RuntimeError)
    LexicalError = Class.new(RuntimeError)

    class Lexer
      include Enumerable

      def initialize(characters)
        @characters = characters
        @lexeme = nil
        @line = 1
        @column  = 0
      end

      def each
        @characters.each do |character|
          next_kind = self.determine_kind(character)

          @column += 1
          @lexeme ||= Lexeme.new(next_kind, @line, @column)
          
          if @lexeme.kind != next_kind
            terminate_lexeme
            
            yield @lexeme if should_yield?

            @lexeme = Lexeme.new(next_kind, @line, @column)
          end

          @lexeme << character
        end

        terminate_lexeme
        
        yield @lexeme if should_yield?
      end

      def determine_kind(content)
        case content
        when /[()]/
          :message_delimiter
        when /[\r\n]/
          :concurrency_delimiter
        when /[[:digit:]]/
          :integer
        when /[[:alpha:]]/
          :identifier
        when /[[:blank:]]/
          :blank
        else
          raise InvalidCharacterError, "Encountered an unexpected character at line #{@line}, column #{@column}."
        end
      end

      private

      def terminate_lexeme
        return unless @lexeme

        raise LexicalError, "Invalid lexeme #{@lexeme.content.inspect} (#{@lexeme.kind}) at line #{@line}, column #{@column}." unless valid?

        case @lexeme.kind
        when :concurrency_delimiter
          @line += 1
          @column = 0
        end
      end

      def valid?
        @lexeme.content =~ validation_expression
      end
      
      def validation_expression
        case @lexeme.kind
        when :message_delimiter
          /[()]/
        when :concurrency_delimiter
          /\r?\n/
        when :integer
          /[[:digit:]]+/
        when :identifier
          /[[:alpha:]]+/
        when :blank
          /[[:blank:]]+/
        end
      end

      def should_yield?
        @lexeme && @lexeme.kind != :blank
      end
    end
  end
end
