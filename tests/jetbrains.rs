mod common;

use common::{jetbrains_attribute, jetbrains_color};
use serde_json::Value as JsonValue;

const DARK: &str = include_str!("../jetbrains/Warm Burnout Dark.icls");
const LIGHT: &str = include_str!("../jetbrains/Warm Burnout Light.icls");
const DARK_THEME: &str = include_str!("../jetbrains/Warm Burnout Dark.theme.json");
const LIGHT_THEME: &str = include_str!("../jetbrains/Warm Burnout Light.theme.json");
const DARK_ISLANDS: &str = include_str!("../jetbrains/Warm Burnout Islands Dark.theme.json");
const LIGHT_ISLANDS: &str = include_str!("../jetbrains/Warm Burnout Islands Light.theme.json");
const PLUGIN_XML: &str = include_str!("../jetbrains/META-INF/plugin.xml");

// -- Valid XML structure --

#[test]
fn dark_has_xml_declaration() {
  assert!(DARK.starts_with("<?xml"), "dark .icls must start with XML declaration");
}

#[test]
fn light_has_xml_declaration() {
  assert!(
    LIGHT.starts_with("<?xml"),
    "light .icls must start with XML declaration"
  );
}

#[test]
fn dark_has_scheme_element() {
  assert!(DARK.contains("<scheme"), "dark .icls must have <scheme> element");
  assert!(DARK.contains("</scheme>"), "dark .icls must have closing </scheme>");
}

#[test]
fn light_has_scheme_element() {
  assert!(LIGHT.contains("<scheme"), "light .icls must have <scheme> element");
  assert!(LIGHT.contains("</scheme>"), "light .icls must have closing </scheme>");
}

// -- Scheme names and parent schemes --

#[test]
fn dark_scheme_name() {
  assert!(
    DARK.contains("name=\"Warm Burnout Dark\""),
    "dark scheme name must be 'Warm Burnout Dark'"
  );
}

#[test]
fn light_scheme_name() {
  assert!(
    LIGHT.contains("name=\"Warm Burnout Light\""),
    "light scheme name must be 'Warm Burnout Light'"
  );
}

#[test]
fn dark_parent_scheme() {
  assert!(
    DARK.contains("parent_scheme=\"Darcula\""),
    "dark must use Darcula as parent scheme"
  );
}

#[test]
fn light_parent_scheme() {
  assert!(
    LIGHT.contains("parent_scheme=\"Default\""),
    "light must use Default as parent scheme"
  );
}

// -- Canonical backgrounds --

#[test]
fn dark_background_is_canonical() {
  assert_eq!(jetbrains_attribute(DARK, "TEXT", "BACKGROUND"), "#1a1510");
}

#[test]
fn light_background_is_canonical() {
  assert_eq!(jetbrains_attribute(LIGHT, "TEXT", "BACKGROUND"), "#f5ede0");
}

// -- Canonical foregrounds --

#[test]
fn dark_foreground_is_canonical() {
  assert_eq!(jetbrains_attribute(DARK, "TEXT", "FOREGROUND"), "#bfbdb6");
}

#[test]
fn light_foreground_is_canonical() {
  assert_eq!(jetbrains_attribute(LIGHT, "TEXT", "FOREGROUND"), "#3a3630");
}

// -- Canonical cursor --

#[test]
fn dark_cursor_is_canonical() {
  assert_eq!(jetbrains_color(DARK, "CARET_COLOR"), "#f5c56e");
}

#[test]
fn light_cursor_is_canonical() {
  assert_eq!(jetbrains_color(LIGHT, "CARET_COLOR"), "#8a6600");
}

// -- No pure black/white backgrounds --

#[test]
fn no_pure_black_background() {
  let bg = jetbrains_attribute(DARK, "TEXT", "BACKGROUND");
  assert_ne!(bg, "#000000", "dark background must not be pure black");
}

#[test]
fn no_pure_white_background() {
  let bg = jetbrains_attribute(LIGHT, "TEXT", "BACKGROUND");
  assert_ne!(bg, "#ffffff", "light background must not be pure white");
}

// -- Dark syntax colors match canonical palette --

