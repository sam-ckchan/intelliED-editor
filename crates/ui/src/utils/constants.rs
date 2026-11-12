use gpui::Pixels;
use gpui::px;

/// Hairline between panels (tool windows, split editors, the stripes): the
/// window background shows through it as the separator.
pub const PANEL_GAP: Pixels = px(1.);

/// Height of the main toolbar, the window's top row.
pub const MAIN_TOOLBAR_HEIGHT: Pixels = px(MAIN_TOOLBAR_HEIGHT_PX);
const MAIN_TOOLBAR_HEIGHT_PX: f32 = 40.;

/// The macOS traffic lights are 14px, 6px apart, and don't scale with the UI.
const TRAFFIC_LIGHT_SIZE_PX: f32 = 14.;
const TRAFFIC_LIGHT_SPACING_PX: f32 = 6.;

// Use pixels here instead of a rem-based size because the macOS traffic
// lights are a static size, and don't scale with the rest of the UI.
//
/// Where the macOS traffic lights sit: on the main toolbar's centre line, as
/// far from the left edge as from the top.
pub const TRAFFIC_LIGHT_POSITION: (f32, f32) = {
    let inset = MAIN_TOOLBAR_HEIGHT_PX / 2. - TRAFFIC_LIGHT_SIZE_PX / 2.;
    (inset, inset)
};

/// Room after the zoom button, before the main toolbar's first control.
const TRAFFIC_LIGHT_MARGIN_PX: f32 = 12.;

/// Room from the window's left edge to the end of the three traffic lights
/// plus a margin.
pub const TRAFFIC_LIGHT_PADDING: f32 =
    TRAFFIC_LIGHT_POSITION.0 + TRAFFIC_LIGHT_SIZE_PX * 3. + TRAFFIC_LIGHT_SPACING_PX * 2. + TRAFFIC_LIGHT_MARGIN_PX;

#[cfg(test)]
mod tests {
    use super::MAIN_TOOLBAR_HEIGHT_PX;
    use super::TRAFFIC_LIGHT_PADDING;
    use super::TRAFFIC_LIGHT_POSITION;
    use super::TRAFFIC_LIGHT_SIZE_PX;

    #[test]
    fn test_traffic_lights_on_main_toolbar_centre_line() {
        assert_eq!(TRAFFIC_LIGHT_POSITION, (13., 13.));
        assert_eq!(TRAFFIC_LIGHT_POSITION.1 + TRAFFIC_LIGHT_SIZE_PX / 2., MAIN_TOOLBAR_HEIGHT_PX / 2.);
    }

    #[test]
    fn test_traffic_light_padding_fits_three_buttons() {
        assert_eq!(TRAFFIC_LIGHT_PADDING, 79.);
    }
}
