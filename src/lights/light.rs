
use std::fmt::Display;

// Struct, the data or model
#[derive(Debug)] // derive the Debug trait, so we can format {:?}
pub struct TrafficLight {
    color: TrafficLightColor,
}
// Implementation, the behavior
impl TrafficLight {
    // `new` is just a convention
    // Self is like `this`
    pub fn new() -> Self {
        Self {
            // color: "red".to_owned(),
            color: TrafficLightColor::Red,
        }
    }
    // every function in an impl defaults to private and is just an everyday function
    // pub fn get_state(&self) -> &TrafficLightColor {
    //     &self.color
    // }
    pub fn turn_green(&mut self) {
        // self.color = "green".to_owned()
        self.color = TrafficLightColor::Green
    }

    pub fn turn_yellow(&mut self) {
        self.color = TrafficLightColor::Yellow
    }
}
// we can also implement a trait, or shared/reusable behavior for our Struct
// this is so the default formatter works and we can
// println!("{}", a_traffic_light)
impl Display for TrafficLight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Traffic light is {}", self.color)
    }
}

#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}
impl Display for TrafficLightColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_string = match self {
            TrafficLightColor::Green => "green",
            TrafficLightColor::Yellow => "yellow",
            TrafficLightColor::Red => "red",
        };
        write!(f, "{}", color_string)
    }
}

#[derive(Debug)]
pub struct HouseLight {
    on: bool,
}
impl HouseLight {
    pub fn new() -> Self {
        Self { on: false }
    }
    // pub fn get_state(&self) -> bool {
    //     self.on
    // }
}
impl Display for HouseLight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Houselight is {}", if self.on { "on" } else { "off" })
    }
}

// we can share behavior between disparate Structs with Traits
pub trait Light {
    fn get_name(&self) -> &str;
    fn get_state(&self) -> &dyn std::fmt::Debug;
}
impl Light for HouseLight {
    fn get_name(&self) -> &str {
        "House light"
    }

    fn get_state(&self) -> &dyn std::fmt::Debug {
        &self.on
    }
}
impl Light for TrafficLight {
    fn get_name(&self) -> &str {
        "Traffic light"
    }

    fn get_state(&self) -> &dyn std::fmt::Debug {
        &self.color
    }
}
