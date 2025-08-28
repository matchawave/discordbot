mod afk;
mod ping;

pub fn get_commands() -> Vec<utils::CommandTemplate> {
    vec![ping::command(), afk::command()]
}
