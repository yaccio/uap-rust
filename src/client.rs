use ua;
use os;
use device;

#[derive(Debug)]
pub struct Client {
    pub user_agent: ua::UserAgent,
    pub os: os::OS,
    pub device: device::Device,
}
