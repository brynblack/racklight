// Copyright (C) 2022  Brynley Llewellyn-Roux
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Module for managing a backlight.

use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

/// A display backlight.
pub struct Backlight {
    bright_pth: PathBuf,
    cur_bright: u8,
    min_bright: u8,
    max_bright: u8,
}

impl Backlight {
    /// Creates a new backlight.
    ///
    /// Takes in a `Path` containing the name of the backlight.
    ///
    /// # Examples
    ///
    /// ```
    /// use racklight::backlight::Backlight;
    ///
    /// let backlight = Backlight::new("acpi_video0");
    /// ```
    pub fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        // Join backlight paths
        let path = Path::new("/sys/class/backlight").join(path);

        // Initialise default values
        Self {
            bright_pth: path.to_path_buf(),
            cur_bright: 0,
            min_bright: 0,
            max_bright: 0,
        }
    }

    fn get(&self, file: &str) -> u8 {
        // Read file contents and parse to u8
        fs::read_to_string(self.bright_pth.join(file))
            .unwrap()
            .trim()
            .parse()
            .unwrap()
    }

    fn set(&self, file: &str, value: u8) {
        // Open file to prepare for writing into it
        let mut file = OpenOptions::new()
            .write(true)
            .open(self.bright_pth.join(file))
            .unwrap();

        // Write the value to the file
        file.write_all(value.to_string().as_bytes()).unwrap();
    }

    fn update(&mut self) {
        // Retrieve values from files and update self with values
        self.cur_bright = self.get("brightness");
        self.max_bright = self.get("max_brightness");
    }

    /// Sets the backlight's brightness.
    ///
    /// Takes in a `u8` as a value to set the brightness to.
    ///
    /// # Examples
    ///
    /// ```
    /// use racklight::backlight::Backlight;
    ///
    /// let mut backlight = Backlight::new("acpi_video0");
    /// backlight.set_brightness(20);
    /// ```
    pub fn set_brightness(&mut self, mut value: u8) {
        // Update values first
        self.update();

        // Clamp value to minimum or maximum brightness
        value = value.max(self.min_bright).min(self.max_bright);

        // Set the brightness to the given value.
        self.set("brightness", value);
    }
}
