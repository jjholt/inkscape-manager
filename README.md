# Inkscape Manager
Attempts to create an overlay for inkscape that allows quickly applying commonly used styles

# Configuration
The most important file is `config.yaml`, which outlines which keys are simple rebinds and which keys are styles.

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

## Creating your own
A style conditionally contains a property and a value. When either of them is absent, that style is expected to be paired with another to do something useful.
That is, to colour a shape in black, you would need 2 clicks: the one that `fill`s and the one that sets colour to `#000000`. 
Internally this creates `fill:#000000`, giving you the flexibility to assign sets of keys for colours, shapes, changing properties, etc.

An empty property is written `:value`, and an empty value is `property:` (though the `:` can be omitted).
Often it is useful to have a set of standard other styles that go with one option. For example when defining a stroked line, you may want to define width, opacity, etc and only change the colour.
That can be easily done:

```yaml
# ...
style: stroke;stroke-width:2.6;
# ...
style: :#ffffff
```
