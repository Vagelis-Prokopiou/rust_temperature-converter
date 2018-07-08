pub fn convert(temperature: &f32, temperature_scale: &str) -> f32 {
    if temperature_scale != "F" && temperature_scale != "C" {
        panic!("Expected value 'F' or 'C'");
    }

    if temperature_scale == "F" {
        return temperature * 1.8 + 32 as f32;
    }

    return (temperature - 32 as f32) * 0.5556;
}