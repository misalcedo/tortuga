module Tortuga
  module Lexical
    class Lexer
      include Enumerable

      def initialize(characters)
        @characters = characters
      end

      def each
        @characters.each do |character|
          
        end
      end
    end
  end
end
