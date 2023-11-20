use std::env;

use r_env::FromEnv;

#[derive(Debug, FromEnv, Default, Eq, PartialEq)]
struct TestStruct {
    #[env_attr(name = "TEST_NAME", default = "name")]
    pub test_name: String,
    #[env_attr(name = "TEST_NUM_TYPE_USIZE", default = 10)]
    pub test_num_type_usize: usize,
    #[env_attr(name = "TEST_NUM_TYPE_U16", default = 32)]
    pub test_num_type_u16: u16,
    #[env_attr(name = "TEST_OPTION", exec = Some(33))]
    pub test_option: Option<usize>,
    #[env_attr(name = "TEST_OPTION_BASE", exec = test_option.map(| data | { data + 3 }).unwrap_or(3))]
    pub test_exec: usize,
}

fn main() {
    env::set_var("TEST_NAME", "test");
    env::set_var("TEST_NUM_TYPE_USIZE", "1");
    env::set_var("TEST_NUM_TYPE_U16", "2");

    let test_struct = TestStruct::from_env();
    assert_ne!(TestStruct {
        test_name: "name".to_string(),
        test_num_type_usize: 10,
        test_num_type_u16: 32,
        test_option: Some(3),
        test_exec: 5,
    }, test_struct);

    assert_eq!(TestStruct {
        test_name: "test".to_string(),
        test_num_type_usize: 1,
        test_num_type_u16: 2,
        test_option: Some(33),
        test_exec: 36,
    }, test_struct);
}