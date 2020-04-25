module Tortuga
    module Syntax
        class Transmission
            attr_reader :actor_reference, :message

            def initialize(actor_reference, message)
                @actor_reference = actor_reference
                @message = message
            end
        end
    end
end