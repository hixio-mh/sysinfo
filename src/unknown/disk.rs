//
// Sysinfo
//
// Copyright (c) 2017 Guillaume Gomez
//

use crate::{DiskExt, DiskType};

use std::{ffi::OsStr, path::Path};

/// Struct containing a disk information.
pub struct Disk {}

impl DiskExt for Disk {
    fn type_(&self) -> DiskType {
        unreachable!()
    }

    fn name(&self) -> &OsStr {
        unreachable!()
    }

    fn file_system(&self) -> &[u8] {
        &[]
    }

    fn mount_point(&self) -> &Path {
        Path::new("")
    }

    fn total_space(&self) -> u64 {
        0
    }

    fn available_space(&self) -> u64 {
        0
    }

    fn is_removable(&self) -> bool {
        false
    }

    fn refresh(&mut self) -> bool {
        true
    }
}
