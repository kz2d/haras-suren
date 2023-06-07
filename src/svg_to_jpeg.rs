use core::fmt;
use std::{error::Error, process::Command, fs::{self, read}, io};

pub fn render_profile() -> io::Result<Vec<u8>> {
    Command::new("convert-svg-to-png").arg("test.svg").spawn().unwrap().wait().unwrap();
    
    read("test.png")
}