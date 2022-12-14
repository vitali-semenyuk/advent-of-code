data = <<-STR
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
STR
data = File.read('./tasks/day7.txt')

Node = Struct.new(:name, :parent, :children, :size, keyword_init: true) do
  def is_dir?
    !!children
  end

  def size
    if is_dir?
      children.sum(&:size)
    else
      values.last
    end
  end

  def ls_r
    return [] unless is_dir?

    dirs = children.filter { _1.children }
    [*dirs, *dirs.map(&:ls_r)].flatten
  end
end

root = Node.new(name: '/', children: [])
current_dir = root

data[1..].split("$ ").each do |command|
  command.strip!

  if command.start_with?('cd')
    _, dir = command.split
    current_dir = if dir == '/'
      root
    elsif dir == '..'
      current_dir.parent
    else
      current_dir.children.find { _1.name == dir }
    end
  else
    _, *content = command.lines
    content.map do |line|
      size, name = line.strip.split
      child = if size == 'dir'
        Node.new(name:, parent: current_dir, children: [])
      else
        Node.new(name:, size: size.to_i)
      end
      current_dir.children.push(child)
    end
  end
end

pp root.ls_r.filter { _1.size <= 100_000 }.sum(&:size)

total_size = 70_000_000
free_space = total_size - root.size
lacking_size = 30_000_000 - free_space

pp root.ls_r.sort_by { _1.size }.find { _1.size > lacking_size }.size
