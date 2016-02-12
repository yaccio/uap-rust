use ua;
use os;
use device;

///`Client` struct, contains the parsed user agent information.
#[derive(Debug, PartialEq, Eq)]
pub struct Client {
    pub user_agent: ua::UserAgent,
    pub os: os::OS,
    pub device: device::Device,
}
