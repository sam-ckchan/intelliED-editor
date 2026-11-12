use std::sync::OnceLock;

use gpui::FontStyle;
use gpui::FontWeight;
use gpui::HighlightStyle;
use gpui::Hsla;
use gpui::SharedString;
use gpui::WindowBackgroundAppearance;
use gpui::rgb;
use gpui::rgba;
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

/// OpenIDEA's three built-in themes (IntelliJ's "Islands", PyCharm 2026.1): rounded islands on a
/// frame. One is chosen at launch and kept for the life of the process.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Variant {
    Dark,
    Light,
    DarkHighContrast,
}

impl Variant {
    /// The variant matching the macOS appearance: light mode is Islands Light, dark mode is Islands
    /// Dark, or High Contrast when "Increase contrast" is on.
    pub const fn for_system(appearance: Appearance, increase_contrast: bool) -> Self {
        match (appearance, increase_contrast) {
            (Appearance::Light, _) => Self::Light,
            (Appearance::Dark, false) => Self::Dark,
            (Appearance::Dark, true) => Self::DarkHighContrast,
        }
    }

    /// Parses OpenIDEA's theme ids, as used by the `ED_THEME` override.
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "dark" => Some(Self::Dark),
            "light" => Some(Self::Light),
            "dark-hc" => Some(Self::DarkHighContrast),
            _ => None,
        }
    }
}

static THEME: OnceLock<Theme> = OnceLock::new();

/// Chooses the theme for this process. Only the first call (before any `theme()`) takes effect.
pub fn init(variant: Variant) {
    THEME.get_or_init(|| build(variant));
}

/// The theme chosen by `init`, Islands Dark if `init` wasn't called (e.g. in tests).
pub fn theme() -> &'static Theme {
    THEME.get_or_init(|| build(Variant::Dark))
}

pub fn build(variant: Variant) -> Theme {
    let (id, name, appearance, palette) = match variant {
        Variant::Dark => ("islands-dark", "Islands Dark", Appearance::Dark, &DARK),
        Variant::Light => ("islands-light", "Islands Light", Appearance::Light, &LIGHT),
        Variant::DarkHighContrast => ("islands-dark-hc", "High Contrast", Appearance::Dark, &DARK_HIGH_CONTRAST),
    };
    Theme {
        id: id.into(),
        name: SharedString::new_static(name),
        appearance,
        styles: ThemeStyles {
            window_background_appearance: WindowBackgroundAppearance::Opaque,
            system: system_colors(),
            accents: accent_colors(palette),
            colors: theme_colors(palette),
            status: status_colors(palette),
            cursor_colors: cursor_colors(palette),
            syntax: syntax_theme(palette),
        },
    }
}

