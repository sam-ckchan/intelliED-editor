use std::sync::LazyLock;

use gpui::FontStyle;
use gpui::FontWeight;
use gpui::HighlightStyle;
use gpui::Hsla;
use gpui::SharedString;
use gpui::WindowBackgroundAppearance;
use gpui::rgb;
use gpui::transparent_black;
use syntax_theme::SyntaxTheme;

use crate::AccentColors;
use crate::Appearance;
use crate::CursorColors;
use crate::StatusColors;
use crate::SystemColors;
use crate::Theme;
use crate::ThemeColors;
use crate::ThemeStyles;

/// Buffer and terminal line height, relative to the font size.
pub const BUFFER_LINE_HEIGHT: f32 = 1.618;
/// Opacity reduction applied to unnecessary (e.g. unused) code.
pub const UNNECESSARY_CODE_FADE: f32 = 0.3;

/// The only theme: IntelliJ's New UI "Dark", flat panels divided by hairlines. Edit these constructors to change the look.
pub fn theme() -> &'static Theme {
    static THEME: LazyLock<Theme> = LazyLock::new(|| Theme {
        id: "intellij-dark".into(),
        name: SharedString::new_static("IntelliJ Dark"),
        appearance: Appearance::Dark,
        styles: ThemeStyles {
            window_background_appearance: WindowBackgroundAppearance::Opaque,
            system: system_colors(),
            accents: accent_colors(),
            colors: theme_colors(),
            status: status_colors(),
            cursor_colors: cursor_colors(),
            syntax: syntax_theme(),
        },
    });
    &THEME
}

// IntelliJ New UI Dark palette: the only raw colour values of the theme. Everything below derives from these.
const GRAY1: u32 = 0x1e1f22;
const GRAY2: u32 = 0x2b2d30;
const GRAY3: u32 = 0x393b40;
const GRAY4: u32 = 0x43454a;
const GRAY5: u32 = 0x4e5157;
const GRAY7: u32 = 0x6f737a;
const GRAY8: u32 = 0x868a91;
const GRAY9: u32 = 0x9da0a8;
const GRAY12: u32 = 0xdfe1e5;
const GRAY14: u32 = 0xffffff;
const BLUE2: u32 = 0x2e436e;
const BLUE6: u32 = 0x3574f0;
const BLUE8: u32 = 0x548af7;
const GREEN7: u32 = 0x73bd79;
const RED6: u32 = 0xdb5c5c;
const YELLOW7: u32 = 0xf2c55c;
const ORANGE: u32 = 0xe08855;
const PURPLE: u32 = 0xa571e6;
const TEAL: u32 = 0x24a394;
const OLIVE: u32 = 0x848504;
const FILE_BLUE: u32 = 0x70aeff;
// The "Dark" editor colour scheme.
const CODE_TEXT: u32 = 0xbcbec4;
const CODE_KEYWORD: u32 = 0xcf8e6d;
const CODE_STRING: u32 = 0x6aab73;
const CODE_NUMBER: u32 = 0x2aacb8;
const CODE_COMMENT: u32 = 0x7a7e85;
const CODE_DOC: u32 = 0x5f826b;
const CODE_FUNCTION: u32 = 0x56a8f5;
const CODE_CONSTANT: u32 = 0xc77dbb;
const CODE_METADATA: u32 = 0xb3ae60;
const CODE_TYPE_PARAMETER: u32 = 0x16baac;
const CODE_TAG: u32 = 0xd5b778;
const CODE_REGEX: u32 = 0x42c3d4;
const CARET_ROW: u32 = 0x26282e;
const CODE_SELECTION: u32 = 0x214283;
const LINE_NUMBER: u32 = 0x4b5059;
const CARET_LINE_NUMBER: u32 = 0xa1a3ab;
const INDENT_GUIDE: u32 = 0x313438;
const SEARCH_MATCH: u32 = 0x2d543f;
const IDENTIFIER_UNDER_CARET: u32 = 0x373b39;
const IDENTIFIER_UNDER_CARET_WRITE: u32 = 0x402f33;
// The terminal's ANSI colours.
const ANSI_BLACK: u32 = 0x000000;
const ANSI_RED: u32 = 0xf0524f;
const ANSI_GREEN: u32 = 0x5c962c;
const ANSI_YELLOW: u32 = 0xa68a0d;
const ANSI_BLUE: u32 = 0x3993d4;
const ANSI_MAGENTA: u32 = 0xa771bf;
const ANSI_CYAN: u32 = 0x00a3a3;
const ANSI_WHITE: u32 = 0x808080;

