use climagic::Lights;

fn main() {
    let lights = Lights::new("0.0.0.0:45444", 2.0, 1.0);
    lights.connect();
    lights.clear();
    lights.send_bytes(b"Hey there!");
    lights.sleep(2.0);
    lights.clear();
}