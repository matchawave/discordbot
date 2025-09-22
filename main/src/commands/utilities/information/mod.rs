mod afk;
mod avatar;
mod banner;
mod channel;
mod guild;
mod info;
mod ping;
mod serverinfo;

pub fn get_commands() -> Vec<utils::CommandTemplate> {
    vec![
        ping::command(),
        afk::command(),
        avatar::command(),
        banner::command(),
        info::command(),
        channel::command(),
        guild::command(),
        serverinfo::command(),
    ]
}
