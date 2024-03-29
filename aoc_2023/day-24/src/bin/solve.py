import sympy
with open('./src/bin/input.txt', 'r') as file:
   hailstones = [tuple(map(int, line.replace("@", ",").split(","))) for line in file.read().splitlines()]

xr, yr, zr, vxr, vyr, vzr = sympy.symbols("xr, yr, zr, vxr, vyr, vzr")

equations = []

for i, (sx, sy, sz, vx, vy, vz) in enumerate(hailstones):
    equations.append((xr - sx) * (vy - vyr) - (yr - sy) * (vx - vxr))
    equations.append((yr - sy) * (vz - vzr) - (zr - sz) * (vy - vyr))
    if i < 2:
        continue
    # x % 1 == 0 check if values are integers
    answers = [soln for soln in sympy.solve(equations) if all(x % 1 == 0 for x in soln.values())]
    if len(answers) == 1:
        break
    
answer = answers[0]

print("Equations:")
for e in equations: print(e)

print(answer[xr] + answer[yr] + answer[zr])
print(i)