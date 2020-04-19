require 'rspec'
require 'tortuga/lexical/lexeme'

RSpec.describe Tortuga::Lexical::Lexeme do
  subject { described_class.new(:identifier, 1, 2) }

  context 'when not a valid kind' do
    it 'raises an error' do
      expect { described_class.new(:foo, 1, 1) }.to raise_error(Tortuga::Lexical::UnsupportedKindError)
    end
  end

  context 'when no content' do
    it 'returns an empty string' do
      expect(subject.contents).to be_empty
    end
  end

  context 'when lexeme has content' do
    let(:message) { "Hello, World!".split(//) }

    before do
      message.each { |character| subject << character }
    end

    it 'returns the buffered content' do
      expect(subject.contents).to eq message.join('')
    end
  end

  context 'when adding multiple characters' do
    let(:message) { "Hello, World!" }

    before do
      subject << message
    end

    it 'returns the buffered content' do
      expect(subject.contents).to eq message[0]
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