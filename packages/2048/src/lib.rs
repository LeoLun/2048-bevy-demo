// mod utils;
#![allow(non_snake_case)]
#![allow(unused_mut)]
mod config;
use config::*;

mod gameRule;
use gameRule::*;

// use rand::Rng;
use wasm_bindgen::prelude::wasm_bindgen;

use bevy::asset::HandleId;
use bevy::prelude::*;

// use bevy::log;
use bevy::window::PresentMode;

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

#[wasm_bindgen]
pub fn run_bevy_app() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Rust Experiments".to_string(),
                // resolution: (900., 100.).into(),
                prevent_default_event_handling: false,
                // cursor_visible: true,
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                canvas: Some("#bevy".to_string()),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_state::<VictoryOrDefeat>()
        .add_systems(Startup, setup)
        .add_plugins((
            defeat::DefeatPlugin,
            game::GamePlugin,
            victory::VictoryPlugin,
        ))
        .run();
}

fn setup(mut commands: Commands) {
    // log::info!("setup");
    commands.spawn(Camera2dBundle::default());
}

mod game {
    // use bevy::log;
    use bevy::prelude::*;
    use bevy::sprite::MaterialMesh2dBundle;
    use bevy::text::Text2dBounds;

    use super::{
        cell_color, check_result, despawn_screen, init_cell_value_save, move_value, CellValue,
        CellValueSave, HandleId, MoveDirection, VictoryOrDefeat, CELL_SIDE_NUM, CELL_SPACE,
        COLOR_BACKGROUND, COLOR_BROWN, COLOR_WHITE, WINDOW_HEIGHT, WINDOW_WIDTH,
    };

    pub struct GamePlugin;

