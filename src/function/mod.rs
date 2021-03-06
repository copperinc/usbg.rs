pub mod hid;
pub mod ecm;
pub mod rndis;
pub mod acm;
pub mod mass_storage;

use std::io;
use std::path::Path;

pub trait UsbGadgetFunction {
    fn instance_name(&self) -> &str;
    fn function_type(&self) -> &str;
    fn write_to(&self, functions_path: &Path) -> io::Result<()>;
}
