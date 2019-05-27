extern crate std;

use std::fs;
use std::io;
use std::path::Path;

use UsbGadgetFunction;
use util::write_data;

#[derive(Clone)]
pub struct ACMFunction<'a> {
    pub instance_name: &'a str,
}

impl<'a> UsbGadgetFunction for ACMFunction<'a> {
    fn instance_name(&self) -> &str {
        return self.instance_name;
    }

    fn function_type(&self) -> &str {
        return "acm";
    }

    fn write_to(&self, functions_path: &Path) -> io::Result<()> {
        let fname = format!("{func_type}.{instance}",
                            func_type = self.function_type(),
                            instance = self.instance_name());
        let function_path = functions_path.join(fname);
        try!(fs::create_dir(&function_path));

        return Ok(());
    }
}
