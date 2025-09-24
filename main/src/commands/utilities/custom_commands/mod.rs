mod page_test;
pub fn get_commands() -> Vec<utils::CommandTemplate> {
    vec![page_test::command()]
}
