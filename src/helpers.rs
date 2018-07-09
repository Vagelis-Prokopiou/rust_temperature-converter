pub fn round(number: &f32, decimals: u8) -> f32 {
    let rounder: f32 = 10_f32.powf(decimals as f32);
    return (number * rounder).round() / rounder;
}

pub fn is_valid_temperature_scale(temperature_scale: &str) -> bool {
    return temperature_scale == "F" || temperature_scale == "C";
}

pub fn convert(temperature: &f32, temperature_scale: &str) -> f32 {
    if !is_valid_temperature_scale(temperature_scale) {
        panic!("Expected value 'F' or 'C'");
    }

    let decimals: u8 = 2;

    if temperature_scale == "F" {
        return round(&(temperature * 1.8 + 32_f32), decimals);
    }

    return round(&((temperature - 32_f32) * 0.5556), decimals);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_test() {
        assert_eq!(round(&0.1234_f32, 1), 0.1);
        assert_eq!(round(&0.1234_f32, 2), 0.12);
        assert_eq!(round(&0.1234_f32, 3), 0.123);
    }

    #[test]
    fn is_valid_temperature_scale_test() {
        assert_eq!(is_valid_temperature_scale(&"F"), true);
        assert_eq!(is_valid_temperature_scale(&"C"), true);
        assert_eq!(is_valid_temperature_scale(&"a"), false);
    }

    #[test]
    fn convert_test() {
        let decimals: u8 = 2;

        // To Fahrenheit
        assert_eq!(convert(&(0.0), &"F"), round(&32.0_f32, decimals));
        assert_eq!(convert(&(2.0), &"F"), round(&35.6_f32, decimals));
        assert_eq!(convert(&(3.4), &"F"), round(&38.12_f32, decimals));
        assert_eq!(convert(&(5.0), &"F"), round(&41.0_f32, decimals));
        assert_eq!(convert(&(23.0), &"F"), round(&73.4_f32, decimals));

        // To Celsius
        assert_eq!(convert(&(0.0), &"C"), round(&-17.78_f32, decimals));
        assert_eq!(convert(&(2.0), &"C"), round(&-16.67_f32, decimals));
        assert_eq!(convert(&(3.4), &"C"), round(&-15.89_f32, decimals));
        assert_eq!(convert(&(5.0), &"C"), round(&-15.0_f32, decimals));
        assert_eq!(convert(&(23.0), &"C"), round(&-5.0_f32, decimals));
    }
}
