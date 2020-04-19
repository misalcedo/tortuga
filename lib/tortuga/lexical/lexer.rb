module Tortuga
  module Lexical
    class Lexer
      include Enumerable

      attr_reader :lexeme

      def initialize(characters)
        @characters = characters
        @lexeme = Lexeme.new()
      end

      def each
        @characters.each do |character|
          
        end
      end
    end
  end
end
