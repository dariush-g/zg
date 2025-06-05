use std::{
    io::{Read, Write},
    net::TcpStream,
};

use bevy::{ecs::event, prelude::*};
use serde::{Deserialize, Serialize};

use crate::gamestate::AppState;

#[derive(Event, Serialize, Deserialize)]
pub struct ConnectedToServerEvent;

pub struct MPlayerPlugin;

impl Plugin for MPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ConnectedToServerEvent>()
            //.add_systems(OnEnter(AppState::Loading), send_ping)
            .add_systems(OnEnter(AppState::Loading), start_playing)
            .add_systems(Update, check_connected.run_if(in_state(AppState::Loading)));
    }
}

fn check_connected(
    mut events: EventReader<ConnectedToServerEvent>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for _ in events.read() {
        next_state.set(AppState::Playing);
    }
}

fn start_playing(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Playing);
}

fn send_ping() {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1") {
        if let Ok(serialized_message) = bincode::serialize(&String::from("ping")) {
            if stream.write_all(&serialized_message).is_ok() {
                let mut buffer = [0; 512];
                if let Ok(bytes) = stream.read(&mut buffer) {
                    if let Ok(message) = bincode::deserialize::<String>(&buffer[0..bytes]) {
                        println!("{message}");
                    }
                }
            }
        }
    }
}
