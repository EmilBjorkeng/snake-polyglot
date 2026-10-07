# Snake - Python

A graphical Snake game written in Python using [pygame](https://www.pygame.org/).

## Running

Requires Python 3 and pygame.

```
git clone https://github.com/EmilBjorkeng/snake-polyglot.git
cd snake-polyglot/python
python snake.py
```

Use the arrow keys to move. When the game ends, the reason and final length are printed in the terminal.

### Installing pygame

```
pip install pygame
```

On some Linux distributions pip refuses to install packages system-wide.
In that case, install pygame from your package manager instead, or use a virtual environment.

## How it works

The board is a grid of cells drawn as squares in a pygame window.
Key presses are buffered, so quick turns between two movement steps are not lost.

## Story

This is very old code from early in my programming career. I was especially proud of the input buffer, which I managed to add all by myself.
