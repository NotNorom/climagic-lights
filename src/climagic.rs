#![allow(dead_code)]

use std::net::{UdpSocket};
use std::thread::sleep;
use std::time::Duration;

const LEDS: &[u8] = &[
    0b_0011_0001,
    0b_0011_0010,
    0b_0011_0011,
    0b_0011_0100,
    0b_0011_0101,
    0b_0011_0110,
    0b_0011_0111,
    0b_0011_1000,
    0b_0011_1001,
];

const COUNTER: &[u8] = &[
    0b_1000_0000,
    0b_0100_0000,
    0b_0010_0000,
    0b_0001_0000,
    0b_0000_1000,
    0b_0000_0100,
    0b_0000_0010,
    0b_0000_0001,
];


// 49..58
// 0b110001..0b111010

pub struct Lights {
    sock: UdpSocket,
    package_pause_time: Duration,
    package_display_time: Duration,
}

impl Lights {
    pub fn new(local_address: &str, package_display_time: f32, package_pause_time: f32) -> Self {
        Lights {
            sock: UdpSocket::bind(local_address).unwrap(),
            package_pause_time: Duration::from_secs_f32(package_pause_time),
            package_display_time: Duration::from_secs_f32(package_display_time),
        }
    }

    pub fn connect(&self) {
        // lights.climagic.com
        // 45.79.197.21
        self.sock.connect("lights.climagic.com:45444").unwrap();
    }
    pub fn clear(&self) {
        self.sock.send(b"CLEAR").unwrap();
    }
    
    pub fn toggle_led(&self, led: usize) {
        self.sock.send(&[LEDS[led]]).unwrap();
    }

    pub fn toggle_leds(&self, leds: Vec<usize>) {
        // number of available leds is 9 so I hope
        // no one puts in more than that 
        let mut result: Vec<u8> = Vec::with_capacity(9);

        for i in 0..leds.len() {
            result.push(LEDS[leds[i]]);
        }

        self.sock.send(&result).unwrap();
    }

    pub fn sleep(&self, duration: f32) {
        sleep(Duration::from_secs_f32(duration));
    }
    
    pub fn send_bytes(&self, bytes: &[u8]) {
        for byte in bytes {
            let seq = &Self::byte_to_led_sequence(byte);

            self.sock.send(seq).unwrap();
            sleep(self.package_display_time);
            self.sock.send(seq).unwrap();
            sleep(self.package_pause_time);
        }
    }
    
    pub fn byte_to_led_sequence(byte: &u8) -> Vec<u8> {
        // I chose this randomly. Looks like a nice value
        let mut result: Vec<u8> = Vec::with_capacity(32);
        
        for i in 0..COUNTER.len() {
            if COUNTER[i] & byte > 0 {
                result.push(LEDS[i]);
            }
        }

        result
    }
}
