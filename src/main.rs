use racklight::backlight::Backlight;
use std::env;

fn main() {
    // Collect arguments
    let mut args = env::args();
    let backlight = args.nth(1).unwrap();
    let value = args.next().unwrap();

    // Create a new backlight with the given path
    let mut backlight = Backlight::new(backlight);

    // Set the backlight brightness to the given value
    backlight.set_brightness(value.parse().unwrap());
}
