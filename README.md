# sway-thumbwheel-proxy
A daemon to proxy events triggered by mouse thumbwheels in order to debounce rapid events.
Should be compatible with i3 as well, but not tested.

## Motivation
The original motivation was my desire to use the thumbwheel of the MX Master 3S in order to scroll through sway workspaces (`workspace prev` and `workspace next`).
Binding the thumbwheel motions to key presses (which in turn, are bound to sway commands in the sway config) was possible using the great [logiops](https://github.com/PixlOne/logiops) project.
However, events triggered too often when scrolling normally and increasing the `interval` option in the logiops config led to decreased responsiveness.
This repository provides server and client binaries which proxy workspace commands to debounce them within a configurable interval.

Server and client communicate via Unix Domain Sockets.

## Example Usage
### Server & Client
The server and client binaries can only be configured via CLI arguments (see `./server -h` and `./client -h` for reference):
- **`--debounce-millis <millis>`**: Milliseconds during which to debounce incoming messages. Default: `200`
- **`--socket-path <path>`**: File path at which the IPC socket will live. Default: `/tmp/sway-thumbwheel-proxy`

### sway `config`
You need the paths of the binaries so note them down.

```i3
set $mod Mod4

# Run server in the background
exec_always /path/to/server/binary

bindsym $mod+Prior exec /path/to/client/binary prev
bindsym $mod+Next exec /path/to/client/binary next
```

### `logid.cfg`
Obviously, you don't need to use logiops in order to use the daemon.
Nonetheless, for reference, here is my setup for the thumbwheel on the MX Master 3S.

```libconfig
thumbwheel:
{
    divert: true;
    invert: false;

    left: {
            mode: "OnInterval";
            interval: 2;
            action: { 
                    type: "Keypress"
                    keys: ["KEY_LEFTMETA", "KEY_PAGEUP"]
            }
    };
    right: {
            mode: "OnInterval"
            interval: 2;
            action: { 
                    type: "Keypress"
                    keys: ["KEY_LEFTMETA", "KEY_PAGEDOWN"]
            }
    };
};
```

## Supported sway Commands
Currently supported:
- `swaymsg workspace prev` => `client prev`
- `swaymsg workspace next` => `client next`
- `swaymsg workspace prev_on_output` => `client prev_on_output`
- `swaymsg workspace next_on_output` => `client next_on_output`
