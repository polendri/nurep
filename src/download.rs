/*!
  A tool for downloading planets.nu data and creating data files for use with nurep.
*/

extern crate getopts;
extern crate planets_nu;
extern crate serialize;

use planets_nu::request;
use serialize::json;
use std::cmp;
use std::i32;
use std::io;
use std::num;
use std::os;
use std::collections::TreeMap;

mod state;

/// Describes the command-line parameters of this utility.
struct Arguments {
    pub program_name: String,
    pub game_id: i32,
    pub output_path: String,
}

/// Prints usage information for the utility.
fn print_usage(program_name: &str) {
    println!("Usage:");
    println!("    {} [options] <game_id> <player_id> <output_path>", program_name);
}

/// Parses command-line arguments, returning a Arguments object on success.
fn parse_args(args: &Vec<String>) -> Option<Arguments> {
    let program_name = args[0].clone();

    if args.len() < 3 {
        print_usage(program_name.as_slice());
        return None;
    }

    let game_id = {
        let game_id_opt: Option<i32> = from_str(args[1].as_slice());
        match game_id_opt {
            Some(x) => x,
            None => {
                print_usage(program_name.as_slice());
                return None;
            },
        }
    };

    let output_path = args[2].clone();

    Some(Arguments {
        program_name:  program_name,
        game_id:       game_id,
        output_path:   output_path,
    })
}

/// Builds a Cluster object from a load turn response.
fn build_cluster(response: &request::LoadTurnResult) -> state::Cluster {
    let mut planets: Vec<state::Planet> = Vec::new();
    let (mut min_x, mut max_x) = (i32::MAX, i32::MIN);
    let (mut min_y, mut max_y) = (i32::MAX, i32::MIN);

    // Build the planets list and determine min/max coordinate values
    for p in response.planets.iter() {
        let (x, y) = p.position;
        min_x = cmp::min(x, min_x);
        max_x = cmp::max(x, max_x);
        min_y = cmp::min(y, min_y);
        max_y = cmp::max(y, max_y);

        planets.push(state::Planet {
            id: p.id,
            position: p.position,
        });
    }

    // Adjust planet coordinates so they're zero-based
    let mut i = 0u;
    while i < planets.len() {
        let (x, y) = planets[i].position;
        planets[i].position = (x - min_x, y - min_y);
        i += 1;
    }

    // Calculate connections by brute force
    let mut connections: Vec<state::Connection> = Vec::new();
    let mut i1 = 0u;

    while i1 < planets.len() - 1 {
        let mut i2 = i1 + 1;
        while i2 < planets.len() {
            let p1 = planets[i1];
            let p2 = planets[i2];
            let (x1, y1) = p1.position;
            let (x2, y2) = p2.position;

            if ((num::pow(x2 - x1, 2) + num::pow(y2 - y1, 2)) as f64).sqrt() <= 81.0 {
                connections.push(state::Connection {
                    id_a: p1.id,
                    id_b: p2.id,
                });
            }

            i2 += 1;
        }

        i1 += 1;
    }

    state::Cluster {
        dimensions: (max_x - min_x, max_y - min_y),
        planets: planets,
        connections: connections,
    }
}

/// Adds the owners of planets for the given turn.
fn add_owners(
        planet_to_owners: &mut TreeMap<i32, TreeMap<i32, i32>>,
        response: &request::LoadTurnResult,
        player_id: i32,
        turn: i32) {
    for p in response.planets.iter() {
        let turn_to_owner =
            if planet_to_owners.contains_key(&p.id) {
                planet_to_owners.get_mut(&p.id).unwrap()
            } else {
                planet_to_owners.insert(p.id, TreeMap::new());
                planet_to_owners.get_mut(&p.id).unwrap()
            };

        if p.owner_id == player_id {
            turn_to_owner.insert(turn, player_id);
        }
        else if !turn_to_owner.contains_key(&turn) {
            turn_to_owner.insert(turn, 0);
        }
    }
}

fn main() {
    let args = match parse_args(&os::args()) {
        Some(a) => a,
        None => return,
    };

    let mut player_id: i32 = 1;
    let mut turn: i32 = 1;
    let mut max_turn: i32 = i32::MIN;
    print!("Downloading game data for player {: >2d}... Turn {: >4d}", player_id, turn);
    io::stdio::flush();

    let response = match request::load_turn(args.game_id, Some(1), None, Some(player_id), false) {
        Ok(x) => x,
        Err(e) => {
            println!("\nError: Request to planets.nu failed. (Reason: {})", e);
            return;
        },
    };
    let cluster = build_cluster(&response);
    let mut planet_to_owners: TreeMap<i32, TreeMap<i32, i32>> = TreeMap::new();
    add_owners(&mut planet_to_owners, &response, player_id, turn);
    turn += 1;

    loop {
        let mut load_turn_success = false;

        loop {
            print!("\rDownloading game data for player {: >2d}... Turn {: >4d}", player_id, turn);
            io::stdio::flush();
            let response = match request::load_turn(args.game_id, Some(turn), None, Some(player_id), false) {
                Ok(x) => {
                    load_turn_success = true;
                    x
                },
                Err(_) => break,
            };

            add_owners(&mut planet_to_owners, &response, player_id, turn);
            max_turn = cmp::max(turn, max_turn);
            turn += 1;
        }

        println!("\rDownloading game data for player {: >2d}... ...Done. ", player_id);
        player_id += 1;
        turn = 1;
        if !load_turn_success {
            break;
        }
    }

    print!("Saving data to disk...");

    let game = state::Game {
        num_turns: max_turn - 2,
        cluster: cluster,
        planet_to_owners: planet_to_owners
    };
    let output_json = json::encode(&game);
    let mut output_file = io::File::create(&Path::new(args.output_path));
    let _ = output_file.write_str(output_json.as_slice());

    println!(" ...Done.");
}
