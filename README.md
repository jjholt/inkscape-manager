# Inkscape Manager
Attempts to create an overlay for inkscape that allows quickly applying commonly used styles.

In this example, `ab` sets a colour (fill) to black, `av` sets it to white. `s` alone creates a stroke style, but it can be paired with a colour (`sb`) to create a black stroke.

https://github.com/jjholt/inkscape-manager/assets/876097/48d0435c-9a33-42e2-ac91-1f14147acaa7


#Dependencies
Clipboard management to apply styles uses `xclip`.

Written for `X11` and only tested using one window manager: `i3`. A rewrite for Wayland might happen eventually.

# Keybinds
A style consists of a list of property:value pairs. If a property or value is missing in a style, pressing that key will wait for another key before doing anything.

In this example, `stroke` is missing a value. Pressing `s` alone would do nothing until it was paired with `b`. It would then create `stroke:#ffffff`.

```yaml
- key: s
  style: stroke;stroke-width:2.6;
- key: b
  style: :#ffffff
```

## Creating your own
If you want to change default values, modify `config.yaml`.

Keybinds are either styles or rebinds. Rebinds are really straightforward, but styles require you to know what SVG styles look like.

The `examples/` folder contains some examples. You might find useful to either use the XML inspector in inkscape or save an image as `.svg`, then find the section that says `styles:` to create exactly what you need.

```yaml
keybinds:
- key: a
  style: fill
- key: d # Solid line
  style: stroke;stroke-dasharray:none;-inkscape-stroke:none
- key: s # Stroked line
  style: stroke;stroke-dasharray:22.67720315,3.77952756;stroke-opacity:1;
- key: v # Paint white
  style: :#ffffff
- key: b # Paint black
  style: :#000000
- key: c # Removes fill, for example.
  style: :none
- key: g # Add an arrow header. #ConcaveTriangle is hardcoded in the styler.
  style: marker-end:url(#ConcaveTriangle);

# Rebinds
- key: w
  rebind_to: e
- key: e
  rebind_to: w

```
