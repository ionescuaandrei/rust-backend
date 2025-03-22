use std::fmt::Display;

use serde::{Deserialize, Serialize};


#[derive(Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Kind {
    System,
    Process,
    Memory,
    Cpu,
    Disk,
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Kind::System => write!(f, "system"),
            Kind::Process => write!(f, "process"),
            Kind::Memory => write!(f, "memory"),
            Kind::Cpu => write!(f, "cpu"),
            Kind::Disk => write!(f, "disk"),
        }
    }
}