/// One theme's raw colours, named after OpenIDEA's design tokens (`src/styles/tokens.css`).
/// `0xRRGGBBAA` for the translucent ones, `0xRRGGBB` otherwise.
pub struct Palette {
    /// `--bg-frame`: around and between the islands, the main toolbar, the stripes, the status bar.
    pub frame: u32,
    /// `--bg-island`: editors (with their tabs), tool windows, terminals.
    pub island: u32,
    pub popup: u32,
    /// `--border`: lines inside an island.
    pub border: u32,
    pub border_popup: u32,
    pub border_strong: u32,
    pub hover: u32,
    /// `--bg-stripe-selected` (translucent): a showing tool window's stripe button.
    pub stripe_selected: u32,
    /// `--bg-tree-selected`: the selected row of a list or tree.
    pub row_selected: u32,
    pub selected_strong: u32,
    pub tab_selected: u32,
    pub scrollbar_thumb: u32,
    pub scrollbar_thumb_hover: u32,
    pub fg_strong: u32,
    pub fg_secondary: u32,
    pub fg_muted: u32,
    pub fg_subtle: u32,
    pub fg_faint: u32,
    pub fg_icon: u32,
    /// `--accent`: fills such as focus borders and primary buttons.
    pub accent: u32,
    /// `--link`: accent text, icons and links.
    pub link: u32,
    pub info: u32,
    pub danger: u32,
    pub warning: u32,
    pub success: u32,
    pub vcs_modified: u32,
    pub vcs_added: u32,
    pub vcs_deleted: u32,
    pub vcs_ignored: u32,
    pub vcs_conflict: u32,
    // The editor colour scheme.
    pub fg_editor: u32,
    pub fg_whitespace: u32,
    pub fg_inlay: u32,
    pub syn_keyword: u32,
    pub syn_string: u32,
    pub syn_number: u32,
    pub syn_comment: u32,
    pub syn_doc_comment: u32,
    /// `--syn-comment-style`: Islands Light shows comments in italics.
    pub italic_comments: bool,
    pub syn_function: u32,
    pub syn_constant: u32,
    pub syn_annotation: u32,
    /// IntelliJ's TYPE_PARAMETER_NAME (no OpenIDEA token): labels and lifetimes.
    pub syn_type_parameter: u32,
    pub syn_tag: u32,
    pub syn_regexp: u32,
    pub caret: u32,
    pub current_line: u32,
    pub editor_selection: u32,
    pub line_number: u32,
    pub line_number_current: u32,
    pub indent_guide: u32,
    pub indent_guide_active: u32,
    pub search_result: u32,
    pub search_result_current: u32,
    pub matched_brace: u32,
    pub identifier_under_caret: u32,
    pub identifier_under_caret_write: u32,
    /// The terminal's ANSI colours: black, red, green, yellow, blue, magenta, cyan, white, then
    /// their bright variants.
    pub ansi: [u32; 16],
}

/// `:root` in tokens.css.
pub const DARK: Palette = Palette {
    frame: 0x26282c,
    island: 0x191a1c,
    popup: 0x26282c,
    border: 0x26282c,
    border_popup: 0x33353b,
    border_strong: 0x40434a,
    hover: 0x2e2f30,
    stripe_selected: 0xffffff29,
    row_selected: 0x2a4371,
    selected_strong: 0x2e4d89,
    tab_selected: 0x233558,
    scrollbar_thumb: 0xffffff33,
    scrollbar_thumb_hover: 0xffffff59,
    fg_strong: 0xdfe1e5,
    fg_secondary: 0x9fa2a8,
    fg_muted: 0x8c8f95,
    fg_subtle: 0x73767c,
    fg_faint: 0x5f6269,
    fg_icon: 0xced0d6,
    accent: 0x3871e1,
    link: 0x71a1fe,
    info: 0x548af7,
    danger: 0xf57e84,
    warning: 0xd59637,
    success: 0x6db083,
    vcs_modified: 0x70aeff,
    vcs_added: 0x73bd79,
    vcs_deleted: 0x6f737a,
    vcs_ignored: 0xd69a6b,
    vcs_conflict: 0xde6a66,
    fg_editor: 0xbcbec4,
    fg_whitespace: 0x6f737a,
    fg_inlay: 0x868a91,
    syn_keyword: 0xcf8e6d,
    syn_string: 0x6aab73,
    syn_number: 0x2aacb8,
    syn_comment: 0x7a7e85,
    syn_doc_comment: 0x5f826b,
    italic_comments: false,
    syn_function: 0x56a8f5,
    syn_constant: 0xc77dbb,
    syn_annotation: 0xb3ae60,
    syn_type_parameter: 0x16baac,
    syn_tag: 0xd5b778,
    syn_regexp: 0x42c3d4,
    caret: 0xced0d6,
    current_line: 0x1f2024,
    editor_selection: 0x214283,
    line_number: 0x4b5059,
    line_number_current: 0xa1a3ab,
    indent_guide: 0x323438,
    indent_guide_active: 0x4e5157,
    search_result: 0x114957,
    search_result_current: 0x165e70,
    matched_brace: 0x43454a,
    identifier_under_caret: 0x373b39,
    identifier_under_caret_write: 0x402f33,
    ansi: [
        0x000000, 0xf27481, 0x6bcc62, 0xe0ce70, 0x5594fa, 0xc092fa, 0x47ccbd, 0xced0d6, //
        0x4e5157, 0xff6b7a, 0x67ff59, 0xffec1a, 0x3399ff, 0xd970ff, 0x40ffe9, 0xffffff,
    ],
};

