/*!
  Data structures used to represent the state of a visualization.
*/

/// ID unique to a single planet.
#[deriving(Decodable, Encodable)]
pub struct PlanetId(pub i32);

/// Represents a single planet in the cluster.
#[deriving(Decodable, Encodable)]
pub struct Planet {
    pub id: PlanetId,
    pub position: (i32, i32),
}

/// Represents a connection between two planets.
#[deriving(Decodable, Encodable)]
pub struct Connection {
    pub id_a: PlanetId,
    pub id_b: PlanetId,
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
pub struct State {
    pub cluster: Cluster,
}
