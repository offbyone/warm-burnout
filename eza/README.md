# Warm Burnout for eza

Your `ls` replacement was still painting directories in searing blue. Not anymore.

## Requirements

- eza v0.19.2 or later (theme.yml support)
- A truecolor terminal

## Install

Copy the theme file to your eza config directory:

```sh
mkdir -p ~/.config/eza
cp theme.yml ~/.config/eza/theme.yml
```

Then add to your `.zshrc` (or `.bashrc`):

```sh
export EZA_CONFIG_DIR="$HOME/.config/eza"
```

Restart your shell.

## What it covers

The theme maps the full Warm Burnout palette to every eza element:

| Element | Color | Hex |
|---------|-------|-----|
| Directories | Aged brass | `#deb074` |
| Symlinks | Verdigris | `#96b898` |
| Executables | Dried sage | `#b4bc78` |
| Mount points | Burnt orange | `#ff8f40` |
| Dates | Warm stone | `#aea195` |
| User (you) | Gold | `#e6c08a` |
| Permissions read | Gold | `#e6c08a` |
| Permissions write | Coral | `#f58088` |
| Permissions execute | Dried sage | `#b4bc78` |
| Size (small) | Foreground | `#bfbdb6` |
| Size (large) | Amber | `#ffb454` |
| Git new | Dried sage | `#b4bc78` |
| Git modified | Amber | `#ffb454` |
| Git deleted | Coral | `#f58088` |
| Source files | Steel patina | `#8aa8b8` |
| Errors | Error | `#f08888` |
| Punctuation | Warm stone | `#aea195` |

## Verify

Run `ll` or `eza -la`. Warm tones everywhere, no blue.

## Palette

Derives from the canonical Warm Burnout palette defined in the root [`AGENTS.md`](../AGENTS.md).
