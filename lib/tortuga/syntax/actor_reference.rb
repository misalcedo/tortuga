module Tortuga
    module Syntax
        class ActorReference
            def initialize(lexeme)
                @lexeme = lexeme
            end

            def identifier
                @lexeme.content
            end
        end
    end
end