    impl Plugin for GamePlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(VictoryOrDefeat::NONE), game_setup)
                .add_systems(Update, game.run_if(in_state(VictoryOrDefeat::NONE)))
                .add_systems(
                    OnExit(VictoryOrDefeat::NONE),
                    despawn_screen::<OnGameScreen>,
                );
        }
    }

    #[derive(Component)]
    struct OnGameScreen;

    fn game_setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        // log::info!("game_setup");
        // 初始化存储数组
        let mut cell_value_save_temp: Vec<Vec<u32>> = init_cell_value_save();
        let mut cell_background_save: Vec<HandleId> = Vec::new();
        // 计算左上方格偏移
        let side_length: f32 =
            (WINDOW_HEIGHT - CELL_SPACE * (CELL_SIDE_NUM as f32 + 1.0)) / CELL_SIDE_NUM as f32;
        let mut x_offset = -(side_length + CELL_SPACE) * (CELL_SIDE_NUM as f32 / 2.0 - 0.5);
        let mut y_offset = (side_length + CELL_SPACE) * (CELL_SIDE_NUM as f32 / 2.0 - 0.5);
        x_offset = 2.0 * x_offset - (-1.0) * (WINDOW_WIDTH / 2.0 - CELL_SPACE) - side_length / 2.0;

        commands
            .spawn((
                SpatialBundle {
                    transform: Transform::from_xyz(0., 0., 0.),
                    ..default()
                },
                OnGameScreen,
            ))
            .with_children(|parent| {
                // log::info!("game_setup2");

                // // 初始化文字信息
                let text_style = TextStyle {
                    font_size: side_length / 2.0,
                    color: COLOR_BROWN,
                    ..Default::default()
                };
                let box_size = Vec2::new(side_length, side_length);

                parent.spawn(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::Box::new(WINDOW_HEIGHT, WINDOW_HEIGHT, 0.0).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(COLOR_BACKGROUND)),
                    transform: Transform::from_xyz((WINDOW_WIDTH - WINDOW_HEIGHT) / 2.0, 0.0, 0.0),
                    ..default()
                });

                for i in 0..CELL_SIDE_NUM {
                    for j in 0..CELL_SIDE_NUM {
                        // 格中显示内容
                        let mut text = "";
                        if cell_value_save_temp[i as usize][j as usize] == 2 {
                            text = "2";
                        }

                        let material_color = materials.add(ColorMaterial::from(cell_color(
                            cell_value_save_temp[i as usize][j as usize],
                        )));
                        cell_background_save.push(material_color.id());
                        // 绑定格，根据数字来确定格的颜色
                        parent.spawn(MaterialMesh2dBundle {
                            mesh: meshes
                                .add(shape::Box::new(side_length, side_length, 0.0).into())
                                .into(),
                            material: material_color,
                            transform: Transform::from_xyz(
                                x_offset + (j as f32) * (side_length + CELL_SPACE),
                                y_offset - (i as f32) * (side_length + CELL_SPACE),
                                0.0,
                            ),
                            ..default()
                        });

                        // 绑定数字
                        parent.spawn((
                            Text2dBundle {
                                text: Text::from_section(text, text_style.clone())
                                    .with_alignment(TextAlignment::Center),
                                text_2d_bounds: Text2dBounds {
                                    // Wrap text in the rectangle
                                    size: box_size,
                                },
                                transform: Transform::from_xyz(
                                    x_offset + (j as f32) * (side_length + CELL_SPACE),
                                    y_offset - (i as f32) * (side_length + CELL_SPACE),
                                    1.0,
                                ),
                                ..default()
                            },
                            CellValue,
                        ));
                    }
                }

                parent.spawn(Text2dBundle {
                    text: Text::from_sections([
                        TextSection::new("SCORE\n", text_style.clone()),
                        TextSection::new("0", text_style.clone()),
                    ]),
                    text_2d_bounds: Text2dBounds {
                        // Wrap text in the rectangle
                        size: box_size,
                    },
                    transform: Transform::from_xyz(-WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0 - 60.0, 0.0),
                    ..default()
                });
            });

        // 将存储数组设为资源
        commands.insert_resource(CellValueSave {
            value_save: cell_value_save_temp.clone(),
            cell_background: cell_background_save,
            score: 0,
        });
    }

    fn game(
        keyboard_input: Res<Input<KeyCode>>,
        _asset_server: Res<AssetServer>,
        mut cell_value_save: ResMut<CellValueSave>,
        mut text_query: Query<&mut Text, With<CellValue>>,
        mut score_query: Query<&mut Text, Without<CellValue>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut next_state: ResMut<NextState<VictoryOrDefeat>>,
    ) {
        let mut moved = MoveDirection::NONE;
        if keyboard_input.just_pressed(KeyCode::Up) {
            moved = MoveDirection::UP;
        }
        if keyboard_input.just_pressed(KeyCode::Down) {
            moved = MoveDirection::DOWN;
        }
        if keyboard_input.just_pressed(KeyCode::Right) {
            moved = MoveDirection::RIGHT;
        }
        if keyboard_input.just_pressed(KeyCode::Left) {
            moved = MoveDirection::LEFT;
        }

        match moved {
            MoveDirection::NONE => return,
            _ => {
                let mut i = 0;
                move_value(moved, &mut cell_value_save);

                score_query.single_mut().sections[1].value = cell_value_save.score.to_string();

                let side_length: f32 = (WINDOW_HEIGHT - CELL_SPACE * (CELL_SIDE_NUM as f32 + 1.0))
                    / CELL_SIDE_NUM as f32;
                // let font = asset_server.load("fonts/FiraSans-Bold.ttf");
                let mut text_style = TextStyle {
                    font_size: side_length / 2.0,
                    color: COLOR_BROWN,
                    ..Default::default()
                };

                for mut text in text_query.iter_mut() {
                    let cell_value_temp = cell_value_save.value_save[i / 4][i % 4];

                    if cell_value_temp > 4 {
                        text_style.color = COLOR_WHITE;
                    } else {
                        text_style.color = COLOR_BROWN;
                    }

                    if cell_value_temp != 0 {
                        text.sections[0].style = text_style.clone();
                        text.sections[0].value =
                            cell_value_save.value_save[i / 4][i % 4].to_string();
                    } else {
                        text.sections[0].value = "".to_string();
                    }
                    materials.set_untracked(
                        cell_value_save.cell_background[i],
                        ColorMaterial::from(cell_color(cell_value_save.value_save[i / 4][i % 4])),
                    );
                    i += 1;
                }

                let result = check_result(&mut cell_value_save);
                match result {
                    VictoryOrDefeat::VICTORY => {
                        // log::info!("VICTORY");
                        next_state.set(VictoryOrDefeat::VICTORY);
                    }
                    VictoryOrDefeat::DEFEAT => {
                        // log::info!("DEFEAT");
                        next_state.set(VictoryOrDefeat::DEFEAT);
                    }
                    VictoryOrDefeat::NONE => {},
                    // VictoryOrDefeat::NONE => log::info!("NONE"),
                }
            }
        }
    }
}

mod defeat {
    // use bevy::log;
    use bevy::prelude::*;
    use super::{despawn_screen, CellValueSave, VictoryOrDefeat, COLOR_BROWN, WINDOW_HEIGHT};

    pub struct DefeatPlugin;

