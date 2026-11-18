pub mod util {
    use rand::RngExt;

    // Constants
    pub const INF: f64 = f64::INFINITY;
    pub const PI: f64 = std::f64::consts::PI;

    // Utility Functions
    pub fn degrees_to_radians(degrees: f64) -> f64 {
        return degrees * PI / 180.0;
    }

    pub fn rnd() -> f64 {
        // Is it a bad idea to create a new rand::rng each time? Probably!
        // Docs say that using a local bind avoids repeated init-checks.
        // ?? Perhaps I can move the initialization out, and pass a local rng var?
        rand::rng().random::<f64>()
    }

    pub fn rnd_range(min: f64, max: f64) -> f64 {
        // rand::rng().random_range(min..max)
        min + (max-min)*rnd() // somehow, this is faster than the built-in range, according to my tests.
    }
}