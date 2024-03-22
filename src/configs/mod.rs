use lazy_static::lazy_static;

use self::server_config::EnvValue;

mod server_config;

lazy_static! {
    pub static ref ENV_VALUES: EnvValue = EnvValue::init();
}
