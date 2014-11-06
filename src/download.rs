extern crate planets_nu;
extern crate serialize;

use planets_nu::request;
use serialize::json;
use std::io;
use std::os;

mod state;

fn print_usage() {
    println!("Usage: nuvis-download <game_id> <player_id> <output_path>");
}

fn main() {
    let args: Vec<String> = os::args();
    if args.len() != 4 {
        print_usage();
        return;
    }

    let game_id = {
        let game_id_opt : Option<i32> = from_str(args[1].as_slice());
        match game_id_opt {
            Some(x) => x,
            None => {
                print_usage();
                return;
            },
        }
    };
    let player_id = {
        let player_id_opt : Option<i32> = from_str(args[2].as_slice());
        match player_id_opt {
            Some(x) => x,
            None => {
                print_usage();
                return;
            },
        }
    };
    let output_path = args[3].as_slice();

    print!("Downloading game data...");

    let response = match request::load_turn(game_id, Some(1), None, Some(player_id), false) {
        Ok(x) => x,
        Err(e) => {
            println!("Error: Request to planets.nu failed. (Reason: {})", e);
            return;
        },
    };
    let mut planet_id_counter: i32 = 1;
    let mut planets: Vec<state::Planet> = Vec::new();
    for p in response.planets.iter() {
        planets.push(state::Planet {
            id: state::PlanetId(planet_id_counter),
            position: p.position,
        });
        planet_id_counter += 1;
    }
    let galaxy = state::Galaxy {
        dimensions: (response.game_settings.map_width, response.game_settings.map_height),
        planets: planets,
        connections: Vec::new()
    };

    /*
    let mut turn : i32 = 2;
    loop {
        let response = match request::load_turn(game_id, Some(turn), None, Some(player_id), false) {
            Ok(x) => x,
            Err(e) => break,
        };

        f

        turn += 1;
    }
    */

    let state = state::State { galaxy: galaxy };
    let output_json = json::encode(&state);
    let mut output_file = io::File::create(&Path::new(output_path));
    let _ = output_file.write_str(output_json.as_slice());
}
