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

  context 'when determining the kind' do
    context 'when the character is a digit' do
      it 'returns integer kind' do
        expect(described_class.determine_kind('1')).to eq :integer
      end
    end
    
    context 'when the character is a letter' do
      it 'returns identifier kind' do
        expect(described_class.determine_kind('Ä')).to eq :identifier
      end
    end

    context 'when the character is a newline' do
      it 'returns concurrency delimiter kind' do
        expect(described_class.determine_kind("\n")).to eq :concurrency_delimiter
        expect(described_class.determine_kind("\r")).to eq :concurrency_delimiter
      end
    end

    context 'when the character is a parenthesis' do
      it 'returns message delimiter kind' do
        expect(described_class.determine_kind('(')).to eq :message_delimiter
        expect(described_class.determine_kind(')')).to eq :message_delimiter
      end
    end

    context 'when the character is an unknown kind' do
      it 'returns nil' do
        expect(described_class.determine_kind("\0")).to be_nil
      end
    end
  end
end