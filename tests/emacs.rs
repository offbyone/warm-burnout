mod common;

use common::{extract_hex_colors, is_valid_hex};

const DARK: &str = include_str!("../emacs/warm-burnout-dark-theme.el");
const LIGHT: &str = include_str!("../emacs/warm-burnout-light-theme.el");
const SHARED: &str = include_str!("../emacs/warm-burnout.el");

#[test]
fn dark_all_hex_colors_valid() {
  for (line, hex) in extract_hex_colors(DARK) {
    assert!(is_valid_hex(hex), "line {line}: invalid hex: {hex}");
  }
}

#[test]
fn light_all_hex_colors_valid() {
  for (line, hex) in extract_hex_colors(LIGHT) {
    assert!(is_valid_hex(hex), "line {line}: invalid hex: {hex}");
  }
}

#[test]
fn shared_all_hex_colors_valid() {
  for (line, hex) in extract_hex_colors(SHARED) {
    assert!(is_valid_hex(hex), "line {line}: invalid hex: {hex}");
  }
}

#[test]
fn shared_dark_uses_canonical_background() {
  assert!(
    SHARED.contains("#1a1510"),
    "shared palette must contain canonical dark background #1a1510"
  );
}

#[test]
fn shared_light_uses_canonical_background() {
  assert!(
    SHARED.contains("#F5EDE0"),
    "shared palette must contain canonical light background #F5EDE0"
  );
}

#[test]
fn shared_dark_uses_canonical_foreground() {
  assert!(
    SHARED.contains("#bfbdb6"),
    "shared palette must contain canonical dark foreground #bfbdb6"
  );
}

#[test]
fn shared_light_uses_canonical_foreground() {
  assert!(
    SHARED.contains("#3a3630"),
    "shared palette must contain canonical light foreground #3a3630"
  );
}

#[test]
fn dark_provides_theme() {
  assert!(
    DARK.contains("(provide-theme 'warm-burnout-dark)"),
    "dark theme must provide-theme"
  );
}

#[test]
fn light_provides_theme() {
  assert!(
    LIGHT.contains("(provide-theme 'warm-burnout-light)"),
    "light theme must provide-theme"
  );
}

#[test]
fn dark_has_font_lock_faces() {
  let required = [
    "font-lock-keyword-face",
    "font-lock-function-name-face",
    "font-lock-type-face",
    "font-lock-string-face",
    "font-lock-comment-face",
    "font-lock-constant-face",
  ];
  for face in required {
    assert!(DARK.contains(face), "dark theme missing face: {face}");
  }
}

#[test]
fn light_has_font_lock_faces() {
  let required = [
    "font-lock-keyword-face",
    "font-lock-function-name-face",
    "font-lock-type-face",
    "font-lock-string-face",
    "font-lock-comment-face",
    "font-lock-constant-face",
  ];
  for face in required {
    assert!(LIGHT.contains(face), "light theme missing face: {face}");
  }
}

#[test]
fn dark_keywords_are_bold() {
  assert!(
    DARK.contains("font-lock-keyword-face ((t (:foreground ,keyword :weight bold)"),
    "keywords must be bold in dark theme"
  );
}

#[test]
fn dark_types_are_italic() {
  assert!(
    DARK.contains("font-lock-type-face ((t (:foreground ,type :slant italic)"),
    "types must be italic in dark theme"
  );
}

#[test]
fn dark_comments_are_italic() {
  assert!(
    DARK.contains("font-lock-comment-face ((t (:foreground ,comment :slant italic)"),
    "comments must be italic in dark theme"
  );
}

#[test]
fn shared_defines_both_palettes() {
  assert!(
    SHARED.contains("warm-burnout-dark-palette"),
    "shared must define dark palette"
  );
  assert!(
    SHARED.contains("warm-burnout-light-palette"),
    "shared must define light palette"
  );
}

#[test]
fn brand_name_in_themes() {
  assert!(DARK.contains("Warm Burnout"), "dark theme must contain brand name");
  assert!(LIGHT.contains("Warm Burnout"), "light theme must contain brand name");
}
