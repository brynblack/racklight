use racklight::backlight::{Backlight, Ops};
use std::{env, path::Path, process};

const BACKLIGHT_PATH: &str = "/sys/class/backlight";

fn main() {
    let mut args = env::args();

    // Collect the second argument
    let mut value = args.nth(1).unwrap_or_else(|| {
        eprintln!("No argument provided!");
        process::exit(1);
    });

    // Determine which operation to perform
    let op = match value.as_bytes()[0] as char {
        '+' => Ops::Increase,
        '-' => Ops::Decrease,
        _ => Ops::Set,
    };

    // Remove all symbols from the value
    value.retain(|c| !"+-".contains(c));

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
    backlight.set_brightness(op, value.parse().unwrap());
}
