use crate::{detect_os, Platform};

#[test]
fn test_detect_os() {
    let os = detect_os();
    // We can't assert a specific value since it depends on the test environment,
    // but we can ensure it's not Unknown for common platforms
    if cfg!(target_os = "windows") {
        assert_eq!(os, Platform::Windows);
    } else if cfg!(target_os = "macos") {
        assert_eq!(os, Platform::MacOS);
    } else if cfg!(target_os = "linux") {
        assert_eq!(os, Platform::Linux);
    }
}

#[test]
fn test_platform_equality() {
    assert_eq!(Platform::Windows, Platform::Windows);
    assert_eq!(Platform::MacOS, Platform::MacOS);
    assert_eq!(Platform::Linux, Platform::Linux);
    assert_eq!(Platform::Unknown, Platform::Unknown);

    assert_ne!(Platform::Windows, Platform::MacOS);
    assert_ne!(Platform::Windows, Platform::Linux);
    assert_ne!(Platform::Windows, Platform::Unknown);
    assert_ne!(Platform::MacOS, Platform::Linux);
    assert_ne!(Platform::MacOS, Platform::Unknown);
    assert_ne!(Platform::Linux, Platform::Unknown);
}
