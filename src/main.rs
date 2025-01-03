#!/bin/rs

use mpris::{PlaybackStatus, Player, PlayerFinder};
use nannou_osc as osc;
use serde_derive::Deserialize;
use std::{fs::read_to_string, thread::sleep, time::Duration};
use toml::from_str;

#[derive(Deserialize)]
struct Data {
    config: Config,
}

#[derive(Deserialize)]
struct Config {
    send_address: Option<String>,
    send_port: Option<u16>,
    small_bubble: Option<bool>,
    sleep_time: Option<u64>,
    program: Option<String>,
}

impl Config {
    fn load(filename: &str) -> Self {
        let contents = read_to_string(filename).expect("Could not read config file");
        let data: Data = from_str(&contents).expect("Could not load config file");

        data.config
    }
}

fn find_player(program: Option<&str>) -> Option<Player> {
    let player_finder = PlayerFinder::new().expect("Could not connect to D-Bus");

    match program {
        Some(program) => player_finder.find_by_name(program).ok(),
        None => player_finder.find_active().ok(),
    }
}

fn format_music(small_bubble: bool, player: Player) -> String {
    let metadata = player.get_metadata().expect("Could not find metadata");

    let playback_status = player
        .get_playback_status()
        .expect("Could not find playback");

    let playback = match playback_status {
        PlaybackStatus::Playing => "",
        PlaybackStatus::Paused => "\u{23f8} ",
        PlaybackStatus::Stopped => return String::new(),
    };

    let position = player.get_position().expect("No position").as_secs();
    let length = metadata.length().expect("No length").as_secs();

    let artist = match metadata.artists() {
        None => "".to_string(),
        Some(artists) => {
            // SEND ME A PULL REQUEST NOW
            if let Some(artist) = artists.get(0) {
                if artist.is_empty() {
                    String::new()
                } else {
                    format!("{} - ", artists.join(", "))
                }
            } else {
                String::new()
            }
        }
    };

    let title = match metadata.title() {
        None => "Untitled",
        Some(title) => title,
    };

    let bubble = if small_bubble { "\u{0003}\u{00f1}" } else { "" };

    format!(
        "{}{}\n{}{:0>2}:{:0>2}/{:0>2}:{:0>2}{}",
        artist,
        title,
        playback,
        position / 60,
        position % 60,
        length / 60,
        length % 60,
        bubble
    )
}

struct OscModel {
    sender: osc::Sender<osc::Connected>,
}

impl OscModel {
    fn connect(send_address: &str, send_port: &str) -> Self {
        let target_addr = format!("{}:{}", send_address, send_port);

        let sender = osc::sender()
            .expect("Could not bind to default osc socket")
            .connect(target_addr)
            .expect("Could not connect to osc socket at address");

        Self { sender }
    }
}

fn main() {
    let config = Config::load("config.toml");

    let sleep_time = match config.sleep_time {
        None => Duration::from_millis(1500),
        Some(sleep_time) => Duration::from_millis(sleep_time),
    };

    let small_bubble = match config.small_bubble {
        None => true,
        Some(small_bubble) => small_bubble,
    };

    let send_address = match config.send_address {
        None => "127.0.0.1".to_string(),
        Some(send_address) => send_address,
    };

    let send_port = match config.send_port {
        None => "9000".to_string(),
        Some(send_port) => send_port.to_string(),
    };

    loop {
        let Some(player) = find_player(config.program.as_deref()) else {
            continue;
        };

        let message = format_music(small_bubble, player);

        let arguments = vec![
            osc::Type::String(message),
            osc::Type::Bool(true),
            osc::Type::Bool(false),
        ];

        let packet = ("/chatbox/input", arguments);

        OscModel::connect(&send_address, &send_port)
            .sender
            .send(packet)
            .ok();

        sleep(sleep_time);
    }
}
