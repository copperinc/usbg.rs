extern crate std;

use std::fs;
use std::io;
use std::path::Path;

use UsbGadgetFunction;
use util::write_data;

#[derive(Clone)]
pub struct MassStorageFunction<'a> {
    pub instance_name: &'a str,
    pub file: &'a str,
	pub ro: &'a str,
	pub removable: &'a str,
	pub cdrom: &'a str,
	pub nofua: &'a str,
}

impl<'a> UsbGadgetFunction for MassStorageFunction<'a> {
    fn instance_name(&self) -> &str {
        return self.instance_name;
    }

    fn function_type(&self) -> &str {
        return "mass_storage";
    }

    fn write_to(&self, functions_path: &Path) -> io::Result<()> {
        let fname = format!("{func_type}.{instance}",
                            func_type = self.function_type(),
                            instance = self.instance_name());
        let function_path = functions_path.join(fname);
        try!(fs::create_dir(&function_path));
        // function attributes
        let lun_path = function_path.join("lun.0");
        try!(write_data(lun_path.join("file").as_path(),
                        format!("{}", self.file).as_bytes()));
        try!(write_data(lun_path.join("ro").as_path(),
                        format!("{}", self.ro).as_bytes()));
        try!(write_data(lun_path.join("removable").as_path(),
                        format!("{}", self.removable).as_bytes()));
        try!(write_data(lun_path.join("cdrom").as_path(),
                        format!("{}", self.cdrom).as_bytes()));
        try!(write_data(lun_path.join("nofua").as_path(),
                        format!("{}", self.nofua).as_bytes()));

        return Ok(());
    }
}
