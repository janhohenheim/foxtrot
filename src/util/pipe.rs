use anyhow::Result;
use bevy::prelude::*;
use std::process::exit;

pub fn log_errors(In(result): In<Result<()>>) {
    if let Err(error) = result {
        error!("{:?}", error);
        exit(-1);
    }
}
