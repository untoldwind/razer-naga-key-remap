use evdev_rs::{Device, DeviceWrapper, GrabMode, InputEvent, ReadFlag, ReadStatus};
use std::fs::{read_dir, File};

pub struct Naga {
    device: Device,
}

const DEVICE_NAMES: &[(&str, &str)] = &[
    ("Razer Razer Naga 2014", "/input2"),
    ("Razer Razer Naga Trinity", "/input2"),
];

impl Naga {
    pub fn new() -> Result<Naga, String> {
        let paths = read_dir("/dev/input")
            .map_err(|e| format!("Problem reading input devices dir: {}", e))?;

        for path_result in paths {
            let path = match path_result {
                Ok(p) => p,
                Err(_) => {
                    continue;
                }
            };

            let file = match File::open(path.path()) {
                Ok(f) => f,
                Err(_) => {
                    continue;
                }
            };

            let mut device = match Device::new_from_file(file) {
                Ok(d) => d,
                Err(_) => {
                    continue;
                }
            };

            for (name, phys) in DEVICE_NAMES {
                if device.name().unwrap_or("").eq(*name)
                    && device.phys().unwrap_or("").ends_with(*phys)
                {
                    device
                        .grab(GrabMode::Grab)
                        .map_err(|e| format!("Could not grab device: {}", e))?;
                    return Ok(Naga { device });
                }
            }
        }

        return Err("No device found".to_string());
    }

    pub fn next_event(&self) -> Result<(ReadStatus, InputEvent), String> {
        match self
            .device
            .next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING)
        {
            Ok(res) => Ok(res),
            Err(errno) => Err(format!("Problem reading event: {}", errno)),
        }
    }
}
