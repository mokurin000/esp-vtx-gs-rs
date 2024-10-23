use std::{error::Error, time::SystemTime};

use clap::Parser;

mod device;
use crate::device::Device;

#[derive(Parser)]
#[command(version, long_about = None, arg_required_else_help(true))]
struct Cli {
    #[arg(short, long)]
    dev: String,
}

const SHOW_BYTE_SIZE: usize = 30;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args = Cli::parse();
    let mut wlan_dev = Device::new(args.dev);

    let start_time = SystemTime::now();
    let mut cnt = 0;
    while start_time.elapsed().unwrap().as_millis() <= 10 * 1000 {
        if let Ok(packet) = wlan_dev.cap.next_packet() {
            println!("received: {:?}", &packet.data[..SHOW_BYTE_SIZE]);
            println!("packet has {} bytes of data", packet.data.len());
        }

        cnt += 1;
    }

    println!("recv {} packets in 10s", cnt);
    println!("{:?}", wlan_dev.cap.stats());

    Ok(())
}
