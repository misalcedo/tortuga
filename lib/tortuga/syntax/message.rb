module Tortuga
    module Syntax
        ParseError = Class.new(RuntimeError)

        class Message
            def initialize(lexemes)
                @lexemes = lexemes
            end

            def parts
                @lexemes.map do |lexeme|
                    case lexeme.kind
                    when :integer
                        lexeme.content.to_i
                    else
                        raise ParseError, "Encountered an unexpected lexeme #{lexeme.content.inspect} in message at line #{lexeme.line}, column #{lexeme.column}."
                    end
                end
            end
        end
    end
end