    impl Plugin for DefeatPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(VictoryOrDefeat::DEFEAT), defeat_setup)
                .add_systems(
                    Update,
                    button_system.run_if(in_state(VictoryOrDefeat::DEFEAT)),
                )
                .add_systems(
                    OnExit(VictoryOrDefeat::DEFEAT),
                    despawn_screen::<OnDefeatScreen>,
                );
        }
    }

    #[derive(Component)]
    struct OnDefeatScreen;

    fn button_system(
        mut interaction_query: Query<(
            &Interaction,
            &Children,
        ), (Changed<Interaction>, With<Button>)>,
        mut text_query: Query<&mut Text>,
        mut next_state: ResMut<NextState<VictoryOrDefeat>>,
    ) {
        for (interaction, children) in &mut interaction_query {
            let mut text = text_query.get_mut(children[0]).unwrap();
            match *interaction {
                Interaction::Pressed => {
                    // log::info!("Pressed");
                    next_state.set(VictoryOrDefeat::NONE);
                }
                Interaction::Hovered => {
                    text.sections[0].style.color = Color::RED
                }
                Interaction::None => {
                    text.sections[0].style.color = COLOR_BROWN
                }
            }
        }
    }

    fn defeat_setup(mut commands: Commands, mut cell_value_save: ResMut<CellValueSave>) {
        // log::info!("defeat_function");
        // // let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        let text_style = TextStyle {
            font_size: WINDOW_HEIGHT / 5.0,
            color: COLOR_BROWN,
            ..Default::default()
        };
        let mut text = String::from("YOU  LOST\nSCORE: ");
        text.push_str(&cell_value_save.score.to_string());
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                OnDefeatScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn((NodeBundle {
                        style: Style {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    },))
                    .with_children(|parent| {
                        parent.spawn(
                            TextBundle::from_section(text, text_style.clone())
                                .with_text_alignment(TextAlignment::Center),
                        );
                    });

                parent
                    .spawn((NodeBundle {
                        style: Style {
                            position_type: PositionType::Relative,
                            top: Val::Px(30.),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    },))
                    .with_children(|parent| {
                        parent
                            .spawn(ButtonBundle {
                                style: Style {
                                    width: Val::Px(150.0),
                                    height: Val::Px(65.0),
                                    // border: UiRect::all(Val::Px(1.0)),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                border_color: BorderColor(Color::BLACK),
                                background_color: Color::rgb(0.9, 0.9, 0.9).into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    "REPLAY",
                                    TextStyle {
                                        font_size: WINDOW_HEIGHT / 8.0,
                                        color: COLOR_BROWN,
                                        ..default()
                                    },
                                ));
                            });
                    });
            });
    }
}

mod victory {
    use bevy::log;
    use bevy::prelude::*;

    use super::{despawn_screen, CellValueSave, VictoryOrDefeat, COLOR_BROWN, WINDOW_HEIGHT};

    pub struct VictoryPlugin;

    impl Plugin for VictoryPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(VictoryOrDefeat::VICTORY), victory_setup)
                .add_systems(
                    Update,
                    button_system.run_if(in_state(VictoryOrDefeat::VICTORY)),
                )
                .add_systems(
                    OnExit(VictoryOrDefeat::VICTORY),
                    despawn_screen::<OnVictoryScreen>,
                );
        }
    }

    #[derive(Component)]
    struct OnVictoryScreen;

    fn button_system(
        mut interaction_query: Query<(
            &Interaction,
            &Children,
        ), (Changed<Interaction>, With<Button>)>,
        mut text_query: Query<&mut Text>,
        mut next_state: ResMut<NextState<VictoryOrDefeat>>,
    ) {
        for (interaction, children) in &mut interaction_query {
            let mut text = text_query.get_mut(children[0]).unwrap();
            match *interaction {
                Interaction::Pressed => {
                    log::info!("Pressed");
                    next_state.set(VictoryOrDefeat::NONE);
                }
                Interaction::Hovered => {
                    text.sections[0].style.color = Color::RED
                }
                Interaction::None => {
                    text.sections[0].style.color = COLOR_BROWN
                }
            }
        }
    }

    fn victory_setup(mut commands: Commands, mut cell_value_save: ResMut<CellValueSave>) {
        log::info!("victory_function");
        let text_style = TextStyle {
            font_size: WINDOW_HEIGHT / 5.0,
            color: COLOR_BROWN,
            ..Default::default()
        };

        let mut text = String::from("WINNER\nSCORE: ");
        text.push_str(&cell_value_save.score.to_string());
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                OnVictoryScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn((NodeBundle {
                        style: Style {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    },))
                    .with_children(|parent| {
                        parent.spawn(
                            TextBundle::from_section(text, text_style.clone())
                                .with_text_alignment(TextAlignment::Center),
                        );
                    });

                parent
                    .spawn((NodeBundle {
                        style: Style {
                            position_type: PositionType::Relative,
                            top: Val::Px(30.),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    },))
                    .with_children(|parent| {
                        parent
                            .spawn(ButtonBundle {
                                style: Style {
                                    width: Val::Px(150.0),
                                    height: Val::Px(65.0),
                                    border: UiRect::all(Val::Px(1.0)),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                border_color: BorderColor(Color::BLACK),
                                background_color: Color::rgb(0.9, 0.9, 0.9).into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    "REPLAY",
                                    TextStyle {
                                        font_size: 40.0,
                                        color: Color::BLACK,
                                        ..default()
                                    },
                                ));
                            });
                    });
            });
    }
}

fn cell_color(cell_value: u32) -> bevy::render::color::Color {
    match cell_value {
        2 => COLOR_CELL_2.clone(),
        4 => COLOR_CELL_4.clone(),
        8 => COLOR_CELL_8.clone(),
        16 => COLOR_CELL_16.clone(),
        32 => COLOR_CELL_32.clone(),
        64 => COLOR_CELL_64.clone(),
        128 => COLOR_CELL_128.clone(),
        256 => COLOR_CELL_256.clone(),
        512 => COLOR_CELL_512.clone(),
        1024 => COLOR_CELL_1024.clone(),
        2048 => COLOR_CELL_2048.clone(),
        _ => COLOR_CELL_NULL.clone(),
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
