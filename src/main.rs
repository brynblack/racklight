use racklight::backlight::Backlight;
use std::{env, path::Path};

const BACKLIGHT_PATH: &str = "/sys/class/backlight";

fn main() {
    // Collect arguments
    let mut args = env::args();
    let value = args.nth(1).unwrap();

    // Retrieve first backlight in directory
    let mut backlights: Vec<_> = Path::new(BACKLIGHT_PATH)
        .read_dir()
        .unwrap()
        .map(|d| d.unwrap())
        .collect();

    // Sort backlights alphabetically
    backlights.sort_by_key(|d| d.path());

    // Create a new backlight from the first backlight
    let mut backlight = Backlight::new(backlights[0].path());

    // Set the backlight brightness to the given value
    backlight.set_brightness(value.parse().unwrap());
}
