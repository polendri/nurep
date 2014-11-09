/*!
  Data structures used to represent the state of a visualization.
*/

use std::collections::TreeMap;

/// Represents a single planet in the cluster.
#[deriving(Decodable, Encodable)]
pub struct Planet {
    pub id: i32,
    pub position: (i32, i32),
}

/// Represents a connection between two planets.
#[deriving(Decodable, Encodable)]
pub struct Connection {
    pub id_a: i32,
    pub id_b: i32,
}

/// Represents a cluster full of planets.
#[deriving(Decodable, Encodable)]
pub struct Cluster {
    pub dimensions: (i32, i32),
    pub planets: Vec<Planet>,
    pub connections: Vec<Connection>,
}

/// Represents all info needed to visualize a whole game.
#[deriving(Decodable, Encodable)]
pub struct Game {
    pub num_turns: i32,
    pub cluster: Cluster,
    /// A map from planet ID to a map from turn ID to the owner of the planet on that turn.
    pub planet_to_owners: TreeMap<i32, TreeMap<i32, i32>>,
}
