module Tortuga
    module Runtime
        class Interpreter
            def initialize(transmissions)
                @transmissions = transmissions
            end

            def interpret
                @transmissions.each do |transmission|
                    case transmission.actor_reference.identifier
                    when "add"
                        puts transmission.message.parts.sum
                    when "substract"
                        head, *tail = transmission.message.parts

                        puts head - tail.sum
                    end
                end
            end
        end
    end
end