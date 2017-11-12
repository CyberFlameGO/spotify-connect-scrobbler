#![crate_name = "librespot"]

#![cfg_attr(feature = "cargo-clippy", allow(unused_io_amount))]

// TODO: many items from tokio-core::io have been deprecated in favour of tokio-io
#![allow(deprecated)]

#[macro_use] extern crate log;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;

extern crate base64;
extern crate crypto;
extern crate futures;
extern crate hyper;
extern crate mdns;
extern crate num_bigint;
extern crate protobuf;
extern crate rand;
extern crate rustfm_scrobble;
extern crate tokio_core;
extern crate url;

pub extern crate librespot_core as core;
pub extern crate librespot_protocol as protocol;
pub extern crate librespot_metadata as metadata;

#[cfg(feature = "alsa-backend")]
extern crate alsa;

#[cfg(feature = "portaudio-rs")]
extern crate portaudio_rs;

#[cfg(feature = "libpulse-sys")]
extern crate libpulse_sys;

pub mod discovery;
pub mod keymaster;
pub mod scrobbler;

include!(concat!(env!("OUT_DIR"), "/lib.rs"));
