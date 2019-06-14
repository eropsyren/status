macro_rules! battery_path {
    () => { "/sys/class/power_supply/BAT0" };
}

macro_rules! cpu_temp_path {
    () => { "/sys/class/hwmon/hwmon2" };
}

pub const DATA: [(&'static str, &'static str); 5] = [
    ("Battery status", concat!(battery_path!(), "/status")),
    ("Battery capacity", concat!(battery_path!(), "/capacity")),
    ("Cpu package", concat!(cpu_temp_path!(), "/temp1_input")),
    ("Cpu core 1", concat!(cpu_temp_path!(), "/temp2_input")),
    ("Cpu core 2", concat!(cpu_temp_path!(), "/temp3_input")),
];
