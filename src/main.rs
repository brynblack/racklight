use racklight::backlight::Backlight;
use std::io;

fn main() {
    let mut backlight = Backlight::new("acpi_video0");
    io::stdin().lines().for_each(|line| {
        let line = line.unwrap();
        backlight.set_brightness(line.parse().unwrap());
    });
}