#[test]
fn dark_syntax_colors_match_palette() {
  let expected: &[(&str, &str, &str)] = &[
    ("TEXT", "FOREGROUND", "#bfbdb6"),
    ("DEFAULT_KEYWORD", "FOREGROUND", "#ff8f40"),
    ("DEFAULT_FUNCTION_DECLARATION", "FOREGROUND", "#ffb454"),
    ("DEFAULT_FUNCTION_CALL", "FOREGROUND", "#ffb454"),
    ("DEFAULT_CLASS_NAME", "FOREGROUND", "#8aa8b8"),
    ("DEFAULT_INTERFACE_NAME", "FOREGROUND", "#8aa8b8"),
    ("DEFAULT_STRING", "FOREGROUND", "#b4bc78"),
    ("DEFAULT_NUMBER", "FOREGROUND", "#d4a8b8"),
    ("DEFAULT_CONSTANT", "FOREGROUND", "#d4a8b8"),
    ("DEFAULT_OPERATION_SIGN", "FOREGROUND", "#f29668"),
    ("DEFAULT_LINE_COMMENT", "FOREGROUND", "#aea195"),
    ("DEFAULT_BLOCK_COMMENT", "FOREGROUND", "#aea195"),
    ("DEFAULT_DOC_COMMENT", "FOREGROUND", "#aea195"),
    ("DEFAULT_METADATA", "FOREGROUND", "#e6c08a"),
    ("DEFAULT_MARKUP_TAG", "FOREGROUND", "#d49484"),
    ("DEFAULT_INSTANCE_FIELD", "FOREGROUND", "#f58088"),
    ("DEFAULT_STATIC_METHOD", "FOREGROUND", "#f58088"),
    ("DEFAULT_VALID_STRING_ESCAPE", "FOREGROUND", "#96b898"),
    ("DEFAULT_INVALID_STRING_ESCAPE", "FOREGROUND", "#f08888"),
    ("CSS.PROPERTY_NAME", "FOREGROUND", "#deb074"),
    ("HTML_TAG_NAME", "FOREGROUND", "#d49484"),
  ];
  for (attr, prop, color) in expected {
    assert_eq!(
      jetbrains_attribute(DARK, attr, prop),
      *color,
      "dark {attr}.{prop} mismatch"
    );
  }
}

// -- Light syntax colors match canonical palette --

#[test]
fn light_syntax_colors_match_palette() {
  let expected: &[(&str, &str, &str)] = &[
    ("TEXT", "FOREGROUND", "#3a3630"),
    ("DEFAULT_KEYWORD", "FOREGROUND", "#924800"),
    ("DEFAULT_FUNCTION_DECLARATION", "FOREGROUND", "#855700"),
    ("DEFAULT_FUNCTION_CALL", "FOREGROUND", "#855700"),
    ("DEFAULT_CLASS_NAME", "FOREGROUND", "#2a5868"),
    ("DEFAULT_INTERFACE_NAME", "FOREGROUND", "#2a5868"),
    ("DEFAULT_STRING", "FOREGROUND", "#4d5c1a"),
    ("DEFAULT_NUMBER", "FOREGROUND", "#7e4060"),
    ("DEFAULT_CONSTANT", "FOREGROUND", "#7e4060"),
    ("DEFAULT_OPERATION_SIGN", "FOREGROUND", "#8f4418"),
    ("DEFAULT_LINE_COMMENT", "FOREGROUND", "#5a5244"),
    ("DEFAULT_BLOCK_COMMENT", "FOREGROUND", "#5a5244"),
    ("DEFAULT_DOC_COMMENT", "FOREGROUND", "#5a5244"),
    ("DEFAULT_METADATA", "FOREGROUND", "#7a5a1c"),
    ("DEFAULT_MARKUP_TAG", "FOREGROUND", "#8e4632"),
    ("DEFAULT_INSTANCE_FIELD", "FOREGROUND", "#a02838"),
    ("DEFAULT_STATIC_METHOD", "FOREGROUND", "#a02838"),
    ("DEFAULT_VALID_STRING_ESCAPE", "FOREGROUND", "#286a48"),
    ("DEFAULT_INVALID_STRING_ESCAPE", "FOREGROUND", "#b03434"),
    ("CSS.PROPERTY_NAME", "FOREGROUND", "#74501c"),
    ("HTML_TAG_NAME", "FOREGROUND", "#8e4632"),
  ];
  for (attr, prop, color) in expected {
    assert_eq!(
      jetbrains_attribute(LIGHT, attr, prop),
      *color,
      "light {attr}.{prop} mismatch"
    );
  }
}