/// `[data-theme="light"]` in tokens.css.
pub const LIGHT: Palette = Palette {
    frame: 0xe9eaee,
    island: 0xffffff,
    popup: 0xffffff,
    border: 0xe9eaee,
    border_popup: 0xd1d3d9,
    border_strong: 0xd1d3d9,
    hover: 0xededed,
    stripe_selected: 0x00000020,
    row_selected: 0xd0dffe,
    selected_strong: 0xbdd3ff,
    tab_selected: 0xe3ebfe,
    scrollbar_thumb: 0x00000033,
    scrollbar_thumb_hover: 0x00000080,
    fg_strong: 0x000000,
    fg_secondary: 0x5f6269,
    fg_muted: 0x6c707e,
    fg_subtle: 0x73767c,
    fg_faint: 0x9fa2a8,
    fg_icon: 0x6c707e,
    accent: 0x3871e1,
    link: 0x2f5eb9,
    info: 0x3871e1,
    danger: 0xc54e58,
    warning: 0xa56906,
    success: 0x338555,
    vcs_modified: 0x0033b3,
    vcs_added: 0x067d17,
    vcs_deleted: 0x6c707e,
    vcs_ignored: 0x8c4f00,
    vcs_conflict: 0xde1b2e,
    fg_editor: 0x080808,
    fg_whitespace: 0xb5b7bd,
    fg_inlay: 0x73767c,
    syn_keyword: 0x0033b3,
    syn_string: 0x067d17,
    syn_number: 0x1750eb,
    syn_comment: 0x8c8c8c,
    syn_doc_comment: 0x8c8c8c,
    italic_comments: true,
    syn_function: 0x00627a,
    syn_constant: 0x871094,
    syn_annotation: 0x9e880d,
    syn_type_parameter: 0x20999d,
    syn_tag: 0x0033b3,
    syn_regexp: 0x264eff,
    caret: 0x000000,
    current_line: 0xf5f8fe,
    editor_selection: 0xa6d2ff,
    line_number: 0xaeb3c2,
    line_number_current: 0x767a8a,
    indent_guide: 0xebecf0,
    indent_guide_active: 0xaeb3c2,
    search_result: 0xfcd47e,
    search_result_current: 0xf2bf57,
    matched_brace: 0x93d9d9,
    identifier_under_caret: 0xedebfc,
    identifier_under_caret_write: 0xfce8f4,
    ansi: [
        0x000000, 0xc93645, 0x357d1f, 0xb2a400, 0x225cd6, 0x6e32e5, 0x007a80, 0xbec0c5, //
        0x6c707e, 0xff5462, 0x3dcc3d, 0xd4bb00, 0x009dff, 0xd659ff, 0x31c4b3, 0xd2d4d9,
    ],
};

