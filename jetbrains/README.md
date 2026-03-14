# Warm Burnout for JetBrains

Your IDE was also incinerating your retinas. The burnout is spreading to every JetBrains product.

Full UI theme, not just syntax colors. Sidebar, tabs, toolbar, popups, everything. Works in IntelliJ IDEA, Android Studio, WebStorm, PyCharm, GoLand, RustRover, and every other JetBrains IDE.

## Install

### Option A: Build and Install Plugin

```sh
cd jetbrains
./build.sh
```

Then in your IDE: **Settings** > **Plugins** > gear icon > **Install Plugin from Disk...** > select `warm-burnout-theme.jar`.

### Option B: Editor Scheme Only

If you only want the editor color scheme without full UI theming, copy the `.icls` files directly:

```sh
# Replace <Product> and <Version> with your IDE (e.g., IntelliJIdea2025.1)
cp "Warm Burnout Dark.icls" "Warm Burnout Light.icls" \
  ~/.config/JetBrains/<Product><Version>/colors/
```

Then select the scheme in **Settings** > **Editor** > **Color Scheme**.

## Configure

After installing the plugin, go to **Settings** > **Appearance & Behavior** > **Appearance**, then select **Warm Burnout Dark** or **Warm Burnout Light** from the Theme dropdown.

## Verify

Open any source file. Keywords should be bold burnt orange, types should be italic steel patina, and the entire IDE (sidebar, tabs, popups) should be warm.

## Palette

Both themes derive from the canonical Warm Burnout palette defined in the root [`AGENTS.md`](../AGENTS.md). Every syntax token and UI element maps directly from the canonical hex values.
