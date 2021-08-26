use std::{env, env::VarError};
pub fn get_terminal() -> Result<String, VarError> {
    match env::var("TERM") {
        Ok(s) => Ok(s),
        Err(e) => Err(e),
    }
}
