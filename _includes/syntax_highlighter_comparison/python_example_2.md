```python
import click
from turtle import Turtle
import math

class KochTurtle(Turtle):
    def __init__(self, max_depth: int):
        Turtle.__init__(self)
        self.max_depth = max_depth

    def draw_snowflake(self):
        self._offset_from_home()
        for _ in range(3):
            self._draw_curve(0)
            self.right(120)

    def _offset_from_home(self):
        self.penup()
        self.goto(-300, 100 * math.sqrt(3))
        self.pendown()

    def _draw_curve(self, depth: int):
        if depth == self.max_depth:
            self.forward(600 / 3**depth)
        else:
            for angle in (0, 60, -120, 60):
                self.left(angle)
                self._draw_curve(depth + 1)

@click.command()
@click.option("--max-depth", default=3, help="Fractal recursion depth.")
def main(max_depth: int):
    koch_turtle = KochTurtle(max_depth)
    koch_turtle.draw_snowflake()
    koch_turtle.screen.exitonclick()

if __name__ == "__main__":
    main()
```
{: highlighter="compare"}
