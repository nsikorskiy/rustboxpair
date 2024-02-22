extern crate libusb;

use std::time::Duration;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("usage: read_device <base-10/0xbase-16> <base-10/0xbase-16>");
        return;
    }

    let vid = convert_argument(args[1].as_ref());
    let pid = convert_argument(args[2].as_ref());

    match libusb::Context::new() {
        Ok(mut context) => {
            match open_device(&mut context, vid, pid) {
                Some(mut handle) => activate_pair(&mut handle).unwrap(),
                None => println!("could not find device {:04x}:{:04x}", vid, pid)
            }
        },
        Err(e) => panic!("could not initialize libusb: {}", e)
    }
}

fn convert_argument(input: &str) -> u16 {
    if input.starts_with("0x") {
        return u16::from_str_radix(input.trim_start_matches("0x"), 16).unwrap();
    }
    u16::from_str_radix(input, 10)
        .expect("Invalid input, be sure to add `0x` for hexadecimal values.")
}

fn open_device(context: &mut libusb::Context, vid: u16, pid: u16) -> Option<libusb::DeviceHandle> {
    let devices = match context.devices() {
        Ok(d) => d,
        Err(_) => return None
    };

    for device in devices.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue
        };

        if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
            match device.open() {
                Ok(handle) => return Some(handle),
                Err(_) => continue
            }
        }
    }

    None
}

fn activate_pair(handle: &mut libusb::DeviceHandle) -> libusb::Result<()> {
    let request_type: u8 = 0xc1;
    let request: u8 = 0x01;
    let value: u16 = 0x0100;
    let index: u16 = 0x00;
    let mut buf: [u8; 255] = [0; 255];
    match handle.read_control(request_type, request, value, index, &mut buf, Duration::from_secs(1)) {
        Ok(ret) => println!("Ret count {}, buf {:?}", ret, buf),
        Err(err) => println!("Err {}", err)
    }
    Ok(())
}

