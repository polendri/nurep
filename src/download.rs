/*!
  A tool for downloading planets.nu data and creating data files for use with nurep.
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
                Ok(ref s) => {
                    let mut s_trunc = s.clone();
                    s_trunc.truncate(s.len() - 1);
                    s_trunc
                },
                Err(_) => return,
            };
            print!("\n\nAuthenticating with planets.nu...");

            let login_result = match request::login(username.as_slice(), password.as_slice()) {
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

    let mut turn : i32 = 1;
    print!("Downloading game data... Turn {: >4d}", turn);

    let response = match request::load_turn(args.game_id, Some(1), api_key.clone(), Some(args.player_id), false) {
        Ok(x) => x,
        Err(e) => {
            println!("\nError: Request to planets.nu failed. (Reason: {})", e);
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
    let cluster = state::Cluster {
        dimensions: (response.game_settings.map_width, response.game_settings.map_height),
        planets: planets,
        connections: Vec::new()
    };
    turn += 1;

    loop {
        print!("\rDownloading game data... Turn {: >4d}", turn);
        io::stdio::flush();
        // TODO: no apikey.clone()
        let response = match request::load_turn(args.game_id, Some(turn), api_key.clone(), Some(args.player_id), false) {
            Ok(x) => x,
            Err(_) => break,
        };

        // TODO

        turn += 1;
    }

    println!("\rDownloading game data... ...Done. ");
    print!("Saving data to disk...");

    let state = state::State { cluster: cluster };
    let output_json = json::encode(&state);
    let mut output_file = io::File::create(&Path::new(args.output_path));
    let _ = output_file.write_str(output_json.as_slice());

    println!(" ...Done.");
}
