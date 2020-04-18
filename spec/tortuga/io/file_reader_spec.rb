require 'rspec'
require 'tortuga/io/file_reader'
require 'tempfile'

RSpec.describe Tortuga::Io::FileReader do
  let(:file) { Tempfile.new(%w(test .tor)) }
  let!(:path) { file.path }

  subject { described_class.new(path, 'utf-8') }

  context 'when file does not exist' do
    before do
      file.close
      file.unlink
    end

    it 'raises an error' do
      expect { subject.to_a }.to raise_error(Tortuga::Io::FileNotFoundError)
    end
  end

  context 'when the file is empty' do
    after do
      file.unlink
    end

    it 'returns an empty enumerable' do
      expect(subject.each { |_| }).to eq nil
    end
  end

  context 'when the file has an invalid extension' do
    after do
      file.unlink
    end

    it 'returns an empty enumerable' do
      expect(subject.to_a).to be_empty
    end
  end

  context 'when the file exists' do
    let(:contents) { 'Hello, World!' }

    before do
      file.write(contents)
      file.rewind
    end

    after do
      file.unlink
    end

    it 'returns an enumerable' do
      expect(subject.map(&:capitalize).to_a).to eq contents.upcase.split(//)
    end
  end
end