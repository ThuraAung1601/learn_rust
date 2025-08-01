// Fahrenheit to Celsius conversion formula
pub fn fahr2cel(fahr: i32) -> f32 {
    (5.0/9.0)*(fahr as f32 - 32.0)
}