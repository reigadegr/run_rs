use crate::{utils};

pub fn reflex_start(env_name: &str) {
    let _ = utils::run_command(format!("conda acivate {} && reflex run", env_name));
}
