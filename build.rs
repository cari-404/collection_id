extern crate winresource;

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("voucher_wallet.ico");
        res.compile().unwrap();
    }
}