mod afk;
mod avatar;
mod banner;
mod info;
mod ping;

pub fn get_commands() -> Vec<utils::CommandTemplate> {
    vec![
        ping::command(),
        afk::command(),
        avatar::command(),
        banner::command(),
        info::command(),
    ]
}
