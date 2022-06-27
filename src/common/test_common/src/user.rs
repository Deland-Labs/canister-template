use ic_cdk::export::Principal;
use rstest::*;

#[fixture]
pub fn anonymous_user() -> Principal {
    Principal::anonymous()
}

pub fn mock_user(index: u32) -> Principal {
    let mut principal_bytes = vec![0u8; 29];
    // The first four bytes are the index.
    principal_bytes[0..4].copy_from_slice(&index.to_be_bytes());
    Principal::from_slice(&principal_bytes)
}


#[fixture]
pub fn mock_user1() -> Principal {
    mock_user(1)
}

#[fixture]
pub fn mock_user2() -> Principal {
    mock_user(2)
}

#[fixture]
pub fn mock_user3() -> Principal {
    mock_user(3)
}

#[fixture]
pub fn mock_now() -> u64 {
    15_844_844_000_000_000
}
