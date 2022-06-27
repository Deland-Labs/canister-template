use std::collections::HashSet;
use std::fmt::{Display, Formatter};

use std::io::{Read, Write};

use candid::{CandidType, Deserialize, Principal};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;

use crate::constants::{
    PAGE_INPUT_MAX_LIMIT, PAGE_INPUT_MAX_OFFSET, PAGE_INPUT_MIN_LIMIT, PAGE_INPUT_MIN_OFFSET,
};
use crate::errors::{ErrorInfo, ServiceResult};

#[cfg(test)]
mod tests;

