use bevy::{prelude::*, window::PrimaryWindow};

fn main() {
    App::new()
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_system(snake_movement)
        .add_systems((size_scaling, position_translation).in_base_set(CoreSet::PostUpdate))
        .add_plugins(DefaultPlugins)
        .run();
}

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGTH: u32 = 10;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    heigth: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            heigth: x,
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct SnakeHead;

fn spawn_snake(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(SnakeHead)
        .insert(Position { x: 3, y: 3 })
        .insert(Size::square(0.8));
}

fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_positions: Query<(&mut Position, With<SnakeHead>)>,
) {
    for (mut pos, _) in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            pos.x -= 1;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            pos.x += 1;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            pos.y -= 1;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            pos.y += 1;
        }
    }
}

fn size_scaling(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Size, &mut Transform)>,
) {
    let window = window_query.get_single().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.heigth / ARENA_HEIGTH as f32 * window.height() as f32,
            1.0,
        );
    }
}

fn position_translation(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Position, &mut Transform)>,
) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let title_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.0) + (title_size / 2.0)
    }

    let window = window_query.get_single().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGTH as f32),
            0.0,
        );
    }
}
