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


## Devlog notes

#### 2025-12-15
considering a better architecture:

##### idea 1 - relationship
add 8 relationship between components: up, up_right, right, down_right, down, down_left, left, up_left

##### idea 2 - Matrix of entities
Board as a resource but it is a matrix of entities


#### 2025-12-11
regarding the ownership I noted yesterday. it's all references.
what confused me was the ResMut<GameState>. because I'd expected it to have an &.
so I have to keep in mind that & is used to declare a reference but variable that keep references don't need &.

#### 2025-12-10
Todo: check for win diagonally
log: Just made it work by keeping 2 components: 
- cell index which is part of an entity with the sprite
- the grid which keeps the value of the cell

I don't understand how bevy handles ownership within components. 
 for example how am i able to query for a mut (which doesnt seem to be a reference)
 and bevy does not lose ownership over it.