mod analytics;
mod moderation;
mod music;
mod settings;

pub fn get_modules() -> Vec<utils::CommandTemplate> {
    let mut modules = vec![];
    modules.extend(settings::get_commands());
    modules.extend(moderation::get_commands());
    modules
}