// -- Font style verification --

#[test]
fn dark_bold_attributes() {
  let bold_attrs = ["DEFAULT_KEYWORD", "DEFAULT_DOC_COMMENT_TAG"];
  for attr in bold_attrs {
    assert_eq!(
      jetbrains_attribute(DARK, attr, "FONT_TYPE"),
      "1",
      "dark {attr} should be bold (FONT_TYPE=1)"
    );
  }
}

#[test]
fn dark_italic_attributes() {
  let italic_attrs = [
    "DEFAULT_CLASS_NAME",
    "DEFAULT_INTERFACE_NAME",
    "DEFAULT_LINE_COMMENT",
    "DEFAULT_BLOCK_COMMENT",
    "DEFAULT_DOC_COMMENT",
  ];
  for attr in italic_attrs {
    assert_eq!(
      jetbrains_attribute(DARK, attr, "FONT_TYPE"),
      "2",
      "dark {attr} should be italic (FONT_TYPE=2)"
    );
  }
}

#[test]
fn light_bold_attributes() {
  let bold_attrs = ["DEFAULT_KEYWORD", "DEFAULT_DOC_COMMENT_TAG"];
  for attr in bold_attrs {
    assert_eq!(
      jetbrains_attribute(LIGHT, attr, "FONT_TYPE"),
      "1",
      "light {attr} should be bold (FONT_TYPE=1)"
    );
  }
}

#[test]
fn light_italic_attributes() {
  let italic_attrs = [
    "DEFAULT_CLASS_NAME",
    "DEFAULT_INTERFACE_NAME",
    "DEFAULT_LINE_COMMENT",
    "DEFAULT_BLOCK_COMMENT",
    "DEFAULT_DOC_COMMENT",
  ];
  for attr in italic_attrs {
    assert_eq!(
      jetbrains_attribute(LIGHT, attr, "FONT_TYPE"),
      "2",
      "light {attr} should be italic (FONT_TYPE=2)"
    );
  }
}

// -- Structural parity: both variants have the same attribute names --

fn extract_attribute_names(src: &str) -> Vec<String> {
  let attrs_start = src.find("<attributes>").expect("missing <attributes>");
  let attrs_end = src.find("</attributes>").expect("missing </attributes>");
  let section = &src[attrs_start..attrs_end];
  section
    .lines()
    .filter_map(|line| {
      let trimmed = line.trim();
      // Top-level attribute options: <option name="..."> (not self-closing, no value=)
      if trimmed.starts_with("<option name=\"") && !trimmed.contains("value=\"") {
        let start = trimmed.find("name=\"").unwrap() + 6;
        let end = trimmed[start..].find('"').unwrap() + start;
        Some(trimmed[start..end].to_string())
      } else {
        None
      }
    })
    .collect()
}

fn extract_color_names(src: &str) -> Vec<String> {
  let colors_start = src.find("<colors>").expect("missing <colors>");
  let colors_end = src.find("</colors>").expect("missing </colors>");
  let section = &src[colors_start..colors_end];
  section
    .lines()
    .filter_map(|line| {
      let trimmed = line.trim();
      if trimmed.starts_with("<option name=\"") && trimmed.contains("value=\"") {
        let start = trimmed.find("name=\"").unwrap() + 6;
        let end = trimmed[start..].find('"').unwrap() + start;
        Some(trimmed[start..end].to_string())
      } else {
        None
      }
    })
    .collect()
}

#[test]
fn both_variants_have_same_attributes() {
  let dark_attrs = extract_attribute_names(DARK);
  let light_attrs = extract_attribute_names(LIGHT);

  for attr in &dark_attrs {
    assert!(
      light_attrs.contains(attr),
      "dark has attribute '{attr}' but light does not"
    );
  }
  for attr in &light_attrs {
    assert!(
      dark_attrs.contains(attr),
      "light has attribute '{attr}' but dark does not"
    );
  }
}

