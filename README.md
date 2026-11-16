# Free Epic Games Reminder

A Windows reminder that checks the Epic Games Store and opens currently free games in your browser.

## Current version

The current implementation is written in **Rust** and is located in `src/main.rs`.
Older Python and C implementations are kept in `legacy/` for reference.

## Download

Download the latest Windows executable from the [Releases](https://github.com/TheDeviler/FreeEpicGames/releases) page,
or download the [v2.0.0 Rust executable](dist/free-epic-games-reminder-v2.0.0-windows-x86_64.exe) directly.

To start the reminder automatically, place the downloaded executable in:

```text
C:\Users\YOURUSERNAME\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup
```

The executable targets Windows 10 and Windows 11.

## Build the Rust version

### Requirements

- Rust and Cargo
- An internet connection

### Run locally

```bash
cargo run
```

### Build a release executable

```bash
cargo build --release
```

The executable is created at `target/release/free-epic-games-reminder.exe`
when built for Windows.

## Repository layout

```text
src/main.rs              Current Rust implementation
legacy/python/           Previous Python implementation
legacy/c/                Previous C implementation
legacy/windows/          Previous Windows executables
```

## FAQ

### Why use this reminder?

It helps you remember to claim the free games released by Epic Games.

### Why claim free games?

Because having more games than your friends is always useful.

## License

See [LICENSE](LICENSE).
