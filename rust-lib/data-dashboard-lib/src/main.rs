use data_dashboard_lib::data_dashboard_lib::{decode_payload, encode_payload, Data, HumidityData, LocationData, Payload, PositionData, PressureData, RotationData, TemperatureData};

fn main() {
    let payload = Payload {
        data: vec![
            Data::Location(LocationData {
                latitude: 37.7749,
                longitude: 122.4194,
                altitude: 0.0,
            }),
            Data::Position(PositionData {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            Data::Rotation(RotationData {
                yaw: 0.0,
                pitch: 0.0,
                roll: 0.0,
            }),
            Data::Temperature(TemperatureData {
                temperature: 0.0,
            }),
            Data::Humidity(HumidityData {
                humidity: 0.0,
            }),
            Data::Pressure(PressureData {
                pressure: 0.0,
            }),
        ],
        timestamp: 0,
    };

    let encoded = encode_payload(payload);
    println!("encoded: {:?}", encoded);
    // print the encoded with 0x00 format
    for byte in encoded.iter() {
        print!("0x{:02x}, ", byte);
    }

    let decoded = decode_payload(encoded);
    println!("decoded: {:?}", decoded);
}