use super::*;
use crate::named_principals::lines_hashset;
use crate::test_common::test::init_test_logger;
use candid::Principal;
use log::info;
use rstest::*;

#[rstest]
fn test_accept_multiline_env() {
    init_test_logger();
}
