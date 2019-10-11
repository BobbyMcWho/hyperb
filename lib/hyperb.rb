require 'helix_runtime'

begin
  require 'hyperb/native'
rescue LoadError
  warn 'Unable to load text_transform/native. Please run `rake build`'
end

require 'hyperb/version'
