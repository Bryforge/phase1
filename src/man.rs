use crate::registry;

pub fn get_man_page(cmd: &str) -> Option<String> {
    registry::man_page(cmd)
}