#[test]
fn both_variants_have_same_colors() {
  let dark_colors = extract_color_names(DARK);
  let light_colors = extract_color_names(LIGHT);

  for color in &dark_colors {
    assert!(
      light_colors.contains(color),
      "dark has color '{color}' but light does not"
    );
  }
  for color in &light_colors {
    assert!(
      dark_colors.contains(color),
      "light has color '{color}' but dark does not"
    );
  }
}

// -- .theme.json validation --

fn parse_theme(src: &str) -> JsonValue {
  serde_json::from_str(src).expect("invalid JSON in .theme.json")
}

#[test]
fn dark_theme_json_is_valid() {
  parse_theme(DARK_THEME);
}

#[test]
fn light_theme_json_is_valid() {
  parse_theme(LIGHT_THEME);
}

#[test]
fn dark_theme_json_name() {
  let v = parse_theme(DARK_THEME);
  assert_eq!(v["name"].as_str(), Some("Warm Burnout Dark"));
}

#[test]
fn light_theme_json_name() {
  let v = parse_theme(LIGHT_THEME);
  assert_eq!(v["name"].as_str(), Some("Warm Burnout Light"));
}

#[test]
fn dark_theme_json_is_dark() {
  let v = parse_theme(DARK_THEME);
  assert_eq!(v["dark"].as_bool(), Some(true));
}

#[test]
fn light_theme_json_is_light() {
  let v = parse_theme(LIGHT_THEME);
  assert_eq!(v["dark"].as_bool(), Some(false));
}

#[test]
fn dark_theme_json_references_icls() {
  let v = parse_theme(DARK_THEME);
  let scheme = v["editorScheme"].as_str().unwrap();
  assert!(
    scheme.contains("Warm Burnout Dark.icls"),
    "dark .theme.json must reference the dark .icls file"
  );
}

#[test]
fn light_theme_json_references_icls() {
  let v = parse_theme(LIGHT_THEME);
  let scheme = v["editorScheme"].as_str().unwrap();
  assert!(
    scheme.contains("Warm Burnout Light.icls"),
    "light .theme.json must reference the light .icls file"
  );
}

#[test]
fn dark_theme_json_has_ui_section() {
  let v = parse_theme(DARK_THEME);
  assert!(v["ui"].is_object(), "dark .theme.json must have 'ui' object");
}

#[test]
fn light_theme_json_has_ui_section() {
  let v = parse_theme(LIGHT_THEME);
  assert!(v["ui"].is_object(), "light .theme.json must have 'ui' object");
}

#[test]
fn dark_theme_json_has_required_ui_components() {
  let v = parse_theme(DARK_THEME);
  let ui = v["ui"].as_object().unwrap();
  let required = [
    "*",
    "Editor",
    "EditorTabs",
    "Panel",
    "Tree",
    "ToolWindow",
    "StatusBar",
    "Popup",
    "Menu",
  ];
  for key in required {
    assert!(ui.contains_key(key), "dark .theme.json missing UI component: {key}");
  }
}

#[test]
fn light_theme_json_has_required_ui_components() {
  let v = parse_theme(LIGHT_THEME);
  let ui = v["ui"].as_object().unwrap();
  let required = [
    "*",
    "Editor",
    "EditorTabs",
    "Panel",
    "Tree",
    "ToolWindow",
    "StatusBar",
    "Popup",
    "Menu",
  ];
  for key in required {
    assert!(ui.contains_key(key), "light .theme.json missing UI component: {key}");
  }
}

#[test]
fn theme_json_ui_keys_parity() {
  let dark = parse_theme(DARK_THEME);
  let light = parse_theme(LIGHT_THEME);
  let dark_keys: Vec<&str> = dark["ui"].as_object().unwrap().keys().map(String::as_str).collect();
  let light_keys: Vec<&str> = light["ui"].as_object().unwrap().keys().map(String::as_str).collect();

  for key in &dark_keys {
    assert!(
      light_keys.contains(key),
      "dark .theme.json has UI key '{key}' but light does not"
    );
  }
  for key in &light_keys {
    assert!(
      dark_keys.contains(key),
      "light .theme.json has UI key '{key}' but dark does not"
    );
  }
}

// -- Dark .theme.json uses canonical palette colors --