/// `[data-theme="dark-hc"]` in tokens.css.
pub const DARK_HIGH_CONTRAST: Palette = Palette {
    frame: 0x1a1b1e,
    island: 0x000000,
    popup: 0x111214,
    border: 0x45474d,
    border_popup: 0x55585f,
    border_strong: 0x6f737a,
    hover: 0x1c1e22,
    stripe_selected: 0xffffff38,
    row_selected: 0x1f3a73,
    selected_strong: 0x2a4b8f,
    tab_selected: 0x1f3a73,
    scrollbar_thumb: 0xffffff4d,
    scrollbar_thumb_hover: 0xffffff80,
    fg_strong: 0xffffff,
    fg_secondary: 0xccced3,
    fg_muted: 0xb6b8be,
    fg_subtle: 0x9fa2a9,
    fg_faint: 0x8b8e95,
    fg_icon: 0xe3e4e8,
    accent: 0x8fb3ff,
    link: 0xa7c5ff,
    info: 0x92b7ff,
    danger: 0xff9b9f,
    warning: 0xf2c55c,
    success: 0x8ec39d,
    vcs_modified: 0x92c3ff,
    vcs_added: 0x8ed694,
    vcs_deleted: 0xa5a8ae,
    vcs_ignored: 0xe5b48c,
    vcs_conflict: 0xff8f8a,
    fg_editor: 0xe3e4e8,
    fg_whitespace: 0x8b8e95,
    fg_inlay: 0xccced3,
    syn_keyword: 0xe8a885,
    syn_string: 0x8ccc94,
    syn_number: 0x4fd0dc,
    syn_comment: 0xa5a9b0,
    syn_doc_comment: 0x8fb89c,
    italic_comments: false,
    syn_function: 0x7cc0ff,
    syn_constant: 0xe0a0d6,
    syn_annotation: 0xd6d17a,
    syn_type_parameter: 0x4fd6c8,
    syn_tag: 0xecd39a,
    syn_regexp: 0x6ad8e6,
    caret: 0xffffff,
    current_line: 0x131418,
    editor_selection: 0x2a5bb8,
    line_number: 0x8b8e95,
    line_number_current: 0xe3e4e8,
    indent_guide: 0x45474d,
    indent_guide_active: 0x8b8e95,
    search_result: 0x146a7d,
    search_result_current: 0x1d8aa3,
    matched_brace: 0x55585f,
    identifier_under_caret: 0x3f4a44,
    identifier_under_caret_write: 0x553840,
    ansi: [
        0x000000, 0xff6b68, 0x7ac74f, 0xe5c33f, 0x5aaaf0, 0xc79be0, 0x2fc7c7, 0xb6b8be, //
        0x8b8e95, 0xff8080, 0x7ae04a, 0xffd84d, 0x6cc8ff, 0xff9cff, 0x4ff0f0, 0xffffff,
    ],
};

// Accent hues shared by all variants (from the project colours).
const ORANGE: u32 = 0xe08855;
const PURPLE: u32 = 0xa571e6;
const TEAL: u32 = 0x24a394;

/// Project colours (IntelliJ's RecentProject.Color1–9), the same in every variant: each project
/// gets one pair, start and end of its gradient, chosen from its name.
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

/// Terminal dim variants of an ANSI colour.
const DIM: i32 = -30;

