use rand::RngExt;

pub fn generate_otp() -> String {
    let code: u32 = rand::rng().random_range(0..1_000_000);
    format!("{:06}", code)
}
