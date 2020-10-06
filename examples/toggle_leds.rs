use climagic::Lights;

fn main() {
    let lights = Lights::new("0.0.0.0:45444", 2.0, 1.0);
    lights.connect();
    lights.clear();
    lights.toggle_leds(vec![0,2,4,6,8]);
    lights.sleep(2.0);
    lights.toggle_led(4);
    lights.sleep(2.0);
    lights.clear();
}