#[test]
fn dark_theme_json_editor_background() {
  let v = parse_theme(DARK_THEME);
  assert_eq!(v["ui"]["Editor"]["background"].as_str(), Some("1a1510"));
}

#[test]
fn dark_theme_json_sidebar_background() {
  let v = parse_theme(DARK_THEME);
  assert_eq!(v["ui"]["Panel"]["background"].as_str(), Some("14120f"));
}

#[test]
fn dark_theme_json_accent_color() {
  let v = parse_theme(DARK_THEME);
  assert_eq!(v["ui"]["EditorTabs"]["underlineColor"].as_str(), Some("b8522e"));
}

// -- Light .theme.json uses canonical palette colors --

#[test]
fn light_theme_json_editor_background() {
  let v = parse_theme(LIGHT_THEME);
  assert_eq!(v["ui"]["Editor"]["background"].as_str(), Some("f5ede0"));
}

#[test]
fn light_theme_json_sidebar_background() {
  let v = parse_theme(LIGHT_THEME);
  assert_eq!(v["ui"]["Panel"]["background"].as_str(), Some("ede6da"));
}

#[test]
fn light_theme_json_accent_color() {
  let v = parse_theme(LIGHT_THEME);
  assert_eq!(v["ui"]["EditorTabs"]["underlineColor"].as_str(), Some("b8522e"));
}

// -- plugin.xml validation --

#[test]
fn plugin_xml_has_id() {
  assert!(
    PLUGIN_XML.contains("<id>com.warm-burnout.theme</id>"),
    "plugin.xml must have plugin ID"
  );
}

#[test]
fn plugin_xml_has_name() {
  assert!(
    PLUGIN_XML.contains("<name>Warm Burnout</name>"),
    "plugin.xml must have plugin name"
  );
}

#[test]
fn plugin_xml_registers_dark_theme() {
  assert!(
    PLUGIN_XML.contains("Warm Burnout Dark.theme.json"),
    "plugin.xml must register dark theme"
  );
}

#[test]
fn plugin_xml_registers_light_theme() {
  assert!(
    PLUGIN_XML.contains("Warm Burnout Light.theme.json"),
    "plugin.xml must register light theme"
  );
}

// -- Editor scheme overrides prevent parent blue bleed --

#[test]
fn dark_search_result_uses_warm_highlight() {
  let bg = jetbrains_attribute(DARK, "SEARCH_RESULT_ATTRIBUTES", "BACKGROUND");
  assert_eq!(bg, "#4c4126", "dark search result should use warm gold highlight");
}

#[test]
fn light_search_result_uses_warm_highlight() {
  let bg = jetbrains_attribute(LIGHT, "SEARCH_RESULT_ATTRIBUTES", "BACKGROUND");
  assert_eq!(bg, "#e0c890", "light search result should use warm gold highlight");
}

#[test]
fn dark_has_breadcrumbs_override() {
  let fg = jetbrains_attribute(DARK, "BREADCRUMBS_DEFAULT", "FOREGROUND");
  assert_eq!(fg, "#aea195", "dark breadcrumbs should use warm stone color");
}

#[test]
fn light_has_breadcrumbs_override() {
  let fg = jetbrains_attribute(LIGHT, "BREADCRUMBS_DEFAULT", "FOREGROUND");
  assert_eq!(fg, "#5a5244", "light breadcrumbs should use warm stone color");
}

// -- Islands theme validation --

#[test]
fn dark_islands_is_valid_json() {
  parse_theme(DARK_ISLANDS);
}

#[test]
fn light_islands_is_valid_json() {
  parse_theme(LIGHT_ISLANDS);
}

#[test]
fn dark_islands_name() {
  let v = parse_theme(DARK_ISLANDS);
  assert_eq!(v["name"].as_str(), Some("Warm Burnout Islands Dark"));
}

#[test]
fn light_islands_name() {
  let v = parse_theme(LIGHT_ISLANDS);
  assert_eq!(v["name"].as_str(), Some("Warm Burnout Islands Light"));
}

#[test]
fn dark_islands_parent_theme() {
  let v = parse_theme(DARK_ISLANDS);
  assert_eq!(v["parentTheme"].as_str(), Some("Islands Dark"));
}

