use ch3f_utils::db;
use std::env;

#[test]
fn db_connection_thrwos_error_if_env_var_not_defined() {
    env::remove_var(db::ENV_DB_PATH);
    assert!(db::establish_connection().is_err());
}
