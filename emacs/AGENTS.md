# Emacs -- Agent Instructions

## Platform Reference

See the root [`AGENTS.md`](../AGENTS.md) for the canonical palette, design principles, and brand rules. Do not duplicate palette tables here.

## Emacs Theme Format

- Emacs themes use `deftheme` in files named `<theme-name>-theme.el`.
- The file must `provide-theme` with the exact theme symbol.
- Shared code lives in `warm-burnout.el` (not a theme file, just a library).
- Requires Emacs 29.1+ for `custom-theme-set-faces` improvements and lexical binding.

## Structure

```
emacs/
  warm-burnout.el                 -- Shared palette definitions, doom integration
  warm-burnout-dark-theme.el      -- Dark variant (deftheme)
  warm-burnout-light-theme.el     -- Light variant (deftheme)
  README.md                       -- Install instructions
  AGENTS.md                       -- This file
```

## Palette Mapping

- `warm-burnout.el` defines `warm-burnout-dark-palette` and `warm-burnout-light-palette` as alists.
- Theme files use `warm-burnout--with-palette` macro to bind all colors as local variables.
- All hex values come directly from the canonical palette in root `AGENTS.md`.

## Font Style System

- `bold` = keywords, storage, preprocessor, tags (HTML)
- `italic` = types, comments, doc strings, decorators
- normal = everything else

## Adding Package Support

1. Add face specs inside the `custom-theme-set-faces` form in both theme files.
2. Use palette variable names, not raw hex values.
3. Follow existing patterns for foreground/background/style assignment.
4. Keep both dark and light variants in sync (same faces, different palette).

## Doom Themes Integration

When `doom-themes` is loaded, `warm-burnout.el` registers face overrides via `doom-themes-set-faces`. This allows solaire-mode and other doom extensions to work correctly. The integration is passive (only activates if doom-themes is present).

## Color Rules

1. Use exact hex values from the canonical palette. No approximations.
2. Region/selection uses opaque blends since Emacs face backgrounds don't support alpha.
3. Diff backgrounds use pre-blended opaque colors (alpha composited over the theme background). Emacs only supports 3/6/9/12-digit hex, not #RRGGBBAA.
