use bakunin_config::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizeConfig {
    pub path: String,
    pub r#move: bool,
    pub copy_on_fail: bool,
}

impl OrganizeConfig {
    pub fn read(value: &Value) -> Self {
        let def = OrganizeConfig::default();
        if !value.is_map() {
            log::warn!("Invalid organize config value, using default");
            return def;
        }

        OrganizeConfig {
            path: value.get("path").try_into_string().unwrap_or(def.path),
            r#move: value.get("move").try_into_bool().unwrap_or(def.r#move),
            copy_on_fail: value.get("copy_on_fail").try_into_bool().unwrap_or(def.copy_on_fail),
        }
    }
}

impl Default for OrganizeConfig {
    fn default() -> Self {
        OrganizeConfig {
            path: String::from("./%albumartist%/%album% (%year%)/%track% %title% - %artist% [%album%](%year%)"),
            r#move: false,
            copy_on_fail: false,
        }
    }
}