# Tic-Tac-Toe

A Rust/Bevy learning project.

Prior to this my experience with rust is only parts of the rust book (around chapter 7). 
I've written 3 small CLI apps before: 
- the guessing game (from the book)
- an agenda as a way to exercise what I've learn from the book
- a CLI tic-tac-toe as a way to practice structs and enums specifically

(these can be found [here](https://github.com/slDias/rust-book/tree/main))

Beyond this I've been practicing Rust with [leetcode](https://leetcode.com/u/slDyas/) exercises as well.

## Implementation overview

### Entities

##### Cells

Represents a cell on the 3x3 grid.

A bundle of:

- CellValues with the value of the cell
- A Sprite to draw the square
- A Pickable with an observer for clicks on the sprite
- A transform to position it on screen

##### Restart Button

A button that is used to restart the game

A bundle of:

- Button, a headless button from bevy's experimental_bevy_ui_widgets feature
- Node and some styling as an UI for the button

##### Winner Label

A text label used to display the winner of the game.

A bundle of:

- WinLabel, an empty struct that serves as an identifier.
- Text and some styling.
- Node and some styling.

##### Camera

contains only Bevy's Camera2d to render everything.

### Custom Components

##### CellValues

An enum with value: X, Y or Empty

##### WinLabel

An empty struct used as an identifier

### Systems Resources

##### GameState

A struct with:

- is_x_turn: true if the next play is from X player
- winner: A CellValue member, starts as CellValues::Empty
- board: A matrix that keeps the Entity.id() of each Cell

On each frame all possible winning combinations are checked by querying the CellValues components and using the GameState.Board to fetch the Cell entities throught Query.get().


## Planned Implementation overview

The following are notes of how I invisioned the implementation before doing it.
It does not reflect the current implementation and it is here for a mere comparison on how development went.

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

#### 2025-12-16
Just did some cleaning up on the check winner logic, simplified the cell component a bit more.
I'm satisfied with the code and I believe I've learned enough from this game.

#### 2025-12-15
considering a better architecture:

##### idea 1 - relationship
add 8 relationship between components: up, up_right, right, down_right, down, down_left, left, up_left

##### idea 2 - Matrix of entities
Board as a resource but it is a matrix of entities

ended up developing the matrix of entities idea


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
