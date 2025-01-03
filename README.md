# 🔊 vrc-osc-mpris

A music status for VRChat using MPRIS (Linux) + OSC.

## Features

* Program selection
* Small chat bubble
* Configurable OSC host

## Installation

1. Clone the repository locally.

```sh
git clone https://github.com/bunboop/vrc-osc-mpris.git \
cd vrc-osc-mpris
```

2. Build the project.

```sh
cargo build --release
```

3. Run the project.

```sh
./target/release/vrc-osc-mpris
```

4. Open any MPRIS-supported application, examples:

* [Amberol](https://apps.gnome.org/Amberol/)
* [Spotify](https://flathub.org/apps/com.spotify.Client)
* [Firefox](https://flathub.org/apps/org.mozilla.firefox)

5. Run VRChat.

If something is playing, you should see a chat bubble displaying the current track above your head.
