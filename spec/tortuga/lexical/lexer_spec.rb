require 'rspec'
require 'tortuga/lexical/lexer'
require 'tortuga/lexical/lexeme'

RSpec.describe Tortuga::Lexical::Lexer do
  subject { described_class.new(characters.each) }

  context 'when no characters' do
    let(:characters) { [].each }

    it 'returns an empty sequence of lexemes' do
      expect(subject.to_a).to be_empty
    end
  end

  context 'when an empty line' do
    let(:characters) { '\r\n'.split(//) }

    it 'returns only a single lexeme' do
      expect(subject.to_a).to eq [Tortuga::Lexical::Lexeme.new(:new_line, 1, 1)]
    end
  end

  context 'when only a single line' do
    let(:characters) { '(add 1 2)\r\n'.split(//) }

    it 'succeeds' do
      pending 'Not implemented'
    end
  end
end