// Roles: what each palette colour (or a formula of it) is used for.
/// Tool windows, the main toolbar, the stripes and the status bar.
const CHROME: u32 = GRAY2;
/// Editors, their tab bars and terminals. Also the hairlines between panels: the window
/// background shows through the 1px gaps.
const EDITOR: u32 = GRAY1;
const POPUP: u32 = GRAY2;
const BORDER: u32 = GRAY3;
const POPUP_BORDER: u32 = GRAY4;
const HOVER: u32 = GRAY3;
const PRESSED: u32 = GRAY4;
/// The selected row of a list or tree.
const ROW_SELECTION: u32 = BLUE2;
const FAINT: u32 = GRAY5;
const TEXT: u32 = GRAY12;
const TEXT_MUTED: u32 = GRAY9;
const TEXT_DIM: u32 = GRAY7;
const CODE_HINT: u32 = GRAY8;
/// Fills: the active tab's underline, focus rings, the open tool window's stripe button.
const ACCENT_FILL: u32 = BLUE6;
/// Accent text, icons and links, lighter than the fill to stay readable on dark surfaces.
const ACCENT: u32 = BLUE8;
const GREEN: u32 = GREEN7;
const RED: u32 = RED6;
const YELLOW: u32 = YELLOW7;

/// Terminal bright / dim variants of an ANSI colour.
const BRIGHT: i32 = 20;
const DIM: i32 = -30;

