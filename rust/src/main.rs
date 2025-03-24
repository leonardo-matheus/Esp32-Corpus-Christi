use bevy::input::gamepad::{GamepadAxis, GamepadAxisType, GamepadConnection, GamepadConnectionEvent};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::math::primitives::Circle; // Importação correta do Circle

const AREA_SIZE: f32 = 2.0;
const PAINT_THRESHOLD: f32 = 0.5;
const DEAD_ZONE: f32 = 0.1;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<GamepadState>()
        .init_resource::<PaintingGrid>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                gamepad_connections,
                update_cursor,
                paint_system,
                render_painted,
                debug_axes,
            ),
        )
        .run();
}

#[derive(Resource, Default)]
struct GamepadState {
    gamepad: Option<Gamepad>,
}

#[derive(Resource)]
struct PaintingGrid {
    pixels: Vec<bool>,
    size: usize,
}

impl Default for PaintingGrid {
    fn default() -> Self {
        let size = 100;
        Self {
            pixels: vec![false; size * size],
            size,
        }
    }
}

#[derive(Component)]
struct Cursor;
#[derive(Component)]
struct Painted;
#[derive(Component)]
struct TapeteOutline;
#[derive(Component)]
struct Brush {
    radius: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_scale(Vec3::new(1.0, 1.0, 1.0)),
        ..default()
    });

    // Tapete outline
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.3),
                custom_size: Some(Vec2::new(AREA_SIZE, AREA_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },
        TapeteOutline,
    ));

// Cursor com escala adequada
commands.spawn((
    MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(Circle::new(0.1))).into(),
        material: materials.add(ColorMaterial::from(Color::rgba(1.0, 0.5, 0.2, 0.5))),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            scale: Vec3::new(2.0, 2.0, 1.0), // Aumenta o tamanho
            ..default()
        },
        ..default()
    },
    Cursor,
    Brush { radius: 0.15 },
));
}

fn gamepad_connections(
    mut gamepad_state: ResMut<GamepadState>,
    mut events: EventReader<GamepadConnectionEvent>,
) {
    for event in events.read() {
        match &event.connection {
            GamepadConnection::Connected(_) => {
                gamepad_state.gamepad = Some(event.gamepad);
                println!("Gamepad conectado: {:?}", event.gamepad);
            }
            GamepadConnection::Disconnected => {
                gamepad_state.gamepad = None;
                println!("Gamepad desconectado: {:?}", event.gamepad);
            }
        }
    }
}

fn update_cursor(
    gamepad_state: Res<GamepadState>,
    axes: Res<Axis<GamepadAxis>>,
    mut query: Query<&mut Transform, With<Cursor>>,
) {
    if let Some(gamepad) = gamepad_state.gamepad {
        // Mude para RightStickX/Y
        let x_axis = GamepadAxisType::RightStickX;  // RX
        let y_axis = GamepadAxisType::RightStickY;  // RY

        if let (Some(x), Some(y)) = (
            axes.get(GamepadAxis::new(gamepad, x_axis)),
            axes.get(GamepadAxis::new(gamepad, y_axis)),
        ) {
            let x = apply_dead_zone(x) * (AREA_SIZE / 2.0);
            let y = apply_dead_zone(y) * (AREA_SIZE / 2.0);

            for mut transform in &mut query {
                transform.translation.x = x;
                transform.translation.y = y;
            }
        }
    }
}

fn paint_system(
    gamepad_state: Res<GamepadState>,
    axes: Res<Axis<GamepadAxis>>,
    mut painting_grid: ResMut<PaintingGrid>,
    cursor_query: Query<(&Transform, &Brush), With<Cursor>>,
) {
    if let Some(gamepad) = gamepad_state.gamepad {
        // Use Right Trigger (RZ)
        let trigger_axis = GamepadAxisType::RightZ;
        
        if let Some(trigger) = axes.get(GamepadAxis::new(gamepad, trigger_axis)) {
            if trigger > 0.3 { // Ajuste o threshold
                let cell_size = AREA_SIZE / painting_grid.size as f32;
                
                for (transform, brush) in cursor_query.iter() {
                    let center_x = transform.translation.x;
                    let center_y = transform.translation.y;
                    
                    let radius_pixels = (brush.radius / AREA_SIZE) * painting_grid.size as f32;
                    let min_x = ((center_x - brush.radius + AREA_SIZE/2.0) / AREA_SIZE * painting_grid.size as f32).floor() as i32;
                    let max_x = ((center_x + brush.radius + AREA_SIZE/2.0) / AREA_SIZE * painting_grid.size as f32).ceil() as i32;
                    let min_y = ((center_y - brush.radius + AREA_SIZE/2.0) / AREA_SIZE * painting_grid.size as f32).floor() as i32;
                    let max_y = ((center_y + brush.radius + AREA_SIZE/2.0) / AREA_SIZE * painting_grid.size as f32).ceil() as i32;

                    for y in min_y..=max_y {
                        for x in min_x..=max_x {
                            if x >= 0 && x < painting_grid.size as i32 && 
                               y >= 0 && y < painting_grid.size as i32 
                            {
                                let dx = (x as f32 + 0.5) * cell_size - (center_x + AREA_SIZE/2.0);
                                let dy = (y as f32 + 0.5) * cell_size - (center_y + AREA_SIZE/2.0);
                                
                                if dx*dx + dy*dy <= brush.radius * brush.radius {
                                    let index = y as usize * painting_grid.size + x as usize;
                                    painting_grid.pixels[index] = true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_painted(
    mut commands: Commands,
    painting_grid: Res<PaintingGrid>,
    query: Query<Entity, With<Painted>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    let cell_size = AREA_SIZE / painting_grid.size as f32;
    for y in 0..painting_grid.size {
        for x in 0..painting_grid.size {
            if painting_grid.pixels[y * painting_grid.size + x] {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(0.8, 0.4, 0.1),
                            custom_size: Some(Vec2::splat(cell_size)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(
                            x as f32 * cell_size - AREA_SIZE / 2.0,
                            y as f32 * cell_size - AREA_SIZE / 2.0,
                            0.0,
                        )),
                        ..default()
                    },
                    Painted,
                ));
            }
        }
    }
    println!("Pixels pintados: {}", painting_grid.pixels.iter().filter(|&&p| p).count());
}

fn debug_axes(gamepad_state: Res<GamepadState>, axes: Res<Axis<GamepadAxis>>) {
    if let Some(gamepad) = gamepad_state.gamepad {
        let x_axis = GamepadAxisType::RightStickX;
        let y_axis = GamepadAxisType::RightStickY;

        if let (Some(x), Some(y)) = (
            axes.get(GamepadAxis::new(gamepad, x_axis)),
            axes.get(GamepadAxis::new(gamepad, y_axis)),
        ) {
            println!(
                "Eixos recebidos - X: {:.2} (raw: {:.2}), Y: {:.2} (raw: {:.2})",
                apply_dead_zone(x) * 2.0,
                x,
                apply_dead_zone(y) * 2.0,
                y
            );
        }
    }
}

fn apply_dead_zone(value: f32) -> f32 {
    if value.abs() < DEAD_ZONE {
        0.0
    } else {
        value.signum() * (value.abs() - DEAD_ZONE) / (1.0 - DEAD_ZONE)
    }
}