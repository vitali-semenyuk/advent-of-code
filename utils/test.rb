require 'colorize'

def log(message, color)
  hr = '=' * 16
  puts "#{hr} [AoC] #{message} #{hr}".colorize(color)
end

def info(message)
  log(message, :light_blue)
end

def success(message)
  log(message, :light_green)
end

def error(message)
  log(message, :red)
end

years = %w(2015 2016 2017 2018 2019 2020 2022 2023).freeze

years.each do |year|
  info "Running tests for year #{year}"

  `(cd #{year} && cargo build -q && cargo clippy -- -D warnings && cargo test)`

  unless $?.success?
    error "Failed to run tests for year #{year}"
    break
  end

  success "Finished tests for year #{year}"
  puts
end
