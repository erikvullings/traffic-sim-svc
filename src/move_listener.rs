/// A location is defined as a WGS84-based standard representation of a location on earth
#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    #[serde(default = "default_location_altitude")]
    pub altitude: Option<f64>,
}

#[inline(always)]
fn default_location_altitude() -> Option<f64> {
    None
}

/// Straight – move in a direct line to all waypoints without taking into account the terrain; CrossCountry – move directly to all waypoints without taking into account the roads; OnlyRoads – stay on the roads to get to the closest point to the waypoints that is still on a road; RoadsAndCrossCountry – move to the waypoints by taking into account the roads; it is allowed to go off the road
#[derive(
    Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, serde::Deserialize, serde::Serialize,
)]
pub enum MoveType {
    Straight,
    CrossCountry,
    OnlyRoads,
    RoadsAndCrossCountry,
}

/// A move request is a specific request for moving or transporting a given simulation entity towards a given destination, possibly over a given set of waypoints or route. *Copyright (C) 2019-2020 XVR Simulation B.V., Delft, The Netherlands, Martijn Hendriks <hendriks @ xvrsim.com>. This file is licensed under the MIT license : https://github.com/DRIVER-EU/avro-schemas/blob/master/LICENSE*
#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct RequestMove {
    pub id: String,
    pub applicant: String,
    pub entities: Vec<String>,
    pub destination: String,
    pub waypoints: Vec<Location>,
    #[serde(rename = "moveType")]
    pub move_type: MoveType,
    #[serde(default = "default_requestmove_route")]
    pub route: Option<String>,
    #[serde(default = "default_requestmove_tags")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[inline(always)]
fn default_requestmove_route() -> Option<String> {
    None
}

#[inline(always)]
fn default_requestmove_tags() -> Option<::std::collections::HashMap<String, String>> {
    None
}
