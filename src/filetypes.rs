use crate::dir_entry;
use crate::filesystem;
use std::fs;

use faccess::PathExt;

/// Whether or not to show
#[derive(Default)]
pub struct FileTypes {
    pub files: bool,
    pub directories: bool,
    pub symlinks: bool,
    pub symlinks_valid: bool,
    pub symlinks_broken: bool,
    pub block_devices: bool,
    pub char_devices: bool,
    pub sockets: bool,
    pub pipes: bool,
    pub executables_only: bool,
    pub empty_only: bool,
}

impl FileTypes {
    pub fn should_ignore(&self, entry: &dir_entry::DirEntry) -> bool {
        if let Some(ref entry_type) = entry.file_type() {
            let is_valid = || {
                entry_type.is_symlink()
                    && fs::read_link(entry.path())
                        .and_then(|path| fs::metadata(path))
                        .is_ok()
            };

            (!self.files && entry_type.is_file())
                || (!self.directories && entry_type.is_dir())
                || !(self.symlinks
                    || !entry_type.is_symlink()
                    || self.symlinks_valid && !is_valid()
                    || self.symlinks_broken && is_valid())
                || (!self.block_devices && filesystem::is_block_device(*entry_type))
                || (!self.char_devices && filesystem::is_char_device(*entry_type))
                || (!self.sockets && filesystem::is_socket(*entry_type))
                || (!self.pipes && filesystem::is_pipe(*entry_type))
                || (self.executables_only && !entry.path().executable())
                || (self.empty_only && !filesystem::is_empty(entry))
                || !(entry_type.is_file()
                    || entry_type.is_dir()
                    || entry_type.is_symlink()
                    || filesystem::is_block_device(*entry_type)
                    || filesystem::is_char_device(*entry_type)
                    || filesystem::is_socket(*entry_type)
                    || filesystem::is_pipe(*entry_type))
        } else {
            true
        }
    }
}
