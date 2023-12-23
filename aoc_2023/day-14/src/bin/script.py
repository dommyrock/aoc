with open('./src/bin/input.txt', 'r') as file:
    grid = tuple(file.read().splitlines())

def cycle():
    global grid
    for _ in range(4):
        grid = tuple(map("".join, zip(*grid)))
        grid = tuple("#".join(["".join(sorted(tuple(group), reverse=True)) for group in row.split("#")]) for row in grid)
        grid = tuple(row[::-1] for row in grid)

seen = {grid}
array = [grid]

iter = 0

while True:
    iter += 1
    cycle()
    if grid in seen:
        break
    seen.add(grid)
    array.append(grid)
    
first = array.index(grid)

print(f"(1B -  {first}) % ({iter} - {first}) + {first}")
    
grid = array[(1000000000 - first) % (iter - first) + first]

print(sum(row.count("O") * (len(grid) - r) for r, row in enumerate(grid)))