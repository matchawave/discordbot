mod prefix;

pub fn get_commands() -> Vec<utils::CommandTemplate> {
    vec![prefix::command()]
}
