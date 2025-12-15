use bevy::{prelude::*};
use bevy::{ui_widgets::{Activate, UiWidgetsPlugins, Button}};


#[derive(Clone, Copy, PartialEq, Debug)]
enum CellValues {
    X,
    Y,
    Empty
}

#[derive(Component)]
struct Cell(CellValues);

#[derive(Component)]
struct WinLabel;


#[derive(Resource)]
struct GameState {
    is_x_turn: bool,
    winner: CellValues,
    board: [[Entity; 3]; 3]
}

pub struct GamePlugin;

impl Plugin for GamePlugin {

    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (initialize_game_state, spawn_buttons));
        app.add_systems(Update, check_for_winner);
    }
}


fn initialize_game_state(mut commands: Commands) {
    commands.spawn(Camera2d);
    let board = draw_board(&mut commands);
    commands.insert_resource(GameState { is_x_turn: true, winner: CellValues::Empty, board });
}

fn draw_board(commands: &mut Commands) -> [[Entity; 3]; 3] {

    const CELL_SIZE: f32 = 50.0;
    const CELL_DISTANCE: f32 = CELL_SIZE + 5.0;
    let mut cell_position = Transform::from_xyz(-CELL_DISTANCE, -CELL_DISTANCE, -1.0);

    core::array::from_fn(|_| {

        let row = core::array::from_fn(|_| {

            let entity = commands
                .spawn((
                    Cell(CellValues::Empty), 
                    Sprite::from_color(Color::WHITE, Vec2::ONE * CELL_SIZE), 
                    cell_position, 
                    Pickable::default()
                ))
                .observe(on_cell_clicked)
                .id();

            cell_position.translation.x += CELL_DISTANCE;
            entity
        });

        cell_position.translation.x = -CELL_DISTANCE;
        cell_position.translation.y += CELL_DISTANCE;

        row
    })
    
}

fn on_cell_clicked(
    event: On<Pointer<Press>>, 
    mut query: Query<(&mut Sprite, &mut Cell)>, 
    mut game_state: ResMut<GameState>, 
    asset_server: Res<AssetServer>
) {

    let (mut sprite, mut cell) = query.get_mut(event.entity).unwrap();

    if cell.0 != CellValues::Empty { return; }

    if game_state.is_x_turn {
        sprite.image = asset_server.load("X.png");
        cell.0 = CellValues::X;
    } else {
        sprite.image = asset_server.load("O.png");
        cell.0 = CellValues::Y;
    }

    game_state.is_x_turn = !game_state.is_x_turn;
}

fn get_winner(game_state: &GameState, cells: Query<&Cell>) -> CellValues {

    fn count_values_in_sequence (sequence: impl Iterator<Item = Entity>, ref_value: CellValues, cells: Query<&Cell>) -> i32 {
        sequence.fold(0,  |res, x| {
            let cell = cells.get(x).unwrap();
            if cell.0 == ref_value { res + 1 } else { res }
        })
    }

    // check horizontal
    for row in game_state.board {
        let first_value = cells.get(row[0]).unwrap().0;
        let count = count_values_in_sequence(row.into_iter(), first_value, cells);
        if count == 3 { return first_value }
    }

    // check vertical
    for column in 0..3 {
        let first_value = cells.get(game_state.board[0][column]).unwrap().0;
        let sequence = game_state.board.into_iter().map(|x| x[column]);
        let count = count_values_in_sequence(sequence, first_value, cells);
        if count == 3 { return first_value }
    }

    // check diagonal up-right
    {
        let first_value = cells.get(game_state.board[0][0]).unwrap().0;
        let sequence = (0..3).map(|i| game_state.board[i][i]);
        let count = count_values_in_sequence(sequence, first_value, cells);
        if count == 3 { return first_value }
    }
    // check diagonal up-left
    {
        let first_value = cells.get(game_state.board[0][2]).unwrap().0;
        let sequence = (0..3).map(|i| game_state.board[i][2 - i]);
        let count = count_values_in_sequence(sequence, first_value, cells);
        if count == 3 { return first_value }
    }

    CellValues::Empty
}

fn check_for_winner(mut game_state: ResMut<GameState>, cells_query: Query<&Cell>, cell_entities: Query<Entity, With<Cell>>, mut commands: Commands) {

    game_state.winner = get_winner(&game_state, cells_query);

    if game_state.winner == CellValues::Empty { return; };

    for obs in cell_entities.iter() {
        commands.entity(obs).remove::<Pickable>();
    }

    commands.spawn((
        Text::new(  if game_state.winner == CellValues::X { "X wins!" } else { "O wins!" }),
        TextFont::from_font_size(30.0),
        Node {
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Start,
            margin: UiRect { left: Val::Px(0.0), right: Val::Px(0.0), top: Val::Px(10.0), bottom: Val::Px(0.0) },
            ..default()
        },
        WinLabel
    ));
}

fn spawn_buttons(mut commands: Commands) {

    let bundle = (
        Button,
        Node { 
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            top: Val::Px(105.0),
            padding: UiRect::all(Val::Px(5.0)),
            ..default() 
        },
        BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
        children![Text::new("Restart")],
    );
    commands.spawn(bundle).observe(on_restart_clicked);
}

type DespawnQueryFilter = Or<(With<Cell>, With<WinLabel>)>;

fn on_restart_clicked(
    _event: On<Activate>,
    despawn_query: Query<Entity, DespawnQueryFilter>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands
) {
    // remove grid and cells
    for ent in despawn_query.iter() {
        commands.entity(ent).despawn();
    }

    game_state.is_x_turn = true;
    game_state.winner = CellValues::Empty;
    game_state.board = draw_board(&mut commands);
}


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(GamePlugin)
    .add_plugins(UiWidgetsPlugins)
    .run();
}
