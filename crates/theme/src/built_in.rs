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

/// The only theme: IntelliJ's "Islands Dark" (PyCharm 2026.1, as in OpenIDEA): rounded islands on a
/// darker frame. Edit these constructors to change the look.
pub fn theme() -> &'static Theme {
    static THEME: LazyLock<Theme> = LazyLock::new(|| Theme {
        id: "islands-dark".into(),
        name: SharedString::new_static("Islands Dark"),
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

// Islands Dark palette (OpenIDEA's design tokens): the only raw colour values of the theme.
const FRAME_GRAY: u32 = 0x26282c;
const ISLAND_GRAY: u32 = 0x191a1c;
const HOVER_GRAY: u32 = 0x2e2f30;
const POPUP_BORDER_GRAY: u32 = 0x33353b;
const STRONG_BORDER_GRAY: u32 = 0x40434a;
const FG_STRONG: u32 = 0xdfe1e5;
const FG_SECONDARY: u32 = 0x9fa2a8;
const FG_MUTED: u32 = 0x8c8f95;
const FG_SUBTLE: u32 = 0x73767c;
const FG_FAINT: u32 = 0x5f6269;
const FG_ICON: u32 = 0xced0d6;
const WHITE: u32 = 0xffffff;
const ACCENT_BLUE: u32 = 0x3871e1;
const LINK_BLUE: u32 = 0x71a1fe;
const INFO_BLUE: u32 = 0x548af7;
const TAB_SELECTED_BLUE: u32 = 0x233558;
const SELECTED_BORDER_BLUE: u32 = 0x2e4d89;
const TREE_SELECTED_BLUE: u32 = 0x2a4371;
const DANGER: u32 = 0xf57e84;
const WARNING: u32 = 0xd59637;
const SUCCESS: u32 = 0x6db083;
const VCS_MODIFIED: u32 = 0x70aeff;
const VCS_ADDED: u32 = 0x73bd79;
const VCS_DELETED: u32 = 0x6f737a;
const VCS_IGNORED: u32 = 0xd69a6b;
const VCS_CONFLICT: u32 = 0xde6a66;
const ORANGE: u32 = 0xe08855;
const PURPLE: u32 = 0xa571e6;
const TEAL: u32 = 0x24a394;
// The Islands Dark editor colour scheme.
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
const CODE_HINT: u32 = 0x868a91;
const CARET: u32 = 0xced0d6;
const CARET_ROW: u32 = 0x1f2024;
const CODE_SELECTION: u32 = 0x214283;
const LINE_NUMBER: u32 = 0x4b5059;
const CARET_LINE_NUMBER: u32 = 0xa1a3ab;
const INDENT_GUIDE: u32 = 0x323438;
const INDENT_GUIDE_ACTIVE: u32 = 0x4e5157;
const SEARCH_RESULT: u32 = 0x114957;
const SEARCH_RESULT_CURRENT: u32 = 0x165e70;
const MATCHED_BRACE: u32 = 0x43454a;
const IDENTIFIER_UNDER_CARET: u32 = 0x373b39;
const IDENTIFIER_UNDER_CARET_WRITE: u32 = 0x402f33;
// The terminal's ANSI colours.
const ANSI_BLACK: u32 = 0x000000;
const ANSI_RED: u32 = 0xf27481;
const ANSI_GREEN: u32 = 0x6bcc62;
const ANSI_YELLOW: u32 = 0xe0ce70;
const ANSI_BLUE: u32 = 0x5594fa;
const ANSI_MAGENTA: u32 = 0xc092fa;
const ANSI_CYAN: u32 = 0x47ccbd;
const ANSI_WHITE: u32 = 0xced0d6;
const ANSI_BRIGHT_BLACK: u32 = 0x4e5157;
const ANSI_BRIGHT_RED: u32 = 0xff6b7a;
const ANSI_BRIGHT_GREEN: u32 = 0x67ff59;
const ANSI_BRIGHT_YELLOW: u32 = 0xffec1a;
const ANSI_BRIGHT_BLUE: u32 = 0x3399ff;
const ANSI_BRIGHT_MAGENTA: u32 = 0xd970ff;
const ANSI_BRIGHT_CYAN: u32 = 0x40ffe9;

/// Project colours (IntelliJ's RecentProject.Color1–9): each project gets one pair, start and end
/// of its gradient, chosen from its name.
const PROJECT_COLORS: [(u32, u32); 9] = [
    (0xe08855, 0xe9806f),
    (0xb08b14, 0xbb7f19),
    (0xa1a359, 0x87aa59),
    (0x3b92b8, 0x6183ec),
    (0x3574f0, 0x7a64f0),
    (0xc84d8f, 0xa956cf),
    (0x955ae0, 0xa84de0),
    (0x24a394, 0x279ccd),
    (0x5fad65, 0x3d968b),
];

// Roles: what each palette colour is used for.
/// The window frame: around and between the islands, the main toolbar, the stripes, the status bar.
const FRAME: u32 = FRAME_GRAY;
/// Islands: editors (with their tabs), tool windows, terminals.
const ISLAND: u32 = ISLAND_GRAY;
const POPUP: u32 = FRAME_GRAY;
const POPUP_BORDER: u32 = POPUP_BORDER_GRAY;
/// Lines inside an island.
const BORDER: u32 = FRAME_GRAY;
const HOVER: u32 = HOVER_GRAY;
/// The selected row of a list or tree.
const ROW_SELECTION: u32 = TREE_SELECTED_BLUE;
const TEXT: u32 = FG_STRONG;
const TEXT_MUTED: u32 = FG_SECONDARY;
const TEXT_DIM: u32 = FG_SUBTLE;
/// Fills: focus borders, primary buttons.
const ACCENT_FILL: u32 = ACCENT_BLUE;
/// Accent text, icons and links, lighter than the fill to stay readable on dark surfaces.
const ACCENT: u32 = LINK_BLUE;
const GREEN: u32 = VCS_ADDED;
const RED: u32 = DANGER;
const YELLOW: u32 = WARNING;

/// Terminal dim variants of an ANSI colour.
const DIM: i32 = -30;

pub fn theme_colors() -> ThemeColors {
    let transparent = transparent_black();
    let scrollbar = color(WHITE).alpha(0.2);
    let scrollbar_hover = color(WHITE).alpha(0.35);
    ThemeColors {
        border: color(BORDER),
        border_variant: color(BORDER),
        border_focused: color(ACCENT_FILL),
        border_selected: color(SELECTED_BORDER_BLUE),
        border_transparent: transparent,
        border_disabled: color(STRONG_BORDER_GRAY),
        elevated_surface_background: color(POPUP),
        surface_background: color(FRAME),
        background: color(FRAME),
        element_background: color(POPUP_BORDER),
        element_hover: color(STRONG_BORDER_GRAY),
        element_active: color(STRONG_BORDER_GRAY),
        // A showing tool window's stripe button, and other "on" toggles.
        element_selected: color(WHITE).alpha(0.16),
        element_selection_background: color(CODE_SELECTION),
        element_disabled: color(POPUP_BORDER),
        ghost_element_background: transparent,
        ghost_element_hover: color(HOVER),
        ghost_element_active: color(POPUP_BORDER),
        ghost_element_selected: color(ROW_SELECTION),
        ghost_element_disabled: transparent,
        text: color(TEXT),
        text_muted: color(TEXT_MUTED),
        text_placeholder: color(TEXT_DIM),
        text_disabled: color(FG_FAINT),
        text_accent: color(ACCENT),
        icon: color(FG_ICON),
        icon_muted: color(FG_MUTED),
        icon_disabled: color(FG_FAINT),
        icon_placeholder: color(FG_FAINT),
        icon_accent: color(ACCENT),
        status_bar_background: color(FRAME),
        title_bar_background: color(FRAME),
        title_bar_inactive_background: color(FRAME),
        toolbar_background: color(ISLAND),
        tab_bar_background: color(ISLAND),
        tab_inactive_background: color(ISLAND),
        tab_active_background: color(TAB_SELECTED_BLUE),
        search_match_background: color(SEARCH_RESULT),
        search_active_match_background: color(SEARCH_RESULT_CURRENT),
        panel_background: color(ISLAND),
        panel_focused_border: color(ACCENT_FILL),
        panel_indent_guide: color(INDENT_GUIDE),
        panel_indent_guide_hover: color(INDENT_GUIDE_ACTIVE),
        panel_indent_guide_active: color(INDENT_GUIDE_ACTIVE),
        panel_overlay_background: color(ISLAND),
        panel_overlay_hover: color(HOVER),
        pane_focused_border: color(ACCENT_FILL),
        pane_group_border: color(FRAME),
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
        editor_background: color(ISLAND),
        editor_gutter_background: color(ISLAND),
        editor_subheader_background: color(FRAME),
        editor_active_line_background: color(CARET_ROW),
        editor_highlighted_line_background: color(CARET_ROW),
        editor_line_number: color(LINE_NUMBER),
        editor_active_line_number: color(CARET_LINE_NUMBER),
        editor_hover_line_number: color(CARET_LINE_NUMBER),
        editor_invisible: color(VCS_DELETED),
        editor_wrap_guide: color(INDENT_GUIDE),
        editor_active_wrap_guide: color(INDENT_GUIDE),
        editor_indent_guide: color(INDENT_GUIDE),
        editor_indent_guide_active: color(INDENT_GUIDE_ACTIVE),
        editor_document_highlight_read_background: color(IDENTIFIER_UNDER_CARET),
        editor_document_highlight_write_background: color(IDENTIFIER_UNDER_CARET_WRITE),
        editor_document_highlight_bracket_background: color(MATCHED_BRACE),
        editor_diff_hunk_added_background: color(GREEN).alpha(0.12),
        editor_diff_hunk_added_hollow_background: color(GREEN).alpha(0.06),
        editor_diff_hunk_added_hollow_border: color(GREEN).alpha(0.36),
        editor_diff_hunk_deleted_background: color(RED).alpha(0.12),
        editor_diff_hunk_deleted_hollow_background: color(RED).alpha(0.06),
        editor_diff_hunk_deleted_hollow_border: color(RED).alpha(0.36),
        terminal_background: color(ISLAND),
        terminal_foreground: color(CODE_TEXT),
        terminal_bright_foreground: color(TEXT),
        terminal_dim_foreground: color(shade(CODE_TEXT, DIM)),
        terminal_ansi_background: color(ISLAND),
        terminal_ansi_black: color(ANSI_BLACK),
        terminal_ansi_bright_black: color(ANSI_BRIGHT_BLACK),
        terminal_ansi_dim_black: color(ANSI_BLACK),
        terminal_ansi_red: color(ANSI_RED),
        terminal_ansi_bright_red: color(ANSI_BRIGHT_RED),
        terminal_ansi_dim_red: color(shade(ANSI_RED, DIM)),
        terminal_ansi_green: color(ANSI_GREEN),
        terminal_ansi_bright_green: color(ANSI_BRIGHT_GREEN),
        terminal_ansi_dim_green: color(shade(ANSI_GREEN, DIM)),
        terminal_ansi_yellow: color(ANSI_YELLOW),
        terminal_ansi_bright_yellow: color(ANSI_BRIGHT_YELLOW),
        terminal_ansi_dim_yellow: color(shade(ANSI_YELLOW, DIM)),
        terminal_ansi_blue: color(ANSI_BLUE),
        terminal_ansi_bright_blue: color(ANSI_BRIGHT_BLUE),
        terminal_ansi_dim_blue: color(shade(ANSI_BLUE, DIM)),
        terminal_ansi_magenta: color(ANSI_MAGENTA),
        terminal_ansi_bright_magenta: color(ANSI_BRIGHT_MAGENTA),
        terminal_ansi_dim_magenta: color(shade(ANSI_MAGENTA, DIM)),
        terminal_ansi_cyan: color(ANSI_CYAN),
        terminal_ansi_bright_cyan: color(ANSI_BRIGHT_CYAN),
        terminal_ansi_dim_cyan: color(shade(ANSI_CYAN, DIM)),
        terminal_ansi_white: color(ANSI_WHITE),
        terminal_ansi_bright_white: color(WHITE),
        terminal_ansi_dim_white: color(shade(ANSI_WHITE, DIM)),
        link_text_hover: color(ACCENT),
        version_control_added: color(VCS_ADDED),
        version_control_deleted: color(VCS_DELETED),
        version_control_modified: color(VCS_MODIFIED),
        version_control_renamed: color(VCS_MODIFIED),
        version_control_conflict: color(VCS_CONFLICT),
        version_control_ignored: color(VCS_IGNORED),
        version_control_word_added: color(shade(GREEN, -50)),
        version_control_word_deleted: color(shade(RED, -50)),
        version_control_conflict_marker_ours: color(GREEN).alpha(0.125),
        version_control_conflict_marker_theirs: color(RED).alpha(0.125),
    }
}

pub fn status_colors() -> StatusColors {
    StatusColors {
        conflict: color(VCS_CONFLICT),
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
        ignored: color(VCS_IGNORED),
        ignored_background: color(POPUP),
        ignored_border: color(POPUP_BORDER),
        info: color(INFO_BLUE),
        info_background: color(POPUP),
        info_border: color(POPUP_BORDER),
        modified: color(VCS_MODIFIED),
        renamed: color(VCS_MODIFIED),
        success: color(SUCCESS),
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
    CursorColors { cursor: color(CARET), selection: color(CODE_SELECTION) }
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

/// The project colour slot for `name`, as IntelliJ and OpenIDEA derive it: a Java string hash
/// (over UTF-16 code units) modulo the number of colours.
pub fn project_color_index(name: &str) -> usize {
    let hash = name.encode_utf16().fold(0_i32, |hash, unit| hash.wrapping_mul(31).wrapping_add(i32::from(unit)));
    (i64::from(hash).unsigned_abs() % PROJECT_COLORS.len() as u64) as usize
}

/// The start and end of `name`'s project colour gradient.
pub fn project_colors(name: &str) -> (Hsla, Hsla) {
    project_colors_at(project_color_index(name))
}

/// The start and end of project colour slot `index` (wrapping).
pub fn project_colors_at(index: usize) -> (Hsla, Hsla) {
    let (start, end) = PROJECT_COLORS[index % PROJECT_COLORS.len()];
    (color(start), color(end))
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
        for hex in [0x3871e1, 0x26282c, 0x000000] {
            let rgba = Rgba::from(color(hex));
            let channels = [rgba.r, rgba.g, rgba.b].map(|channel| (channel * 255.).round() as u32);
            assert_eq!(channels[0] << 16 | channels[1] << 8 | channels[2], hex);
            assert_eq!(rgba.a, 1.);
        }
    }

    #[test]
    fn test_project_color_index_matches_openidea() {
        // OpenIDEA's projectColorIndex is 1-based; these are its slots minus one.
        assert_eq!(project_color_index(""), 0);
        assert_eq!(project_color_index("a"), 97 % 9);
        assert_eq!(project_color_index("ab"), (97 * 31 + 98) % 9);
        assert!(project_color_index("intelliED-editor") < 9);
        assert_eq!(project_colors("ab"), project_colors_at(project_color_index("ab")));
        assert_eq!(project_colors_at(9), project_colors_at(0));
    }

    #[test]
    fn test_fg_sets_only_color() {
        assert_eq!(fg(RED), HighlightStyle { color: Some(color(RED)), ..HighlightStyle::default() });
    }

    #[test]
    fn test_shade_scales_and_clamps_channels() {
        assert_eq!(shade(FRAME, 0), FRAME);
        assert_eq!(shade(0x808080, -50), 0x404040);
        assert_eq!(shade(0xff8040, 20), 0xff994c);
        assert_eq!(shade(FG_STRONG, -100), 0x000000);
    }

    #[test]
    fn test_roles_use_islands_surfaces() {
        assert_eq!(FRAME, 0x26282c);
        assert_eq!(ISLAND, 0x191a1c);
        assert_eq!(ACCENT_FILL, 0x3871e1);
        assert_eq!(ROW_SELECTION, 0x2a4371);
    }

    #[test]
    fn test_theme_is_islands_dark() {
        let theme = theme();
        assert_eq!(theme.name.as_ref(), "Islands Dark");
        assert_eq!(theme.appearance, Appearance::Dark);

        let colors = theme.colors();
        // The frame: around the islands, the main toolbar and the status bar.
        assert_eq!(colors.title_bar_background, colors.background);
        assert_eq!(colors.status_bar_background, colors.background);
        // Islands: editors (with their tab rows), tool windows and terminals share one colour.
        assert_eq!(colors.tab_bar_background, colors.editor_background);
        assert_eq!(colors.panel_background, colors.editor_background);
        assert_eq!(colors.terminal_background, colors.editor_background);
        assert_eq!(colors.terminal_ansi_background, colors.terminal_background);
        assert!(colors.editor_background.l < colors.background.l);
        assert!(colors.panel_overlay_hover.is_opaque());
        assert_ne!(colors.tab_active_background, colors.editor_background);
        assert_eq!(colors.element_selection_background, theme.cursor_colors().selection);
    }

    #[test]
    fn test_terminal_dim_darker_and_bright_distinct() {
        let colors = theme_colors();
        for (dim, normal, bright) in [
            (colors.terminal_ansi_dim_red, colors.terminal_ansi_red, colors.terminal_ansi_bright_red),
            (colors.terminal_ansi_dim_green, colors.terminal_ansi_green, colors.terminal_ansi_bright_green),
            (colors.terminal_ansi_dim_blue, colors.terminal_ansi_blue, colors.terminal_ansi_bright_blue),
        ] {
            // Bright colours are IntelliJ's own (more saturated), not lighter shades.
            assert!(dim.l < normal.l && bright != normal);
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
