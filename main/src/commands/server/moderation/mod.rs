mod clear_snipe;
mod edit_snipe;
mod reaction_history;
mod reaction_snipe;
mod snipe;

pub fn get_commands() -> Vec<utils::CommandTemplate> {
    vec![
        snipe::command(),
        clear_snipe::command(),
        edit_snipe::command(),
        reaction_history::command(),
        reaction_snipe::command(),
    ]
}
