use std::panic;
use std::sync::{Arc, Mutex};

use assert4rs::Assert;

#[test]
fn panic_location_points_to_call_site_not_library_internals() {
    let captured: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    let captured_clone = captured.clone();

    let previous_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        if let Some(location) = info.location() {
            *captured_clone.lock().unwrap() = Some(location.file().to_string());
        }
    }));

    let result = panic::catch_unwind(|| {
        Assert::that(1).is(2);
    });

    panic::set_hook(previous_hook);

    assert!(result.is_err());
    let file = captured
        .lock()
        .unwrap()
        .clone()
        .expect("panic location was captured");
    assert!(
        file.ends_with("track_caller.rs"),
        "expected panic location in this test file, got: {file}"
    );
}
