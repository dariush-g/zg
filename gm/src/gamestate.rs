use std::net::TcpStream;

use bevy::prelude::*;

#[derive(Default, States, Debug, Eq, PartialEq, Clone, Hash)]
pub enum AppState {
    StartScreen,
    Loading,
    #[default]
    Playing,
    Paused,
}

#[derive(Event)]
pub struct StartGameEvent;

pub struct GameStatePlugin;

impl bevy::app::Plugin for GameStatePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_state::<AppState>()
            .add_event::<StartGameEvent>()
            .add_systems(OnEnter(AppState::StartScreen), setup_startscreen)
            .add_systems(
                Update,
                start_game_event_handler.run_if(in_state(AppState::StartScreen)),
            )
            .add_systems(Update, button_input.run_if(in_state(AppState::StartScreen))) //.run_if(in_state(AppState::StartScreen)))
            .add_systems(OnExit(AppState::StartScreen), cleanup::<Node>)
            .add_systems(OnEnter(AppState::Loading), cleanup::<Node>)
            .add_systems(OnExit(AppState::Loading), cleanup::<Node>)
            .add_systems(OnEnter(AppState::Loading), loading_screen)
            .add_systems(
                Update,
                loading_screen_system.run_if(in_state(AppState::Loading)),
            );
    }
}

fn setup_startscreen(
    mut commands: Commands,
    //asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2d);
    let container = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        position_type: PositionType::Absolute,
        ..default()
    };

    let square_color = Color::srgb(0.65, 0.65, 0.65);

    let square = Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(25.),
        height: Val::Percent(10.),
        border: UiRect::all(Val::Px(2.)),
        top: Val::Percent(53.),
        justify_content: JustifyContent::Center,
        align_content: AlignContent::Center,
        justify_items: JustifyItems::Center,
        justify_self: JustifySelf::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let child1 = commands
        .spawn((square, BackgroundColor(square_color), Button))
        .with_child((
            Text("PLAY".to_owned()),
            TextFont {
                font_size: 30.,
                ..Default::default()
            },
        ))
        .id();
    let square = Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(25.),
        height: Val::Percent(10.),
        border: UiRect::all(Val::Px(2.)),
        top: Val::Percent(65.),
        justify_content: JustifyContent::Center,
        align_content: AlignContent::Center,
        justify_items: JustifyItems::Center,
        justify_self: JustifySelf::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let child2 = commands
        .spawn((square, BackgroundColor(square_color), Button))
        .with_child((
            Text("SETTINGS".to_owned()),
            TextFont {
                font_size: 30.,
                ..Default::default()
            },
        ))
        .id();

    let square = Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(25.),
        height: Val::Percent(10.),
        border: UiRect::all(Val::Px(2.)),
        top: Val::Percent(77.),
        justify_content: JustifyContent::Center,
        align_content: AlignContent::Center,
        justify_items: JustifyItems::Center,
        justify_self: JustifySelf::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let child3 = commands
        .spawn((square, BackgroundColor(square_color), Button))
        .id();

    let parent = commands.spawn(container).id();

    commands
        .entity(parent)
        .add_children(&[child1, child2, child3]);
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[allow(warnings)]
fn button_input(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut writer: EventWriter<StartGameEvent>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                if text.0 == "PLAY" {
                    writer.write(StartGameEvent);
                }
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

// Transition state when event is received
fn start_game_event_handler(
    mut events: EventReader<StartGameEvent>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for _ in events.read() {
        next_state.set(AppState::Loading);
    }
}

#[derive(Component, Copy, Clone, Debug)]
struct LoadingScreenTimer(f32);

fn loading_screen(mut commands: Commands) {
    commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::End,
        align_items: AlignItems::End,
        position_type: PositionType::Absolute,
        ..default()
    });
    commands.spawn((
        Node {
            width: Val::Percent(10.),
            height: Val::Percent(10.),
            top: Val::Percent(95.),
            ..Default::default()
        },
        Text::new("Loading"),
        LoadingScreenTimer(0.0),
    ));
}

fn loading_screen_system(
    mut text_q: Query<(&mut Node, &mut Text, &mut LoadingScreenTimer)>,
    time: Res<Time>,
) {
    // println!("{}", time.delta_secs());
    if let Ok((_, mut text, mut timer)) = text_q.single_mut() {
        if timer.0 >= 4. {
            timer.0 = 0.;
            text.0 = String::from("Loading");
        }
        timer.0 += time.delta_secs();
        if timer.0 < 1.0 && timer.0 > 0.8 {
            text.0 += ".";
            timer.0 = 1.0;
        }
        if timer.0 < 2.0 && timer.0 > 1.8 {
            text.0 += ".";
            timer.0 = 2.;
        }
        if timer.0 < 3.0 && timer.0 > 2.8 {
            text.0 += ".";
            timer.0 = 3.;
        }
    }
}

fn cleanup<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    q.into_iter().for_each(|e| {
        commands.entity(e).despawn();
    });
}
