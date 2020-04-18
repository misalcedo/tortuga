module Tortuga
  module Io
    FileNotFoundError = Class.new(IOError)

    class FileReader
      include Enumerable

      attr_reader :path, :encoding

      def initialize(path, encoding)
        @path = path
        @encoding = encoding
      end

      def each
        File.open(path, external_encoding: encoding) do |file|
          loop do
            yield file.readchar
          rescue EOFError
            break
          end
        end
      rescue Errno::ENOENT
        raise FileNotFoundError, "File not found at specified path: #{path}."
      end
    end
  end
end
