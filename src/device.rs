use pcap::{Active, Capture, Inactive};

#[allow(dead_code)]
pub struct Device {
    pub dev_name: String,
    pub cap: Capture<Active>,
}

unsafe impl Sync for Device {}
unsafe impl Send for Device {}

/*
    change channel of device:
    refer scripts/set_netcard.sh
*/
impl Device {
    pub fn new(dev_name: String) -> Self {
        let mut cap: Option<Capture<Inactive>> = None;
        for i in pcap::Device::list().unwrap() {
            if i.name == dev_name {
                cap = Some(Capture::from_device(i).unwrap());
                break;
            }
        }

        if cap.is_none() {
            panic!(
                "could not find the device:{} or could not open it!",
                dev_name
            );
        }

        let cap = unsafe { cap.unwrap_unchecked() }
            .snaplen(1800) // limit captured length
            .promisc(true)
            .rfmon(true)
            .immediate_mode(true)
            .buffer_size(16000000);

        let active_cap = cap.open().unwrap();

        Device {
            dev_name,
            cap: active_cap,
        }
    }
}
