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

CACHE = {}
def call_cached(graph, v, depth, rate, enabled)
  key = [v[:name], depth, rate]
  res = CACHE[key]
  return res if res

  res = dfs(graph, v, depth, rate, enabled)
  CACHE[key] = res
  res
end

def dfs(graph, v, depth=30, rate=0, enabled=[])
  return 0 if depth == 0

  results = v[:tunnels].map do |t|
    call_cached(graph, graph[t], depth - 1, rate, enabled) + rate
  end
  if !enabled.include?(v[:name]) && v[:flow_rate] > 0
    results << (call_cached(graph, v, depth - 1, rate + v[:flow_rate], [*enabled, v[:name]]) + rate)
  end

  results.max
end

pp dfs(graph, graph['AA'])

# part 1 = 1986

CACHE_2 = {}
def call_cached_2(graph, v, w, depth, rate, enabled)
  key = [v[:name], w[:name], depth, rate, enabled.hash]
  res = CACHE_2[key]
  return res if res

  res1 = dfs_2(graph, v, w, depth, rate, enabled)
  # if res && res != res1
    # pp(v: v[:name], w: w[:name], depth:, rate:, enabled:)
    # pp(key:, res:, res1:)

    # exit 1
  # end

  CACHE_2[key] = res1
  res1
end

def dfs_2(graph, v, w, depth=26, rate=0, enabled=[])
  return 0 if depth == 0

  results = v[:tunnels].product(w[:tunnels]).map do |t, el|
    call_cached_2(graph, graph[t], graph[el], depth - 1, rate, enabled) + rate
  end
  if !enabled.include?(v[:name]) && v[:flow_rate] > 0 && !enabled.include?(w[:name]) && w[:flow_rate] > 0 && v[:name] != w[:name]
    results << (call_cached_2(graph, v, w, depth - 1, rate + v[:flow_rate] + w[:flow_rate], [*enabled, v[:name], w[:name]]) + rate)
  end
  if !enabled.include?(v[:name]) && v[:flow_rate] > 0
    w[:tunnels].each do |el|
      results << (call_cached_2(graph, v, graph[el], depth - 1, rate + v[:flow_rate], [*enabled, v[:name]]) + rate)
    end
  end
  if !enabled.include?(w[:name]) && w[:flow_rate] > 0
    v[:tunnels].each do |t|
      results << (call_cached_2(graph, graph[t], w, depth - 1, rate + w[:flow_rate], [*enabled, w[:name]]) + rate)
    end
  end

  results.max
end

pp dfs_2(graph, graph['AA'], graph['AA'])

# part 2 = 2431 - too low
