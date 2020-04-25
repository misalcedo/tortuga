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
                    when "multiply"
                        puts transmission.message.parts.reduce(1) { |accumulator, part| accumulator * part }
                    end
                end
            end
        end
    end
end