pub fn theme_colors() -> ThemeColors {
    let transparent = transparent_black();
    let scrollbar = color(GRAY14).alpha(0.15);
    let scrollbar_hover = color(GRAY14).alpha(0.25);
    let indent_guide = color(INDENT_GUIDE);
    let indent_guide_active = color(FAINT);
    ThemeColors {
        border: color(BORDER),
        border_variant: color(BORDER),
        border_focused: color(ACCENT_FILL),
        border_selected: color(ACCENT_FILL),
        border_transparent: transparent,
        border_disabled: color(BORDER),
        elevated_surface_background: color(POPUP),
        surface_background: color(CHROME),
        background: color(EDITOR),
        element_background: color(HOVER),
        element_hover: color(PRESSED),
        element_active: color(FAINT),
        element_selected: color(PRESSED),
        element_selection_background: color(CODE_SELECTION),
        element_disabled: color(HOVER),
        ghost_element_background: transparent,
        ghost_element_hover: color(HOVER),
        ghost_element_active: color(PRESSED),
        ghost_element_selected: color(ROW_SELECTION),
        ghost_element_disabled: transparent,
        text: color(TEXT),
        text_muted: color(TEXT_MUTED),
        text_placeholder: color(TEXT_DIM),
        text_disabled: color(TEXT_DIM),
        text_accent: color(ACCENT),
        icon: color(TEXT),
        icon_muted: color(TEXT_MUTED),
        icon_disabled: color(TEXT_DIM),
        icon_placeholder: color(TEXT_DIM),
        icon_accent: color(ACCENT),
        status_bar_background: color(CHROME),
        title_bar_background: color(CHROME),
        title_bar_inactive_background: color(CHROME),
        toolbar_background: color(EDITOR),
        tab_bar_background: color(EDITOR),
        tab_inactive_background: color(EDITOR),
        tab_active_background: color(EDITOR),
        search_match_background: color(SEARCH_MATCH),
        search_active_match_background: color(shade(SEARCH_MATCH, 40)),
        panel_background: color(CHROME),
        panel_focused_border: color(ACCENT_FILL),
        panel_indent_guide: color(BORDER),
        panel_indent_guide_hover: color(FAINT),
        panel_indent_guide_active: color(FAINT),
        panel_overlay_background: color(CHROME),
        panel_overlay_hover: color(HOVER),
        pane_focused_border: color(ACCENT_FILL),
        pane_group_border: color(EDITOR),
        scrollbar_thumb_background: scrollbar,
        scrollbar_thumb_hover_background: scrollbar_hover,
        scrollbar_thumb_active_background: scrollbar_hover,
        scrollbar_thumb_border: transparent,
        scrollbar_track_background: transparent,
        scrollbar_track_border: transparent,
        minimap_thumb_background: scrollbar,
        minimap_thumb_hover_background: scrollbar_hover,
        minimap_thumb_active_background: scrollbar_hover,
        minimap_thumb_border: transparent,
        editor_foreground: color(CODE_TEXT),
        editor_code_lens_foreground: None,
        editor_background: color(EDITOR),
        editor_gutter_background: color(EDITOR),
        editor_subheader_background: color(CHROME),
        editor_active_line_background: color(CARET_ROW),
        editor_highlighted_line_background: color(CARET_ROW),
        editor_line_number: color(LINE_NUMBER),
        editor_active_line_number: color(CARET_LINE_NUMBER),
        editor_hover_line_number: color(CARET_LINE_NUMBER),
        editor_invisible: color(TEXT_DIM),
        editor_wrap_guide: color(BORDER),
        editor_active_wrap_guide: color(BORDER),
        editor_indent_guide: indent_guide,
        editor_indent_guide_active: indent_guide_active,
        editor_document_highlight_read_background: color(IDENTIFIER_UNDER_CARET),
        editor_document_highlight_write_background: color(IDENTIFIER_UNDER_CARET_WRITE),
        editor_document_highlight_bracket_background: color(PRESSED),
        editor_diff_hunk_added_background: color(GREEN).alpha(0.12),
        editor_diff_hunk_added_hollow_background: color(GREEN).alpha(0.06),
        editor_diff_hunk_added_hollow_border: color(GREEN).alpha(0.36),
        editor_diff_hunk_deleted_background: color(RED).alpha(0.12),
        editor_diff_hunk_deleted_hollow_background: color(RED).alpha(0.06),
        editor_diff_hunk_deleted_hollow_border: color(RED).alpha(0.36),
        terminal_background: color(EDITOR),
        terminal_foreground: color(CODE_TEXT),
        terminal_bright_foreground: color(TEXT),
        terminal_dim_foreground: color(shade(CODE_TEXT, DIM)),
        terminal_ansi_background: color(EDITOR),
        terminal_ansi_black: color(ANSI_BLACK),
        terminal_ansi_bright_black: color(FAINT),
        terminal_ansi_dim_black: color(ANSI_BLACK),
        terminal_ansi_red: color(ANSI_RED),
        terminal_ansi_bright_red: color(shade(ANSI_RED, BRIGHT)),
        terminal_ansi_dim_red: color(shade(ANSI_RED, DIM)),
        terminal_ansi_green: color(ANSI_GREEN),
        terminal_ansi_bright_green: color(shade(ANSI_GREEN, BRIGHT)),
        terminal_ansi_dim_green: color(shade(ANSI_GREEN, DIM)),
        terminal_ansi_yellow: color(ANSI_YELLOW),
        terminal_ansi_bright_yellow: color(shade(ANSI_YELLOW, BRIGHT)),
        terminal_ansi_dim_yellow: color(shade(ANSI_YELLOW, DIM)),
        terminal_ansi_blue: color(ANSI_BLUE),
        terminal_ansi_bright_blue: color(shade(ANSI_BLUE, BRIGHT)),
        terminal_ansi_dim_blue: color(shade(ANSI_BLUE, DIM)),
        terminal_ansi_magenta: color(ANSI_MAGENTA),
        terminal_ansi_bright_magenta: color(shade(ANSI_MAGENTA, BRIGHT)),
        terminal_ansi_dim_magenta: color(shade(ANSI_MAGENTA, DIM)),
        terminal_ansi_cyan: color(ANSI_CYAN),
        terminal_ansi_bright_cyan: color(shade(ANSI_CYAN, BRIGHT)),
        terminal_ansi_dim_cyan: color(shade(ANSI_CYAN, DIM)),
        terminal_ansi_white: color(ANSI_WHITE),
        terminal_ansi_bright_white: color(GRAY14),
        terminal_ansi_dim_white: color(shade(ANSI_WHITE, DIM)),
        link_text_hover: color(ACCENT),
        version_control_added: color(GREEN),
        version_control_deleted: color(RED),
        version_control_modified: color(FILE_BLUE),
        version_control_renamed: color(FILE_BLUE),
        version_control_conflict: color(RED),
        version_control_ignored: color(OLIVE),
        version_control_word_added: color(shade(GREEN, -50)),
        version_control_word_deleted: color(shade(RED, -50)),
        version_control_conflict_marker_ours: color(GREEN).alpha(0.125),
        version_control_conflict_marker_theirs: color(RED).alpha(0.125),
    }
}

