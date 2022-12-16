data = <<-STR
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
STR
# data = File.read('./tasks/day16.txt')

graph = data.lines.each_with_object({}) do |line, hash|
  valve, tunnels = line.split('; ')
  parts = valve.split
  name = parts[1]
  _, flow_rate = parts[4].split('=')
  tunnels = tunnels.split[4..].map { _1.gsub(',', '')}

  hash[name] = {name:, flow_rate: flow_rate.to_i, tunnels:}
end

def dfs(graph, v, depth=30, rate=0, visited=[])
  return 1 if depth == 0
  return 1 if visited.size == graph.size

  pp visited

  if !visited.include?(v[:name])
    if v[:flow_rate] > 0
      return dfs(graph, v, depth - 1, rate + v[:flow_rate], [*visited, v[:name]])
    else
      return dfs(graph, v, depth, rate,[*visited, v[:name]])
    end
  end

  results = v[:tunnels].map do |t|
    dfs(graph, graph[t], depth - 1, rate, visited)
  end

  results.max
end

pp graph
puts
pp dfs(graph, graph['AA'])
