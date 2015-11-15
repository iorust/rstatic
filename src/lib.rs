extern crate iron;
extern crate staticfile;
extern crate mount;

use std::path::Path;
// use std::time::Duration;

use iron::Iron;
use staticfile::Static;
use mount::Mount;

pub struct Serve<'a> {
    pub port: u16,
    pub path: &'a str,
    pub maxage: u64,
}

impl<'a> Serve<'a> {
    pub fn listen(&self) {
        let mut mount = Mount::new();
        let handler = Static::new(Path::new(&self.path));
            // .cache(Duration::from_secs(self.maxage));

        mount.mount("", handler);
        Iron::new(mount).http(("127.0.0.1", self.port)).expect("Failed to serve");
    }
}
