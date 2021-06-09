use core::time;
use std::{borrow::BorrowMut, thread, time::{Duration, Instant}};

use device::RazerDevice;
use hidapi::*;
use rusb::*;

use crate::razer::RazerPacket;
mod razer;
mod device;


const TIMEOUT: Duration = Duration::from_secs(1);

fn main() {

    let mut context = Context::new().unwrap();

    let mut dev = RazerDevice::new(&mut context, 0x0233).unwrap();


    let mut packet = razer::RazerPacket::new(0x03, 0x0b, &[0xFF, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00]);
    packet.data_size = 60;
    packet.args[15..60].copy_from_slice(&[0xFFu8; 45]);
    println!("OUT -> {}", packet);
    let start = Instant::now();
    match dev.write_and_read_cmd(packet) {
        Ok(p) => {
            println!("Took {} us", start.elapsed().as_micros());
            println!("IN <- {}", p)
        },
        Err(e) => println!("IN <- ERROR {:?}", e)
    }

    let mut packet = razer::RazerPacket::new(0x03, 0x0a, &[0x05, 0x00]);
    packet.data_size = 60;
    println!("OUT -> {}", packet);
    let start = Instant::now();
    match dev.write_and_read_cmd(packet) {
        Ok(p) => {
            println!("Took {} us", start.elapsed().as_micros());
            println!("IN <- {}", p)
        },
        Err(e) => println!("IN <- ERROR {:?}", e)
    }

    /*
    let api = HidApi::new().expect("Error obtaining HID api");

    #[cfg(unix)]
    let mut devices: Vec<&DeviceInfo> = api.device_list().filter(|x| x.vendor_id() == 0x1532).collect();

    #[cfg(windows)]
    let mut devices: Vec<&DeviceInfo> = api.device_list().filter(|x| x.vendor_id() == 0x1532 && x.usage() == 128).collect();
    
    #[cfg(windows)]
    let mut a = 0;

    //devices.dedup_by_key(|x| x.product_id());
    if devices.len() == 0 {
        panic!("No razer devices found");
    }

    for d in &devices {
        println!("{:?} {}", &d, &d.usage());
    }

    let mut buffer: [u8; 91] = [0; 91];

    for d in &devices {
        println!("Found {} (0x{:04X})", d.product_string().unwrap(), d.product_id());
        let x = d.open_device(&api).expect("Unable to open device!");

        let packet = razer::RazerPacket::new(0x0d, 0x01, &[0x00, 0x00, 0x00, 0x00]);
        println!("{}", packet);

        x.write(&packet.create_packet()).expect("Error writing");

        thread::sleep(std::time::Duration::from_micros(1000));
        x.read(&mut buffer).expect("Error reading response");

        //let packet = razer::RazerPacket::new(0x0d, 0x01, &[0x00, 0x01, 0x40]);
        //println!("{}", packet);
        //x.send_feature_report(&packet.create_packet()).expect("Error writing");
        //thread::sleep(std::time::Duration::from_micros(1000));
        //x.get_feature_report(&mut buffer).expect("Error reading response");
        //println!("{}", RazerPacket::from_raw(&buffer));


        //let packet = razer::RazerPacket::new(0x0d, 0x01, &[0x00, 0x01, 0x55]);
        //println!("{}", packet);
//
        //x.send_feature_report(&packet.create_packet()).expect("Error writing");
        //thread::sleep(std::time::Duration::from_micros(1000));
        //x.get_feature_report(&mut buffer).expect("Error reading response");
        //println!("{}", RazerPacket::from_raw(buffer));
    }
    */
}
