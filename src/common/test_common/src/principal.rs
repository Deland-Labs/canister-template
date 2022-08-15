use common::types::CanisterId;
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

const TYPE_OPAQUE: u8 = 0x01;
fn mock_canister(val: u64) -> CanisterId {
    let mut data = [0_u8; 10];

    // Specify explicitly the length, so as to assert at compile time that a u64
    // takes exactly 8 bytes
    let val: [u8; 8] = val.to_be_bytes();

    // for-loops in const fn are not supported
    data[0] = val[0];
    data[1] = val[1];
    data[2] = val[2];
    data[3] = val[3];
    data[4] = val[4];
    data[5] = val[5];
    data[6] = val[6];
    data[7] = val[7];

    // Even though not defined in the interface spec, add another 0x1 to the array
    // to create a sub category that could be used in future.
    data[8] = 0x01;
    data[9] = TYPE_OPAQUE;

    CanisterId(Principal::try_from_slice(data.as_slice()).unwrap())
}

#[fixture]
pub fn mock_canister1() -> CanisterId {
    mock_canister(1)
}

#[fixture]
pub fn mock_canister2() -> CanisterId {
    mock_canister(2)
}

#[fixture]
pub fn mock_canister3() -> CanisterId {
    mock_canister(3)
}