pub fn status_colors() -> StatusColors {
    StatusColors {
        conflict: color(RED),
        created: color(GREEN),
        created_background: color(GREEN).alpha(0.25),
        deleted: color(RED),
        deleted_background: color(RED).alpha(0.25),
        error: color(RED),
        error_background: color(RED).alpha(0.2),
        error_border: color(RED),
        hidden: color(TEXT_DIM),
        hint: color(CODE_HINT),
        hint_background: color(POPUP),
        hint_border: color(POPUP_BORDER),
        ignored: color(OLIVE),
        ignored_background: color(POPUP),
        ignored_border: color(POPUP_BORDER),
        info: color(TEXT_MUTED),
        info_background: color(POPUP),
        info_border: color(POPUP_BORDER),
        modified: color(FILE_BLUE),
        renamed: color(FILE_BLUE),
        success: color(GREEN),
        success_background: color(POPUP),
        success_border: color(POPUP_BORDER),
        unreachable: color(TEXT_MUTED),
        warning: color(YELLOW),
        warning_background: color(YELLOW).alpha(0.2),
        warning_border: color(YELLOW),
    }
}

/// The local cursor and selection.
pub fn cursor_colors() -> CursorColors {
    CursorColors { cursor: color(CODE_TEXT), selection: color(CODE_SELECTION) }
}

pub fn accent_colors() -> AccentColors {
    AccentColors([ACCENT, TEAL, RED, ORANGE, YELLOW, GREEN, PURPLE].map(color))
}

pub fn system_colors() -> SystemColors {
    SystemColors {
        transparent: transparent_black(),
        mac_os_traffic_light_red: color(0xec6b5f),
        mac_os_traffic_light_yellow: color(0xf4bf4e),
        mac_os_traffic_light_green: color(0x62c454),
    }
}

