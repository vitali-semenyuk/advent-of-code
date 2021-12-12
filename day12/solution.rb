# frozen_string_literal: true

$pathes = []

def dfs(g, start, twice = false, visited = [])
  return [*visited, start] if start == 'end'

  g[start].each do |node|
    if !visited.include?(node) || (twice && node != 'start' && visited.tally[node] == 1)
      vis = [*visited]
      vis << start if start.chars.all? { /[[:lower:]]/.match(_1) } || start == 'start'
      $pathes << dfs(g, node, twice && !visited.include?(node), vis)
    end
  end

  visited
end

edges = lines.map { _1.split('-') }

graph = edges.each_with_object({}) do |edge, g|
  g[edge.first] ||= []
  g[edge.last] ||= []
  g[edge.first] << edge.last
  g[edge.last] << edge.first
end

dfs(graph, 'start')

p1 $pathes.select { _1.include?('end') }.size

$pathes = []

dfs(graph, 'start', true)

p2 $pathes.select { _1.include?('end') }.size

__END__
start-A
start-b
A-c
A-b
b-d
A-end
b-end
