//! IntelliJ's double-Shift: tapping Shift twice in quick succession opens
//! Search Everywhere. Keymaps can't express a modifier tap, so the workspace
//! watches modifier changes itself.

use std::time::Duration;
use std::time::Instant;

use gpui::Modifiers;

/// How soon the second tap must follow the first.
const DOUBLE_TAP_INTERVAL: Duration = Duration::from_millis(400);

/// Recognises two Shift taps: Shift pressed and released on its own, twice,
/// with no other key, modifier or mouse press in between.
#[derive(Default)]
pub(crate) struct DoubleShift {
    /// Shift is down and nothing else has happened since it went down.
    tapping: bool,
    last_tap: Option<Instant>,
}

impl DoubleShift {
    /// Feeds a modifier change; returns whether it completes a double tap.
    pub(crate) fn modifiers_changed(&mut self, modifiers: Modifiers, now: Instant) -> bool {
        let shift_only = modifiers.shift && modifiers.number_of_modifiers() == 1;
        if shift_only {
            self.tapping = true;
            return false;
        }
        if modifiers.modified() {
            // Another modifier joined in: a shortcut, not a tap.
            self.interrupt();
            return false;
        }
        if !std::mem::take(&mut self.tapping) {
            return false;
        }
        let is_double = self.last_tap.is_some_and(|last| now.duration_since(last) <= DOUBLE_TAP_INTERVAL);
        self.last_tap = if is_double { None } else { Some(now) };
        is_double
    }

    /// A key or mouse press: Shift was held for typing or clicking, not tapped.
    pub(crate) const fn interrupt(&mut self) {
        self.tapping = false;
        self.last_tap = None;
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use std::time::Instant;

    use gpui::Modifiers;

    use super::DOUBLE_TAP_INTERVAL;
    use super::DoubleShift;

    fn shift() -> Modifiers {
        Modifiers { shift: true, ..Modifiers::none() }
    }

    fn tap(detector: &mut DoubleShift, at: Instant) -> bool {
        assert!(!detector.modifiers_changed(shift(), at));
        detector.modifiers_changed(Modifiers::none(), at)
    }

    #[test]
    fn two_quick_taps_fire_once() {
        let (mut detector, start) = (DoubleShift::default(), Instant::now());
        assert!(!tap(&mut detector, start));
        assert!(tap(&mut detector, start + Duration::from_millis(150)));
        // The pair is consumed: a third tap starts over.
        assert!(!tap(&mut detector, start + Duration::from_millis(300)));
    }

    #[test]
    fn slow_taps_do_not_fire() {
        let (mut detector, start) = (DoubleShift::default(), Instant::now());
        assert!(!tap(&mut detector, start));
        let late = start + DOUBLE_TAP_INTERVAL + Duration::from_millis(1);
        assert!(!tap(&mut detector, late));
        // The late tap counts as a new first tap.
        assert!(tap(&mut detector, late + Duration::from_millis(100)));
    }

    #[test]
    fn typing_with_shift_is_not_a_tap() {
        let (mut detector, start) = (DoubleShift::default(), Instant::now());
        assert!(!detector.modifiers_changed(shift(), start));
        detector.interrupt(); // a key went down while Shift was held
        assert!(!detector.modifiers_changed(Modifiers::none(), start));
        assert!(!tap(&mut detector, start + Duration::from_millis(100)));
    }

    #[test]
    fn a_key_between_taps_cancels() {
        let (mut detector, start) = (DoubleShift::default(), Instant::now());
        assert!(!tap(&mut detector, start));
        detector.interrupt();
        assert!(!tap(&mut detector, start + Duration::from_millis(100)));
    }

    #[test]
    fn other_modifiers_cancel() {
        let (mut detector, start) = (DoubleShift::default(), Instant::now());
        assert!(!tap(&mut detector, start));
        assert!(!detector.modifiers_changed(shift(), start));
        assert!(!detector.modifiers_changed(Modifiers { platform: true, ..shift() }, start));
        assert!(!detector.modifiers_changed(Modifiers::none(), start));
        assert!(!tap(&mut detector, start + Duration::from_millis(100)));
    }
}
