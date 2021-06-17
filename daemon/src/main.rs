use core::time;
use std::{borrow::BorrowMut, process::exit, thread, time::{Duration, Instant}};

use chroma::set_keyboard_effect;
use common::effects::{CaptureDisplayEffect, Effect, EffectLayer};
use device::{RAZER_VENDOR_ID, RazerDevice};
use hidapi::*;
use rand::Rng;
use rusb::*;

use crate::razer::RazerPacket;
mod razer;
mod device;
mod chroma;


const TIMEOUT: Duration = Duration::from_secs(1);

struct HotPlugHandler;

/*
impl<T: UsbContext> rusb::Hotplug<T> for HotPlugHandler {
    fn device_arrived(&mut self, device: Device<T>) {
        println!("Device added! Scanning");
        if let Ok(mut ctx) = Context::new() {
            if let Ok(dev) = RazerDevice::scan_devices(ctx.borrow_mut()) {
                
            }
        }
    }

    fn device_left(&mut self, device: Device<T>) {
        println!("Device removed! Scanning");
        if let Ok(mut ctx) = Context::new() {
            if let Ok(dev) = RazerDevice::scan_devices(ctx.borrow_mut()) {
                
            }
        }
    }
}
*/

fn main() {
    //rusb::set_log_level(rusb::LogLevel::Debug);
    //let mut context = Context::new().unwrap();
    let mut context = HidApi::new().unwrap();

    let mut devices = match RazerDevice::scan_devices(&mut context) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Error scanning for devices! {:?}", e);
            exit(1)
        }
    };

    /*
    if rusb::has_hotplug() {
        thread::spawn(move || {
            if let Ok(mut reg) = context.register_callback(Some(RAZER_VENDOR_ID), None, None, Box::new(HotPlugHandler{})) {
                loop {
                    context.handle_events(None).unwrap();
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            } else {
                println!("Error creating hotplug!");
            }
        });
    } else {
        println!("Device does not support hotplug :(");
    }
    */

    for x in &devices {
        println!("{:?} - SN: {}", x.device_type, x.serial);
    }

    let mut laptop: &mut RazerDevice = devices[0].borrow_mut();
    let mut rng = rand::thread_rng();
    let mut layer = EffectLayer::create_blank([[true; 15]; 6]);
    if let Some(mut effect) = CaptureDisplayEffect::new(layer.get_width(), layer.get_height()) {
        effect.init(&mut layer);
        let effect_interval = Duration::from_millis(40);
        loop {
            let now = Instant::now();
            effect.update(&mut layer);
            set_keyboard_effect(laptop, &layer);

            if let Some(remain) = effect_interval.checked_sub(now.elapsed()) {
                std::thread::sleep(remain);
            }
        }
    }

    //loop{std::thread::sleep(std::time::Duration::from_millis(100))}

    /*
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
    */
}
