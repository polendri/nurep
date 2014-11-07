/*!
  A tool for downloading planets.nu data and creating data files for use with nuvis.
*/

extern crate getopts;
extern crate planets_nu;
extern crate serialize;

use planets_nu::request;
use serialize::json;
use std::io;
use std::os;

mod state;

/// Describes the command-line parameters of this utility.
struct Arguments {
    pub program_name: String,
    pub game_id: i32,
    pub player_id: i32,
    pub output_path: String,
    pub username: Option<String>,
}

/// Prints usage information for the utility.
fn print_usage(program_name: &str, opts: &[getopts::OptGroup]) {
    let brief = format!(
        "Usage:\n    {} [options] <game_id> <player_id> <output_path>",
        program_name);
    println!("{}", getopts::usage(brief.as_slice(), opts));
}

/// Parses command-line arguments, returning a Arguments object on success.
fn parse_args(args: &Vec<String>) -> Option<Arguments> {
    let program_name = args[0].clone();

    let opts = [
        getopts::optopt("a", "auth", "authenticate using the provided username", "USERNAME"),
    ];

    let matches = match getopts::getopts(args.tail(), opts) {
        Ok(m) => m,
        Err(_) => {
            print_usage(program_name.as_slice(), opts);
            return None;
        },
    };

    if matches.free.len() < 3 {
        print_usage(program_name.as_slice(), opts);
        return None;
    }

    let game_id = {
        let game_id_opt: Option<i32> = from_str(matches.free[0].as_slice());
        match game_id_opt {
            Some(x) => x,
            None => {
                print_usage(program_name.as_slice(), opts);
                return None;
            },
        }
    };

    let player_id = {
        let player_id_opt: Option<i32> = from_str(matches.free[1].as_slice());
        match player_id_opt {
            Some(x) => x,
            None => {
                print_usage(program_name.as_slice(), opts);
                return None;
            },
        }
    };

    let output_path = matches.free[2].clone();
    let username = matches.opt_str("a");

    Some(Arguments {
        program_name:  program_name,
        game_id:       game_id,
        player_id:     player_id,
        output_path:   output_path,
        username:      username,
    })
}

fn main() {
    let args = match parse_args(&os::args()) {
        Some(a) => a,
        None => return,
    };

    let api_key = match args.username {
        Some(username) => {
            println!("Please enter the planets.nu password associated with the specified username.");
            print!("Password: ");
            // TODO: Find a way to not echo this back to the user :(
            let password = match io::stdin().read_line() {
                Ok(s) => s,
                Err(_) => return,
            };
            print!("\n\nAuthenticating with planets.nu...");
            print!("username: {}, password: {}", username, password);

            let login_result = match request::login(username, password) {
                Ok(r) => r,
                Err(e) => {
                    println!(" ...Failed.");
                    println!("");
                    println!("Error occurred during authentication: {}", e);
                    return;
                },
            };
            println!(" ...Done.");
            Some(login_result.api_key)
        },
        None => None,
    };

    print!("Downloading game data...");

    /*
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
    */
    println!(" ...Done.");

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

    /*
    let state = state::State { galaxy: galaxy };
    let output_json = json::encode(&state);
    let mut output_file = io::File::create(&Path::new(output_path));
    let _ = output_file.write_str(output_json.as_slice());
    */
}
