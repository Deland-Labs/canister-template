use crate::state::{State, STATE};
use crate::stats_service::{Stats, StatsService};
use candid::candid_method;
use ic_cdk::api;
use ic_cdk_macros::*;
use log::{debug, error, info};
use std::collections::HashMap;
