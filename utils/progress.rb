FULL_SYMBOL = '⭐'
EMPTY_SYMBOL = ' '

def progress_bar(progress, max: 100, scale: 10)
  value = [progress, max].min

  step = max / scale

  full_blocks = (value / step).to_i
  empty_blocks = scale - full_blocks

  FULL_SYMBOL * full_blocks + EMPTY_SYMBOL * empty_blocks
end

# Array.from(document.querySelectorAll('.eventlist-event')).map(el => [el.getElementsByTagName('a')[0].text.slice(1, -1), +el.getElementsByClassName('star-count')[0]?.textContent.slice(0, -1)])

require 'json'

data = JSON.parse(File.read('data.json'))
total = data.reduce(0) { |acc, (_, progress)| acc + progress.to_i }
data = data.reverse.map do |year, progress|
  progress = progress.to_i
  bar = progress_bar(progress, max: 50, scale: 10)
  "| #{year}  | #{progress.to_s.rjust(5)} | #{bar} |"
end
data << "| Total | #{total.to_s.rjust(5)} | #{progress_bar(total, max: 500, scale: 10)} |"

puts data.join("\n")
