use gpui::WindowAppearance;
use objc2::msg_send;

use crate::NSStringExt;
use crate::ns::id;

pub(crate) unsafe fn window_appearance_from_native(appearance: id) -> WindowAppearance {
    let name: id = msg_send![appearance, name];
    unsafe {
        if name == NSAppearanceNameVibrantLight {
            WindowAppearance::VibrantLight
        } else if name == NSAppearanceNameVibrantDark {
            WindowAppearance::VibrantDark
        } else if name == NSAppearanceNameAqua {
            WindowAppearance::Light
        } else if name == NSAppearanceNameDarkAqua {
            WindowAppearance::Dark
        } else {
            appearance_from_name(name.to_str())
        }
    }
}

/// Other appearances, such as the "Increase contrast" ones
/// (`NSAppearanceNameAccessibilityHighContrastDarkAqua`), by their name.
fn appearance_from_name(name: &str) -> WindowAppearance {
    if name.contains("Dark") {
        return WindowAppearance::Dark;
    }
    if !name.contains("Aqua") && !name.contains("Light") {
        log::warn!("unknown appearance: {name:?}");
    }
    WindowAppearance::Light
}

#[link(name = "AppKit", kind = "framework")]
unsafe extern "C" {
    pub static NSAppearanceNameAqua: id;
    pub static NSAppearanceNameDarkAqua: id;
    pub static NSAppearanceNameVibrantLight: id;
    pub static NSAppearanceNameVibrantDark: id;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_appearance_from_name_handles_high_contrast() {
        assert_eq!(appearance_from_name("NSAppearanceNameAccessibilityHighContrastDarkAqua"), WindowAppearance::Dark);
        assert_eq!(appearance_from_name("NSAppearanceNameAccessibilityHighContrastAqua"), WindowAppearance::Light);
        assert_eq!(
            appearance_from_name("NSAppearanceNameAccessibilityHighContrastVibrantDark"),
            WindowAppearance::Dark
        );
        assert_eq!(appearance_from_name("unknown"), WindowAppearance::Light);
    }
}
