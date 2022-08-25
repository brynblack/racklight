# racklight - 🚀 Blazingly fast CLI brightness control 🚀 (WIP)
- Custom bezier curves (WIP)
- Safe, reliable usage
- Tiny binary size
- Minimal dependences

## Usage
Currently, racklight does not have any Udev rules set, so the only way to set the brightness is to run it as root.
This will be fixed at some point in the future, however for the meantime you can use something like sudo or doas to run the binary.

```
sudo -E cargo run acpi_video0 30
```