RSpec.describe Hyperb do
  it 'has a version number' do
    expect(Hyperb::VERSION).not_to be nil
  end

  it 'can make a basic get request' do
    # Todo: figure a way to stub this since webmock won't.
    result = Hyperb.get('http://www.example.com')

    expect(result['status']).to eq('200 OK')
  end
end
