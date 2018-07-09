pub fn is_valid_temperature_scale(temperature_scale: &str) -> bool {
    return temperature_scale == "F" || temperature_scale == "C";
}

pub fn convert(temperature: &f32, temperature_scale: &str) -> f32 {
    if !is_valid_temperature_scale(temperature_scale) {
        panic!("Expected value 'F' or 'C'");
    }

    if temperature_scale == "F" {
        return temperature * 1.8 + 32 as f32;
    }

    return (temperature - 32 as f32) * 0.5556;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_temperature_scale_test() {
        assert_eq!(is_valid_temperature_scale(&"F"), true);
        assert_eq!(is_valid_temperature_scale(&"C"), true);
        assert_eq!(is_valid_temperature_scale(&"a"), false);
    }

    #[test]
    fn convert_test() {
        // To Fahrenheit
        assert_eq!(convert(&(0 as f32), &"F"), 32);
        assert_eq!(convert(&(2 as f32), &"F"), 35.6);
        assert_eq!(convert(&(5 as f32), &"F"), 41);
        assert_eq!(convert(&(23 as f32), &"F"), 73.4);

        // To Celsius
        assert_eq!(convert(&(0 as f32), &"C"), -17.7778);
        assert_eq!(convert(&(2 as f32), &"C"), -16.6667);
        assert_eq!(convert(&(5 as f32), &"C"), -15);
        assert_eq!(convert(&(23 as f32), &"C"), -5);
    }
}