/// IntelliJ colours declarations, not uses: types, variables and operators stay plain text.
pub fn syntax_theme() -> SyntaxTheme {
    let italic = |hex| HighlightStyle { font_style: Some(FontStyle::Italic), ..fg(hex) };
    let bold = |hex| HighlightStyle { font_weight: Some(FontWeight::BOLD), ..fg(hex) };
    SyntaxTheme::new(
        [
            ("attribute", fg(CODE_METADATA)),
            ("boolean", fg(CODE_KEYWORD)),
            ("comment", fg(CODE_COMMENT)),
            ("comment.doc", fg(CODE_DOC)),
            ("constant", fg(CODE_CONSTANT)),
            ("constructor", fg(CODE_TEXT)),
            ("definition.function", fg(CODE_FUNCTION)),
            ("embedded", fg(CODE_TEXT)),
            ("emphasis", italic(CODE_TEXT)),
            ("emphasis.strong", bold(CODE_TEXT)),
            ("enum", fg(CODE_TEXT)),
            ("function", fg(CODE_FUNCTION)),
            ("function.method", fg(CODE_FUNCTION)),
            ("hint", fg(CODE_HINT)),
            ("keyword", fg(CODE_KEYWORD)),
            ("label", fg(CODE_TYPE_PARAMETER)),
            ("lifetime", fg(CODE_TYPE_PARAMETER)),
            ("link_text", HighlightStyle { font_style: Some(FontStyle::Normal), ..fg(ACCENT) }),
            ("link_uri", fg(ACCENT)),
            ("number", fg(CODE_NUMBER)),
            ("operator", fg(CODE_TEXT)),
            ("predictive", fg(TEXT_DIM)),
            ("preproc", fg(CODE_METADATA)),
            ("primary", fg(CODE_TEXT)),
            ("property", fg(CODE_CONSTANT)),
            ("punctuation", fg(CODE_TEXT)),
            ("punctuation.bracket", fg(CODE_TEXT)),
            ("punctuation.delimiter", fg(CODE_TEXT)),
            ("punctuation.list_marker", fg(CODE_KEYWORD)),
            ("punctuation.special", fg(CODE_KEYWORD)),
            ("string", fg(CODE_STRING)),
            ("string.escape", fg(CODE_KEYWORD)),
            ("string.regex", fg(CODE_REGEX)),
            ("string.special", fg(CODE_STRING)),
            ("tag", fg(CODE_TAG)),
            ("text.literal", fg(CODE_STRING)),
            ("title", bold(CODE_CONSTANT)),
            ("type", fg(CODE_TEXT)),
            ("type.builtin", fg(CODE_KEYWORD)),
            ("type.super", fg(CODE_TEXT)),
            ("variable", fg(CODE_TEXT)),
            ("variable.parameter", fg(CODE_TEXT)),
            ("variable.special", fg(CODE_KEYWORD)),
            ("variant", italic(CODE_CONSTANT)),
        ]
        .map(|(name, style): (&str, HighlightStyle)| (name.to_string(), style)),
    )
}

fn color(hex: u32) -> Hsla {
    rgb(hex).into()
}

fn fg(hex: u32) -> HighlightStyle {
    HighlightStyle { color: Some(color(hex)), ..HighlightStyle::default() }
}

/// Scales each RGB channel by `100 + percent` percent, clamped: negative darkens, positive lightens.
const fn shade(hex: u32, percent: i32) -> u32 {
    let scale = (100 + percent) as u32;
    let mut out = 0;
    let mut shift = 0;
    while shift < 24 {
        let channel = ((hex >> shift) & 0xff) * scale / 100;
        out |= (if channel > 0xff { 0xff } else { channel }) << shift;
        shift += 8;
    }
    out
}

#[cfg(test)]
mod tests {
    use gpui::FontWeight;
    use gpui::Rgba;

    use super::*;

    #[test]
    fn test_color_parses_rgb_hex() {
        for hex in [0x3574f0, 0x2b2d30, 0x000000] {
            let rgba = Rgba::from(color(hex));
            let channels = [rgba.r, rgba.g, rgba.b].map(|channel| (channel * 255.).round() as u32);
            assert_eq!(channels[0] << 16 | channels[1] << 8 | channels[2], hex);
            assert_eq!(rgba.a, 1.);
        }
    }

    #[test]
    fn test_fg_sets_only_color() {
        assert_eq!(fg(RED), HighlightStyle { color: Some(color(RED)), ..HighlightStyle::default() });
    }

    #[test]
    fn test_shade_scales_and_clamps_channels() {
        assert_eq!(shade(GRAY2, 0), GRAY2);
        assert_eq!(shade(0x808080, -50), 0x404040);
        assert_eq!(shade(0xff8040, 20), 0xff994c);
        assert_eq!(shade(GRAY12, -100), 0x000000);
    }

