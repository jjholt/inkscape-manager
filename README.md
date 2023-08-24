# Inkscape Manager
Attempts to create an overlay for inkscape that allows quickly applying commonly used styles.

In this example, `av` sets a colour (fill) to black, then and `ab` sets it to white. `sv` creates a stroke that is coloured black.

https://github.com/jjholt/inkscape-manager/assets/876097/ed27480b-4620-4046-ac17-85e6c08fe1d7


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

Keybinds are either styles or rebinds:

```yaml
keybinds:
- key: a
  style: fill
- key: s # Stroke + constants
  style: stroke;stroke-width:2.6;stroke-dasharray:15.9,2.6;stroke-opacity:1;
- key: b # Colour with white
  style: :#ffffff
- key: v
  style: :#000000
- key: g
  style: opacity:0.5

# Rebinds
- key: w
  rebind_to: e
- key: e
  rebind_to: w
```