pub fn theme_colors(p: &Palette) -> ThemeColors {
    let transparent = transparent_black();
    let scrollbar = color_alpha(p.scrollbar_thumb);
    let scrollbar_hover = color_alpha(p.scrollbar_thumb_hover);
    let [
        black,
        red,
        green,
        yellow,
        blue,
        magenta,
        cyan,
        white,
        bright_black,
        bright_red,
        bright_green,
        bright_yellow,
        bright_blue,
        bright_magenta,
        bright_cyan,
        bright_white,
    ] = p.ansi;
    ThemeColors {
        border: color(p.border),
        border_variant: color(p.border),
        border_focused: color(p.accent),
        border_selected: color(p.selected_strong),
        border_transparent: transparent,
        border_disabled: color(p.border_strong),
        elevated_surface_background: color(p.popup),
        surface_background: color(p.frame),
        background: color(p.frame),
        element_background: color(p.border_popup),
        element_hover: color(p.border_strong),
        element_active: color(p.border_strong),
        // A showing tool window's stripe button, and other "on" toggles.
        element_selected: color_alpha(p.stripe_selected),
        element_selection_background: color(p.editor_selection),
        element_disabled: color(p.border_popup),
        ghost_element_background: transparent,
        ghost_element_hover: color(p.hover),
        ghost_element_active: color(p.border_popup),
        ghost_element_selected: color(p.row_selected),
        ghost_element_disabled: transparent,
        text: color(p.fg_strong),
        text_muted: color(p.fg_secondary),
        text_placeholder: color(p.fg_subtle),
        text_disabled: color(p.fg_faint),
        text_accent: color(p.link),
        icon: color(p.fg_icon),
        icon_muted: color(p.fg_muted),
        icon_disabled: color(p.fg_faint),
        icon_placeholder: color(p.fg_faint),
        icon_accent: color(p.link),
        status_bar_background: color(p.frame),
        title_bar_background: color(p.frame),
        title_bar_inactive_background: color(p.frame),
        toolbar_background: color(p.island),
        tab_bar_background: color(p.island),
        tab_inactive_background: color(p.island),
        tab_active_background: color(p.tab_selected),
        search_match_background: color(p.search_result),
        search_active_match_background: color(p.search_result_current),
        panel_background: color(p.island),
        panel_focused_border: color(p.accent),
        panel_indent_guide: color(p.indent_guide),
        panel_indent_guide_hover: color(p.indent_guide_active),
        panel_indent_guide_active: color(p.indent_guide_active),
        panel_overlay_background: color(p.island),
        panel_overlay_hover: color(p.hover),
        pane_focused_border: color(p.accent),
        pane_group_border: color(p.frame),
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
        editor_foreground: color(p.fg_editor),
        editor_code_lens_foreground: None,
        editor_background: color(p.island),
        editor_gutter_background: color(p.island),
        editor_subheader_background: color(p.frame),
        editor_active_line_background: color(p.current_line),
        editor_highlighted_line_background: color(p.current_line),
        editor_line_number: color(p.line_number),
        editor_active_line_number: color(p.line_number_current),
        editor_hover_line_number: color(p.line_number_current),
        editor_invisible: color(p.fg_whitespace),
        editor_wrap_guide: color(p.indent_guide),
        editor_active_wrap_guide: color(p.indent_guide),
        editor_indent_guide: color(p.indent_guide),
        editor_indent_guide_active: color(p.indent_guide_active),
        editor_document_highlight_read_background: color(p.identifier_under_caret),
        editor_document_highlight_write_background: color(p.identifier_under_caret_write),
        editor_document_highlight_bracket_background: color(p.matched_brace),
        editor_diff_hunk_added_background: color(p.vcs_added).alpha(0.12),
        editor_diff_hunk_added_hollow_background: color(p.vcs_added).alpha(0.06),
        editor_diff_hunk_added_hollow_border: color(p.vcs_added).alpha(0.36),
        editor_diff_hunk_deleted_background: color(p.danger).alpha(0.12),
        editor_diff_hunk_deleted_hollow_background: color(p.danger).alpha(0.06),
        editor_diff_hunk_deleted_hollow_border: color(p.danger).alpha(0.36),
        terminal_background: color(p.island),
        terminal_foreground: color(p.fg_editor),
        terminal_bright_foreground: color(p.fg_strong),
        terminal_dim_foreground: color(shade(p.fg_editor, DIM)),
        terminal_ansi_background: color(p.island),
        terminal_ansi_black: color(black),
        terminal_ansi_bright_black: color(bright_black),
        terminal_ansi_dim_black: color(black),
        terminal_ansi_red: color(red),
        terminal_ansi_bright_red: color(bright_red),
        terminal_ansi_dim_red: color(shade(red, DIM)),
        terminal_ansi_green: color(green),
        terminal_ansi_bright_green: color(bright_green),
        terminal_ansi_dim_green: color(shade(green, DIM)),
        terminal_ansi_yellow: color(yellow),
        terminal_ansi_bright_yellow: color(bright_yellow),
        terminal_ansi_dim_yellow: color(shade(yellow, DIM)),
        terminal_ansi_blue: color(blue),
        terminal_ansi_bright_blue: color(bright_blue),
        terminal_ansi_dim_blue: color(shade(blue, DIM)),
        terminal_ansi_magenta: color(magenta),
        terminal_ansi_bright_magenta: color(bright_magenta),
        terminal_ansi_dim_magenta: color(shade(magenta, DIM)),
        terminal_ansi_cyan: color(cyan),
        terminal_ansi_bright_cyan: color(bright_cyan),
        terminal_ansi_dim_cyan: color(shade(cyan, DIM)),
        terminal_ansi_white: color(white),
        terminal_ansi_bright_white: color(bright_white),
        terminal_ansi_dim_white: color(shade(white, DIM)),
        link_text_hover: color(p.link),
        version_control_added: color(p.vcs_added),
        version_control_deleted: color(p.vcs_deleted),
        version_control_modified: color(p.vcs_modified),
        version_control_renamed: color(p.vcs_modified),
        version_control_conflict: color(p.vcs_conflict),
        version_control_ignored: color(p.vcs_ignored),
        version_control_word_added: color(shade(p.vcs_added, -50)),
        version_control_word_deleted: color(shade(p.danger, -50)),
        version_control_conflict_marker_ours: color(p.vcs_added).alpha(0.125),
        version_control_conflict_marker_theirs: color(p.danger).alpha(0.125),
    }
}

