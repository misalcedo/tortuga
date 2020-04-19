require 'rspec'
require 'tortuga/lexical/lexer'
require 'tortuga/lexical/lexeme'

RSpec.describe Tortuga::Lexical::Lexer do
  subject { described_class.new(characters) }

  context 'when no characters' do
    let(:characters) { [] }

    it 'returns an empty sequence of lexemes' do
      expect(subject.to_a).to be_empty
    end
  end

  context 'when an empty line' do
    let(:characters) { "\r\n".split(//) }

    it 'returns only a single lexeme' do
      expect(subject.to_a).to eq [Tortuga::Lexical::Lexeme.new(:concurrency_delimiter, 1, 1, "\r\n")]
    end
  end

  context 'when only a single line' do
    let(:characters) { "(add 1 2)\r\n".split(//) }

    it 'succeeds' do
      expect(subject.to_a).to eq [
        Tortuga::Lexical::Lexeme.new(:message_delimiter, 1, 1, "("),
        Tortuga::Lexical::Lexeme.new(:identifier, 1, 2, "add"),
        Tortuga::Lexical::Lexeme.new(:integer, 1, 6, "1"),
        Tortuga::Lexical::Lexeme.new(:integer, 1, 8, "2"),
        Tortuga::Lexical::Lexeme.new(:message_delimiter, 1, 9, ")"),
        Tortuga::Lexical::Lexeme.new(:concurrency_delimiter, 1, 10, "\r\n")
      ]

    end
  end

  context 'when determining the kind' do
    let(:characters) { [] }

    context 'when the character is a digit' do
      it 'returns integer kind' do
        expect(subject.determine_kind('1')).to eq :integer
      end
    end
    
    context 'when the character is a letter' do
      it 'returns identifier kind' do
        expect(subject.determine_kind('Ä')).to eq :identifier
      end
    end

    context 'when the character is a new line or carriage return' do
      it 'returns concurrency delimiter kind' do
        expect(subject.determine_kind("\n")).to eq :concurrency_delimiter
        expect(subject.determine_kind("\r")).to eq :concurrency_delimiter
      end
    end

    context 'when the character is a parenthesis' do
      it 'returns message delimiter kind' do
        expect(subject.determine_kind('(')).to eq :message_delimiter
        expect(subject.determine_kind(')')).to eq :message_delimiter
      end
    end

    context 'when the character is an unknown kind' do
      it 'raises an error' do
        expect { subject.determine_kind("\0") }.to raise_error(Tortuga::Lexical::InvalidCharacterError)
      end
    end
  end
end