mod settings;
pub fn get_modules() -> Vec<utils::CommandTemplate> {
    let mut modules = vec![];
    modules.extend(settings::get_commands());
    modules
}
