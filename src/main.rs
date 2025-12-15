use bevy::{prelude::*};
use bevy::{ui_widgets::{Activate, UiWidgetsPlugins, Button}};


#[derive(Clone, Copy, PartialEq, Debug)]
enum CellValues {
    X,
    Y,
    Empty
}

#[derive(Component)]
struct CellIndex {
    x: usize,
    y: usize
}

#[derive(Component, Clone, Copy)]
struct Grid {
    values: [[CellValues; 3]; 3]
}

#[derive(Component)]
struct WinLabel;


#[derive(Resource)]
struct GameState {
    is_x_turn: bool,
    winner: CellValues,
    processed_winner: bool
}

pub struct GamePlugin;

impl Plugin for GamePlugin {

    fn build(&self, app: &mut App) {

        let on_startup = (
            initialize_game_state,
            spawn_buttons
        ).chain();

        app.add_systems(Startup, on_startup);
        app.add_systems(Update, check_for_winner);
    }
}


fn initialize_game_state(mut commands: Commands) {

    commands.spawn(Camera2d);
    commands.spawn(Grid { values: [[CellValues::Empty; 3]; 3] });
    commands.insert_resource(GameState { is_x_turn: true, winner: CellValues::Empty, processed_winner: false });

    draw_grid(&mut commands);
}

fn draw_grid(commands: &mut Commands) {

    const CELL_SIZE: f32 = 50.0;
    const CELL_MARGIN: f32 = 5.0;
    const TOP_LEFT_CELL: Vec2 = Vec2 {x: -55.0, y: -55.0};

    for row_index in 0..3 {

        for column_index in 0..3 {

            let cell_offset = Vec2 {
                x: (CELL_SIZE + CELL_MARGIN) * column_index as f32, 
                y: (CELL_SIZE + CELL_MARGIN) * row_index as f32
            };

            let cell_index = CellIndex {x: row_index as usize, y: column_index as usize};

            let cell_sprite = Sprite::from_color(Color::WHITE, Vec2::ONE * CELL_SIZE);

            let cell_screen_position = TOP_LEFT_CELL + cell_offset;
            let cell_transform = Transform::from_xyz(
                cell_screen_position.x, 
                cell_screen_position.y, 
                -1.0
            );

            commands
                .spawn((cell_index, cell_sprite, cell_transform, Pickable::default()))
                .observe(make_move);
        }
    }
}

fn make_move(
    event: On<Pointer<Press>>, 
    mut query: Query<(&mut Sprite, &CellIndex)>, 
    mut game_state: ResMut<GameState>, 
    mut grid_query: Query<&mut Grid>,
    asset_server: Res<AssetServer>
) {


    let Ok((mut sprite, cell)) = query.get_mut(event.entity) else {
        return;
    };

    let Ok(mut grid) = grid_query.single_mut() else { return; };

    if grid.values[cell.x][cell.y] != CellValues::Empty { return; }

    if game_state.is_x_turn {
        sprite.image = asset_server.load("X.png");
        grid.values[cell.x][cell.y] = CellValues::X;
    } else {
        sprite.image = asset_server.load("O.png");
        grid.values[cell.x][cell.y] = CellValues::Y;
    }

    if has_winner(&game_state, cell, &grid) {
        if game_state.is_x_turn {
            game_state.winner = CellValues::X;
        } else {
            game_state.winner = CellValues::Y;
        }
        return;
    }

    game_state.is_x_turn = !game_state.is_x_turn;
}

fn has_winner(game_state: &GameState, last_play_position: &CellIndex, grid: &Grid) -> bool {

    let expected_value: CellValues = if game_state.is_x_turn {
        CellValues::X
    } else {
        CellValues::Y
    };

    let mut vertical_counter = 0;
    for i in last_play_position.x..3 {
        if grid.values[i][last_play_position.y] == expected_value {
            vertical_counter += 1;
        }
    }

    for i in 0..last_play_position.x {
        if grid.values[i][last_play_position.y] == expected_value {
            vertical_counter += 1;
        }
    }

    if vertical_counter == 3 {
        return true;
    }

    let mut horizontal_counter = 0;
    for i in last_play_position.y..3 {
        if grid.values[last_play_position.x][i] == expected_value {
            horizontal_counter += 1;
        }
    }

    for i in 0..last_play_position.y {
        if grid.values[last_play_position.x][i] == expected_value {
            horizontal_counter += 1;
        }
    }

    if horizontal_counter == 3 {
        return true;
    }

    let northeast = [
        grid.values[0][0], grid.values[1][1], grid.values[2][2]
    ];
    
    if northeast.iter().all(|&v| v == expected_value) {
        return true;
    }

    let northwest = [
        grid.values[2][0], grid.values[1][1], grid.values[0][2]
    ];

    if northwest.iter().all(|&v| v == expected_value) {
        return true;
    }

    false
}

fn check_for_winner(mut game_state: ResMut<GameState>, query: Query<Entity, With<CellIndex>>, mut commands: Commands) {
    if game_state.winner == CellValues::Empty || game_state.processed_winner { return; }

    println!("has winner!");

    for obs in query.iter() {
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

    game_state.processed_winner = true;

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
    commands.spawn(bundle).observe(button_event_handler);
}

fn button_event_handler(
    _event: On<Activate>,
    to_despawn_query: Query<Entity, Or<(With<Grid>, With<CellIndex>, With<WinLabel>)>>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands
) {
    // remove grid and cells
    for ent in to_despawn_query.iter() {
        commands.entity(ent).despawn();
    }

    game_state.is_x_turn = true;
    game_state.processed_winner = false;
    game_state.winner = CellValues::Empty;

    commands.spawn(Grid { values: [[CellValues::Empty; 3]; 3] });

    draw_grid(&mut commands);
}


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(GamePlugin)
    .add_plugins(UiWidgetsPlugins)
    .run();
}