#[test]
fn light_islands_parent_theme() {
  let v = parse_theme(LIGHT_ISLANDS);
  assert_eq!(v["parentTheme"].as_str(), Some("Islands Light"));
}

#[test]
fn dark_islands_has_island_keys() {
  let v = parse_theme(DARK_ISLANDS);
  let island = &v["ui"]["Island"];
  assert!(island.is_object(), "dark Islands must have Island section");
  assert!(island["arc"].is_string(), "missing Island.arc");
  assert!(island["borderWidth"].is_string(), "missing Island.borderWidth");
  assert!(island["borderColor"].is_string(), "missing Island.borderColor");
}

#[test]
fn light_islands_has_island_keys() {
  let v = parse_theme(LIGHT_ISLANDS);
  let island = &v["ui"]["Island"];
  assert!(island.is_object(), "light Islands must have Island section");
  assert!(island["arc"].is_string(), "missing Island.arc");
  assert!(island["borderWidth"].is_string(), "missing Island.borderWidth");
  assert!(island["borderColor"].is_string(), "missing Island.borderColor");
}

#[test]
fn dark_islands_has_main_window_background() {
  let v = parse_theme(DARK_ISLANDS);
  assert!(
    v["ui"]["MainWindow"]["background"].is_string(),
    "dark Islands must have MainWindow.background (canvas color)"
  );
}

#[test]
fn light_islands_has_main_window_background() {
  let v = parse_theme(LIGHT_ISLANDS);
  assert!(
    v["ui"]["MainWindow"]["background"].is_string(),
    "light Islands must have MainWindow.background (canvas color)"
  );
}

#[test]
fn dark_islands_transparent_stripe_border() {
  let v = parse_theme(DARK_ISLANDS);
  let border = v["ui"]["ToolWindow"]["Stripe"]["borderColor"].as_str().unwrap();
  assert!(
    border.ends_with("00") || border == "00000000",
    "dark Islands ToolWindow.Stripe.borderColor must be transparent"
  );
}

#[test]
fn light_islands_transparent_stripe_border() {
  let v = parse_theme(LIGHT_ISLANDS);
  let border = v["ui"]["ToolWindow"]["Stripe"]["borderColor"].as_str().unwrap();
  assert!(
    border.ends_with("00") || border == "00000000",
    "light Islands ToolWindow.Stripe.borderColor must be transparent"
  );
}

#[test]
fn dark_islands_editor_background_matches_classic() {
  let classic = parse_theme(DARK_THEME);
  let islands = parse_theme(DARK_ISLANDS);
  assert_eq!(
    classic["ui"]["Editor"]["background"].as_str(),
    islands["ui"]["Editor"]["background"].as_str(),
    "dark Islands editor background must match classic"
  );
}

#[test]
fn light_islands_editor_background_matches_classic() {
  let classic = parse_theme(LIGHT_THEME);
  let islands = parse_theme(LIGHT_ISLANDS);
  assert_eq!(
    classic["ui"]["Editor"]["background"].as_str(),
    islands["ui"]["Editor"]["background"].as_str(),
    "light Islands editor background must match classic"
  );
}

#[test]
fn dark_islands_references_same_icls() {
  let classic = parse_theme(DARK_THEME);
  let islands = parse_theme(DARK_ISLANDS);
  assert_eq!(
    classic["editorScheme"].as_str(),
    islands["editorScheme"].as_str(),
    "dark Islands must reference same .icls as classic"
  );
}

#[test]
fn light_islands_references_same_icls() {
  let classic = parse_theme(LIGHT_THEME);
  let islands = parse_theme(LIGHT_ISLANDS);
  assert_eq!(
    classic["editorScheme"].as_str(),
    islands["editorScheme"].as_str(),
    "light Islands must reference same .icls as classic"
  );
}

#[test]
fn plugin_xml_registers_islands_dark() {
  assert!(
    PLUGIN_XML.contains("Warm Burnout Islands Dark.theme.json"),
    "plugin.xml must register Islands dark theme"
  );
}

#[test]
fn plugin_xml_registers_islands_light() {
  assert!(
    PLUGIN_XML.contains("Warm Burnout Islands Light.theme.json"),
    "plugin.xml must register Islands light theme"
  );
}
