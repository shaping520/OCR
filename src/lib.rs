use android_logger::Config;
use log::Level;

#[no_mangle]
pub extern "system" fn Java_top_yogiczy_mytv_RustBridge_helloFromRust() {
    android_logger::init_once(
        Config::default().with_min_level(Level::Info)
    );

    log::info!("Hello from Rust Android SO!");
}
