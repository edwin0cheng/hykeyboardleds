use evdev::*;
use std::path::PathBuf;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn find_device_for_leds() -> Option<PathBuf> {
    let devices = evdev::enumerate();
    for (path, device) in devices {
        if let Some(s) = device.supported_leds() {
            if s.contains(LedCode::LED_NUML) || s.contains(LedCode::LED_CAPSL) {
                return Some(path);
            }
        }
    }
    None
}

fn print_widget(led_state: &AttributeSet<LedCode>) {
    let leds = [(LedCode::LED_CAPSL, "A"), (LedCode::LED_NUML, "1")];
    let mut led_widgets = vec![];

    for (led, name) in &leds {
        let locked = led_state
            .contains(*led)
            .then(|| "locked")
            .unwrap_or_default();

        led_widgets.push(format!(
            "(label :class '{locked} led' :valign 'center' :text '{name}' :width 18 )"
        ));
    }

    println!("(box :spacing 6 {})", led_widgets.join(" "));
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let path = find_device_for_leds().ok_or("No device found")?;
    let mut device = Device::open(path)?;

    // Get the current state of the LEDs
    let mut led_state = device.get_led_state()?;

    // Print the initial state
    print_widget(&led_state);

    loop {
        for event in device.fetch_events().unwrap() {
            match event.destructure() {
                EventSummary::Led(_, led, value) => {
                    if value == 0 {
                        led_state.remove(led);
                    } else {
                        led_state.insert(led);
                    }

                    print_widget(&led_state);
                }
                _ => {}
            }
        }
    }
}