pub fn status_colors(p: &Palette) -> StatusColors {
    StatusColors {
        conflict: color(p.vcs_conflict),
        created: color(p.vcs_added),
        created_background: color(p.vcs_added).alpha(0.25),
        deleted: color(p.danger),
        deleted_background: color(p.danger).alpha(0.25),
        error: color(p.danger),
        error_background: color(p.danger).alpha(0.2),
        error_border: color(p.danger),
        hidden: color(p.fg_subtle),
        hint: color(p.fg_inlay),
        hint_background: color(p.popup),
        hint_border: color(p.border_popup),
        ignored: color(p.vcs_ignored),
        ignored_background: color(p.popup),
        ignored_border: color(p.border_popup),
        info: color(p.info),
        info_background: color(p.popup),
        info_border: color(p.border_popup),
        modified: color(p.vcs_modified),
        renamed: color(p.vcs_modified),
        success: color(p.success),
        success_background: color(p.popup),
        success_border: color(p.border_popup),
        unreachable: color(p.fg_secondary),
        warning: color(p.warning),
        warning_background: color(p.warning).alpha(0.2),
        warning_border: color(p.warning),
    }
}

/// The local cursor and selection.
pub fn cursor_colors(p: &Palette) -> CursorColors {
    CursorColors { cursor: color(p.caret), selection: color(p.editor_selection) }
}

