
pub mod data_dashboard_lib {
    use ciborium;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Payload {
        pub data: Vec<Data>,
        pub timestamp: u64,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum Data {
        Location(LocationData),
        Position(PositionData),
        Rotation(RotationData),
        Temperature(TemperatureData),
        Humidity(HumidityData),
        Pressure(PressureData),
        Acceleration(AccelerationData),
        AngularVelocity(AngularVelocityData),
        Message(MessageData),
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct LocationData {
        pub latitude: f64,
        pub longitude: f64,
        pub altitude: f64,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PositionData {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RotationData {
        pub yaw: f64,
        pub pitch: f64,
        pub roll: f64,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct TemperatureData {
        pub temperature: f64,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct HumidityData {
        pub humidity: f64,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PressureData {
        pub pressure: f64,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct AccelerationData {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct AngularVelocityData {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MessageData {
        pub message: String,
    }

    pub fn encode_payload(payload: Payload) -> Vec<u8> {
        let mut encoded_payload: Vec<u8> = Vec::new();

        ciborium::ser::into_writer(&payload, &mut encoded_payload).unwrap();
        encoded_payload
    }

    pub fn decode_payload(encoded_payload: Vec<u8>) -> Payload {
        let decoded_payload: Payload = ciborium::de::from_reader(&encoded_payload[..]).unwrap();

        decoded_payload
    }

}