    #[test]
    fn test_roles_use_intellij_surfaces() {
        assert_eq!(CHROME, 0x2b2d30);
        assert_eq!(EDITOR, 0x1e1f22);
        assert_eq!(ACCENT_FILL, 0x3574f0);
        assert_eq!(ROW_SELECTION, 0x2e436e);
    }

    #[test]
    fn test_theme_is_flat_intellij_dark() {
        let theme = theme();
        assert_eq!(theme.name.as_ref(), "IntelliJ Dark");
        assert_eq!(theme.appearance, Appearance::Dark);

        let colors = theme.colors();
        // Tool windows, the main toolbar and the status bar share the chrome.
        assert_eq!(colors.title_bar_background, colors.panel_background);
        assert_eq!(colors.status_bar_background, colors.panel_background);
        // Editors, their tab bars and the hairlines between panels share the editor colour.
        assert_eq!(colors.tab_bar_background, colors.editor_background);
        assert_eq!(colors.tab_active_background, colors.editor_background);
        assert_eq!(colors.background, colors.editor_background);
        assert_eq!(colors.terminal_ansi_background, colors.terminal_background);
        assert!(colors.editor_background.l < colors.panel_background.l);
        assert!(colors.panel_overlay_hover.is_opaque());
        assert_eq!(colors.element_selection_background, theme.cursor_colors().selection);
    }

    #[test]
    fn test_terminal_dim_and_bright_bracket_normal() {
        let colors = theme_colors();
        for (dim, normal, bright) in [
            (colors.terminal_ansi_dim_red, colors.terminal_ansi_red, colors.terminal_ansi_bright_red),
            (colors.terminal_ansi_dim_green, colors.terminal_ansi_green, colors.terminal_ansi_bright_green),
            (colors.terminal_ansi_dim_blue, colors.terminal_ansi_blue, colors.terminal_ansi_bright_blue),
        ] {
            assert!(dim.l < normal.l && normal.l < bright.l);
        }
    }

    #[test]
    fn test_status_colors_use_popup_surfaces() {
        let status = status_colors();
        assert_eq!(status.ignored_background, status.hint_background);
        assert_eq!(status.ignored_border, status.hint_border);
        assert_eq!(status.error_background, color(RED).alpha(0.2));
        assert_eq!(status.modified, theme_colors().version_control_modified);
    }

    #[test]
    fn test_syntax_theme_resolves_capture_names() {
        let theme = theme();
        let syntax = theme.syntax();
        assert_eq!(syntax.style_for_name("keyword").and_then(|style| style.color), Some(color(CODE_KEYWORD)));
        assert_eq!(syntax.style_for_name("string").and_then(|style| style.color), Some(color(CODE_STRING)));
        assert_eq!(syntax.style_for_name("function").and_then(|style| style.color), Some(color(CODE_FUNCTION)));
        // Uses stay plain text; only declarations and literals are coloured.
        assert_eq!(syntax.style_for_name("type").and_then(|style| style.color), Some(color(CODE_TEXT)));
        assert_eq!(syntax.style_for_name("title").and_then(|style| style.font_weight), Some(FontWeight::BOLD));
        assert_eq!(syntax.style_for_name("variant").and_then(|style| style.font_style), Some(FontStyle::Italic));
        assert!(syntax.highlight_id("string.escape").is_some());
        assert!(syntax.style_for_name("missing").is_none());
    }

    #[test]
    fn test_theme_is_one_static_instance() {
        assert!(std::ptr::eq(theme(), theme()));
        assert_eq!(theme().syntax(), &syntax_theme());
    }

    #[test]
    fn test_accent_colors_cycle() {
        let accents = theme().accents();
        assert_eq!(accents.color_for_index(0), color(ACCENT));
        assert_eq!(accents.color_for_index(accents.0.len() as u32), accents.color_for_index(0));
    }
}