pub fn accent_colors(p: &Palette) -> AccentColors {
    AccentColors([p.link, TEAL, p.danger, ORANGE, p.warning, p.vcs_added, PURPLE].map(color))
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
pub fn syntax_theme(p: &Palette) -> SyntaxTheme {
    let italic = |hex| HighlightStyle { font_style: Some(FontStyle::Italic), ..fg(hex) };
    let bold = |hex| HighlightStyle { font_weight: Some(FontWeight::BOLD), ..fg(hex) };
    let comment = |hex| if p.italic_comments { italic(hex) } else { fg(hex) };
    let text = p.fg_editor;
    SyntaxTheme::new(
        [
            ("attribute", fg(p.syn_annotation)),
            ("boolean", fg(p.syn_keyword)),
            ("comment", comment(p.syn_comment)),
            ("comment.doc", comment(p.syn_doc_comment)),
            ("constant", fg(p.syn_constant)),
            ("constructor", fg(text)),
            ("definition.function", fg(p.syn_function)),
            ("embedded", fg(text)),
            ("emphasis", italic(text)),
            ("emphasis.strong", bold(text)),
            ("enum", fg(text)),
            ("function", fg(p.syn_function)),
            ("function.method", fg(p.syn_function)),
            ("hint", fg(p.fg_inlay)),
            ("keyword", fg(p.syn_keyword)),
            ("label", fg(p.syn_type_parameter)),
            ("lifetime", fg(p.syn_type_parameter)),
            ("link_text", HighlightStyle { font_style: Some(FontStyle::Normal), ..fg(p.link) }),
            ("link_uri", fg(p.link)),
            ("number", fg(p.syn_number)),
            ("operator", fg(text)),
            ("predictive", fg(p.fg_subtle)),
            ("preproc", fg(p.syn_annotation)),
            ("primary", fg(text)),
            ("property", fg(p.syn_constant)),
            ("punctuation", fg(text)),
            ("punctuation.bracket", fg(text)),
            ("punctuation.delimiter", fg(text)),
            ("punctuation.list_marker", fg(p.syn_keyword)),
            ("punctuation.special", fg(p.syn_keyword)),
            ("string", fg(p.syn_string)),
            ("string.escape", fg(p.syn_keyword)),
            ("string.regex", fg(p.syn_regexp)),
            ("string.special", fg(p.syn_string)),
            ("tag", fg(p.syn_tag)),
            ("text.literal", fg(p.syn_string)),
            ("title", bold(p.syn_constant)),
            ("type", fg(text)),
            ("type.builtin", fg(p.syn_keyword)),
            ("type.super", fg(text)),
            ("variable", fg(text)),
            ("variable.parameter", fg(text)),
            ("variable.special", fg(p.syn_keyword)),
            ("variant", italic(p.syn_constant)),
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

/// A translucent `0xRRGGBBAA` colour.
fn color_alpha(hex: u32) -> Hsla {
    rgba(hex).into()
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

    const VARIANTS: [Variant; 3] = [Variant::Dark, Variant::Light, Variant::DarkHighContrast];

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
    fn test_color_alpha_parses_rgba_hex() {
        assert_eq!(color_alpha(0xffffff33), color(0xffffff).alpha(0x33 as f32 / 255.));
        assert_eq!(color_alpha(0x00000080).a, 0x80 as f32 / 255.);
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
        assert_eq!(fg(0xff0000), HighlightStyle { color: Some(color(0xff0000)), ..HighlightStyle::default() });
    }

    #[test]
    fn test_shade_scales_and_clamps_channels() {
        assert_eq!(shade(DARK.frame, 0), DARK.frame);
        assert_eq!(shade(0x808080, -50), 0x404040);
        assert_eq!(shade(0xff8040, 20), 0xff994c);
        assert_eq!(shade(DARK.fg_strong, -100), 0x000000);
    }

    #[test]
    fn test_variant_for_system() {
        assert_eq!(Variant::for_system(Appearance::Light, false), Variant::Light);
        // Increase contrast only has a dark high-contrast theme.
        assert_eq!(Variant::for_system(Appearance::Light, true), Variant::Light);
        assert_eq!(Variant::for_system(Appearance::Dark, false), Variant::Dark);
        assert_eq!(Variant::for_system(Appearance::Dark, true), Variant::DarkHighContrast);
    }

    #[test]
    fn test_variant_from_id() {
        assert_eq!(Variant::from_id("dark"), Some(Variant::Dark));
        assert_eq!(Variant::from_id("light"), Some(Variant::Light));
        assert_eq!(Variant::from_id("dark-hc"), Some(Variant::DarkHighContrast));
        assert_eq!(Variant::from_id("Light"), None);
        assert_eq!(Variant::from_id(""), None);
    }

    #[test]
    fn test_build_names_and_appearances() {
        let dark = build(Variant::Dark);
        assert_eq!((dark.name.as_ref(), dark.appearance), ("Islands Dark", Appearance::Dark));
        let light = build(Variant::Light);
        assert_eq!((light.name.as_ref(), light.appearance), ("Islands Light", Appearance::Light));
        let high_contrast = build(Variant::DarkHighContrast);
        assert_eq!((high_contrast.name.as_ref(), high_contrast.appearance), ("High Contrast", Appearance::Dark));
        assert_eq!(light.colors().background, color(0xe9eaee));
        assert_eq!(light.colors().editor_background, color(0xffffff));
        assert_eq!(high_contrast.colors().editor_background, color(0x000000));
    }

    #[test]
    fn test_every_variant_keeps_islands_layout() {
        for variant in VARIANTS {
            let theme = build(variant);
            let colors = theme.colors();
            // The frame: around the islands, the main toolbar and the status bar.
            assert_eq!(colors.title_bar_background, colors.background);
            assert_eq!(colors.status_bar_background, colors.background);
            // Islands: editors (with their tab rows), tool windows and terminals share one colour.
            assert_eq!(colors.tab_bar_background, colors.editor_background);
            assert_eq!(colors.panel_background, colors.editor_background);
            assert_eq!(colors.terminal_background, colors.editor_background);
            assert_eq!(colors.terminal_ansi_background, colors.terminal_background);
            assert_ne!(colors.editor_background, colors.background);
            assert!(colors.panel_overlay_hover.is_opaque());
            assert_ne!(colors.tab_active_background, colors.editor_background);
            assert_eq!(colors.element_selection_background, theme.cursor_colors().selection);
            assert_eq!(theme.status().modified, colors.version_control_modified);
        }
    }

    #[test]
    fn test_islands_contrast_with_frame_by_appearance() {
        for variant in VARIANTS {
            let theme = build(variant);
            let colors = theme.colors();
            // Dark islands sit darker than the frame, light islands lighter.
            assert_eq!(colors.editor_background.l > colors.background.l, theme.appearance.is_light(), "{variant:?}");
            // Text contrasts with the island it is drawn on.
            assert!((colors.text.l - colors.editor_background.l).abs() > 0.6, "{variant:?}");
            assert!((colors.editor_foreground.l - colors.editor_background.l).abs() > 0.6, "{variant:?}");
        }
    }

    #[test]
    fn test_terminal_dim_darker_and_bright_distinct() {
        for palette in [&DARK, &LIGHT, &DARK_HIGH_CONTRAST] {
            let colors = theme_colors(palette);
            for (dim, normal, bright) in [
                (colors.terminal_ansi_dim_red, colors.terminal_ansi_red, colors.terminal_ansi_bright_red),
                (colors.terminal_ansi_dim_green, colors.terminal_ansi_green, colors.terminal_ansi_bright_green),
                (colors.terminal_ansi_dim_blue, colors.terminal_ansi_blue, colors.terminal_ansi_bright_blue),
            ] {
                assert!(dim.l < normal.l && bright != normal);
            }
        }
    }

    #[test]
    fn test_status_colors_use_popup_surfaces() {
        let status = status_colors(&DARK);
        assert_eq!(status.ignored_background, status.hint_background);
        assert_eq!(status.ignored_border, status.hint_border);
        assert_eq!(status.error_background, color(DARK.danger).alpha(0.2));
        assert_eq!(status_colors(&LIGHT).info_background, color(LIGHT.popup));
    }

    #[test]
    fn test_syntax_theme_resolves_capture_names() {
        let syntax = syntax_theme(&DARK);
        assert_eq!(syntax.style_for_name("keyword").and_then(|style| style.color), Some(color(0xcf8e6d)));
        assert_eq!(syntax.style_for_name("string").and_then(|style| style.color), Some(color(DARK.syn_string)));
        assert_eq!(syntax.style_for_name("function").and_then(|style| style.color), Some(color(DARK.syn_function)));
        // Uses stay plain text; only declarations and literals are coloured.
        assert_eq!(syntax.style_for_name("type").and_then(|style| style.color), Some(color(DARK.fg_editor)));
        assert_eq!(syntax.style_for_name("title").and_then(|style| style.font_weight), Some(FontWeight::BOLD));
        assert_eq!(syntax.style_for_name("variant").and_then(|style| style.font_style), Some(FontStyle::Italic));
        assert_eq!(syntax.style_for_name("comment").and_then(|style| style.font_style), None);
        assert!(syntax.highlight_id("string.escape").is_some());
        assert!(syntax.style_for_name("missing").is_none());
    }

    #[test]
    fn test_light_syntax_theme_uses_light_scheme() {
        let syntax = syntax_theme(&LIGHT);
        assert_eq!(syntax.style_for_name("keyword").and_then(|style| style.color), Some(color(0x0033b3)));
        assert_eq!(syntax.style_for_name("type").and_then(|style| style.color), Some(color(0x080808)));
        assert_eq!(syntax.style_for_name("comment").and_then(|style| style.font_style), Some(FontStyle::Italic));
        assert_eq!(syntax.style_for_name("comment.doc").and_then(|style| style.font_style), Some(FontStyle::Italic));
    }

    #[test]
    fn test_theme_defaults_to_islands_dark_and_is_static() {
        // Tests never call `init`, so the first `theme()` settles on Islands Dark and `init` is a no-op.
        assert_eq!(theme().name.as_ref(), "Islands Dark");
        init(Variant::Light);
        assert!(std::ptr::eq(theme(), theme()));
        assert_eq!(theme(), &build(Variant::Dark));
        assert_eq!(theme().syntax(), &syntax_theme(&DARK));
    }

    #[test]
    fn test_accent_colors_cycle() {
        let accents = accent_colors(&LIGHT);
        assert_eq!(accents.color_for_index(0), color(LIGHT.link));
        assert_eq!(accents.color_for_index(accents.0.len() as u32), accents.color_for_index(0));
    }
}
