use uuid::Uuid;


///
pub fn gen_uuid() -> String {
    Uuid::new_v4().to_string()
}


///
pub fn str_to_float(val: &str) -> Result<f64, String> {
    match val.parse::<f64>() {
        Ok(parsed) => Ok(parsed),
        Err(_) => Err(format!("Error parsing float from {}", val)),
    }
}


///
pub fn str_to_usize(val: &str) -> Result<usize, String> {
    match val.parse::<usize>() {
        Ok(parsed) => Ok(parsed),
        Err(_) => Err(format!("Error parsing usize from {}", val)),
    }
}
