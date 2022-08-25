use racklight::backlight::Backlight;
use std::env;

fn main() {
    let mut args = env::args();

    let backlight = args.nth(1).unwrap();
    let value = args.next().unwrap();

    let mut backlight = Backlight::new(backlight);

    backlight.set_brightness(value.parse().unwrap());
}
