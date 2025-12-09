# Tic-Tac-Toe

A Rust/Bevy learning project

## Implementation overview



### Components

##### Cell

- Position
- Value

##### GameState

- isXTurn -> true if it is X's turn, false if it's O's turn.
- Grid -> A 3 x 3 matrix of Cell.

### System

- initialization
    - creates the GameState with 'empty' values
- draw the grid
    - will create the Cell's components and put then in the Grid
- draw the current turn (X or Y)
    - Adds a sprite to the left of the grid that read isXTurn
- listen to click in a cell
- draw selected cell
- check for winner
    - if winner: draw win
    - end
- change player turn
- repeat from 'draw the current turn'

### Notes

- Use picking to detect which cell was clicked: https://bevy.org/examples-webgpu/picking/sprite-picking/

- grid/cells are drawn using sprites like in the picking example
