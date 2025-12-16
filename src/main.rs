use bevy::{prelude::*};
use bevy::{ui_widgets::{Activate, UiWidgetsPlugins, Button}};


#[derive(Component, Clone, Copy, PartialEq, Debug)]
enum CellValues {
    X,
    Y,
    Empty
}

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
                    CellValues::Empty, 
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
    mut query: Query<(&mut Sprite, &CellValues, Entity)>, 
    mut game_state: ResMut<GameState>, 
    asset_server: Res<AssetServer>,
    mut commands: Commands
) {

    let (mut sprite, cell, entity) = query.get_mut(event.entity).unwrap();

    if *cell != CellValues::Empty { return; }

    let mut entity_comm = commands.get_entity(entity).unwrap();

    if game_state.is_x_turn {
        sprite.image = asset_server.load("X.png");
        entity_comm.insert(CellValues::X);
    } else {
        sprite.image = asset_server.load("O.png");
        entity_comm.insert(CellValues::Y);
    }

    game_state.is_x_turn = !game_state.is_x_turn;
}

fn get_winner(game_state: &GameState, cells: Query<&CellValues>) -> CellValues {

    let mut winning_sequences: Vec<[Entity; 3]> = Vec::with_capacity(8);

    // check horizontal
    winning_sequences.extend(game_state.board);

    // check vertical
    winning_sequences.extend(
        (0..3).map(|column| core::array::from_fn(|i| game_state.board[i][column]))
    );

    // check diagonals
    winning_sequences.push(core::array::from_fn(|i| game_state.board[i][i]));
    winning_sequences.push(core::array::from_fn(|i| game_state.board[i][2 - i]));

    for seq in winning_sequences {

        let mut seq_unwrap = seq.into_iter().map(|e| cells.get(e).unwrap());
        let ref_value = seq_unwrap.next().unwrap();
        if *ref_value != CellValues::Empty && seq_unwrap.all(|x| x == ref_value) { return *ref_value }

    }

    CellValues::Empty
}

fn check_for_winner(
    mut game_state: ResMut<GameState>, 
    cells_query: Query<&CellValues>, 
    cell_entities: Query<Entity, With<CellValues>>, 
    mut commands: Commands
) {

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

type DespawnQueryFilter = Or<(With<CellValues>, With<WinLabel>)>;

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
