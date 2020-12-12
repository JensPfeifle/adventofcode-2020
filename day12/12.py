import turtle

with open("12.in") as f:
    input = f.read().strip().splitlines()

instructions = [(i[0], float(i[1:])) for i in input]
print(instructions)


turtle.mode("logo")  # 0=N
t = turtle.Turtle()
t.setheading(90.0)  # face east
assert t.position() == (0.0, 0.0)
# face east
for d, s in instructions:
    print(d, s)
    if d == "N":
        t.sety(t.ycor() + s)
    if d == "S":
        t.sety(t.ycor() - s)
    if d == "E":
        t.setx(t.xcor() + s)
    if d == "W":
        t.setx(t.xcor() - s)
    if d == "L":
        t.left(s)
    if d == "R":
        t.right(s)
    if d == "F":
        t.forward(s)

x,y = t.position()
print(x,y)
print(abs(x) + abs(y))
turtle.done()
