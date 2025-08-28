mod custom_commands;
mod embeds;
mod information;
mod reminder;
mod timer;

pub fn get_modules() -> Vec<utils::CommandTemplate> {
    let mut modules = vec![];
    modules.extend(information::get_commands());

    modules
}
