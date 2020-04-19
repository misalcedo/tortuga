require 'rspec'
require 'tortuga/lexical/lexeme'

RSpec.describe Tortuga::Lexical::Lexeme do
  subject { described_class.new(:identifier, 1, 2) }

  context 'when no content' do
    it 'returns an empty string' do
      expect(subject.content).to be_empty
    end
  end

  context 'when lexeme has content' do
    let(:message) { "Hello, World!".split(//) }

    before do
      message.each { |character| subject << character }
    end

    it 'returns the buffered content' do
      expect(subject.content).to eq message.join('')
    end
  end

  context 'when lexeme is created with content' do
    let(:message) { "Hello, World!" }

    subject { described_class.new(:identifier, 1, 2, message) }

    it 'returns the content' do
      expect(subject.content).to eq message
    end
  end

  context 'when adding multiple characters' do
    let(:message) { "Hello, World!" }

    before do
      subject << message
    end

    it 'returns the buffered content' do
      expect(subject.content).to eq message[0]
    end
  end
end