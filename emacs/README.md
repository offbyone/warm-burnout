# Warm Burnout for Emacs

Your editor was also burning your retinas. Now it matches the rest of the damage.

## Install

### Manual

Clone the repo and add the `emacs/` directory to your load paths:

```elisp
(add-to-list 'load-path "/path/to/warm-burnout/emacs")
(add-to-list 'custom-theme-load-path "/path/to/warm-burnout/emacs")
(load-theme 'warm-burnout-dark t)
```

### use-package + straight.el

```elisp
(use-package warm-burnout
  :straight (:host github :repo "felipefdl/warm-burnout" :files ("emacs/*.el"))
  :config
  (load-theme 'warm-burnout-dark t))
```

### Doom Emacs

In `packages.el`:

```elisp
(package! warm-burnout
  :recipe (:host github :repo "felipefdl/warm-burnout" :files ("emacs/*.el")))
```

In `config.el`:

```elisp
(setq doom-theme 'warm-burnout-dark)
```

### Spacemacs

In `dotspacemacs-additional-packages`:

```elisp
(warm-burnout :location (recipe :fetcher github
                                :repo "felipefdl/warm-burnout"
                                :files ("emacs/*.el")))
```

Then set `dotspacemacs-themes '(warm-burnout-dark)`.

## Variants

- `warm-burnout-dark` — AAA contrast (>= 7.0:1), warm dark background
- `warm-burnout-light` — AA contrast (>= 4.5:1), warm parchment background

Switch with `M-x load-theme`.

## Supported Packages

Font-lock, tree-sitter, org-mode, magit, markdown-mode, company, corfu, vertico, consult, marginalia, which-key, flycheck, flymake, lsp-mode, diff-hl, rainbow-delimiters, dired, compilation, show-paren, tab-bar, whitespace-mode.

Doom-themes integration activates automatically when `doom-themes` is loaded.

## Palette

See the root [AGENTS.md](../AGENTS.md) for the canonical palette tables. Every hex value in these theme files comes directly from there.
