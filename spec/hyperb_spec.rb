RSpec.describe Hyperb do
  it "has a version number" do
    expect(Hyperb::VERSION).not_to be nil
  end

  it "doesn't error when requiring the native" do
    require 'hyperb/native'
  end
end
