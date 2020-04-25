require 'tortuga/syntax/transmission'
require 'tortuga/syntax/actor_reference'
require 'tortuga/syntax/message'

module Tortuga
    module Syntax
        SyntaxError = Class.new(RuntimeError)

        class Parser
            def initialize(lexer)
                @lexer = lexer
            end

            def parse
                @lexer
                    .chunk { |lexeme| lexeme.kind == :concurrency_delimiter }
                    .reject {|(skip, transmission)| skip }
                    .map {|(skip, transmission)| transmission }
                    .map { |transmission| create_transmission(transmission) }
            end

            def create_transmission(transmission)
                start_message, actor_reference, *message, end_message = transmission

                validate!(start_message, :message_delimiter)                 
                validate!(actor_reference, :identifier)                 
                
                message.each { |part| validate!(part, :integer) }

                validate!(end_message, :message_delimiter)
                
                Transmission.new(ActorReference.new(actor_reference), Message.new(message))
            end

            def validate!(lexeme, kind)
                unless lexeme.kind == kind
                    raise SyntaxError, "Encountered an unexpected lexeme #{lexeme.content.inspect} at line #{lexeme.line}, column #{lexeme.column}."
                end
            end
        end
    end
end