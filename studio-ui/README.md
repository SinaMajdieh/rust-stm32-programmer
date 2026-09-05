# STM32 Studio UI

A polished, Rust-native desktop UI shell for an STM32 firmware-generation and programming tool.

The design follows one idea: **Prompt → Generate → Build → Program → Ready**. It intentionally avoids looking like a chatbot or a full IDE. The hardware is always visible, the task prompt is the main interaction, and diagnostics stay available without dominating the screen.

## Why Slint

The UI is written in Slint and compiled into Rust with `slint-build`. Slint supports external `.slint` files, reusable components, exported globals, and generated Rust handles/callbacks. The current project uses those facilities so the visual layer stays declarative while the backend remains ordinary Rust. citeturn849305search0turn849305search2turn849305search5

## Project structure

```text
stm32-studio-ui/
├── Cargo.toml
├── build.rs
├── README.md
├── src/
│   ├── main.rs
│   └── backend.rs
└── ui/
    ├── app-window.slint          # root window + backend-facing API
    ├── theme.slint               # all colors, typography, radii
    └── components/
        ├── app-header.slint
        ├── code-view.slint
        ├── icon-button.slint
        ├── landing.slint
        ├── output-panel.slint
        ├── pipeline.slint
        ├── primary-button.slint
        ├── project-sidebar.slint
        ├── prompt-box.slint
        ├── recent-projects.slint
        ├── status-bar.slint
        ├── status-dot.slint
        └── workspace.slint
```

Every UI component is deliberately small. `app-window.slint` composes them; individual components should only know about their own presentation and callbacks.

## 1. Install the Rust toolchain

Install current stable Rust/Cargo using your normal Rust installation method.

Then, from this directory:

```bash
cargo run
```

The application opens immediately. Type a task such as:

```text
Make the onboard LED blink every 500ms
```

and press **Generate & Flash**. The included `DemoBackend` simulates the whole workflow so you can evaluate the UI without connecting a board.

## 2. Linux dependencies

Slint's standard desktop Winit backend supports Linux on X11 and Wayland. On Debian-based systems, the Slint documentation lists the X11 development dependencies below; the exact runtime libraries depend on the desktop environment and backend selected. citeturn571416search0

```bash
sudo apt update
sudo apt install -y \
    build-essential \
    pkg-config \
    libx11-xcb-dev \
    xinput \
    libxcursor-dev \
    libxkbcommon-x11-dev \
    libx11-dev
```

For Wayland-oriented development, use the Wayland development/runtime packages provided by your distribution. Slint can also be configured with a specific backend/renderer through its documented backend features and `SLINT_BACKEND`. citeturn571416search0turn571416search5

## 3. Better Slint development experience

Install the Slint language server and viewer:

```bash
cargo install slint-lsp
cargo install slint-viewer
```

The official viewer can preview `.slint` files and supports live-reload style iteration; the language server provides editor integration. citeturn571416search1turn571416search2

## 4. Fonts

The theme requests:

```text
UI:   Inter
Code: JetBrains Mono
```

Slint uses system-installed fonts when you specify a family name. It also supports embedding TTF/TTC/OTF files directly in `.slint` files, which is preferable for deterministic releases. citeturn513259search2

For development on Ubuntu, install your preferred fonts through the distribution or place the font files in the project and then import them from `ui/theme.slint`.

## 5. Changing the look

The only file you should normally edit for global visual changes is:

```text
ui/theme.slint
```

It contains semantic design tokens:

```text
Theme.background
Theme.surface
Theme.surface-2
Theme.surface-3
Theme.border
Theme.text
Theme.text-secondary
Theme.text-muted
Theme.accent
Theme.success
Theme.warning
Theme.error
Theme.font-ui
Theme.font-mono
Theme.radius-small
Theme.radius-medium
Theme.radius-large
```

Change those values and the entire interface follows the new visual language. This is intentional: component files should describe *what* something is, not contain scattered magic colors.

## 6. Connecting the real backend

The root `AppWindow` exposes only the UI state that the production Rust application needs:

```text
prompt                 current user task
has-project            landing vs generated-project workspace
busy                   disable UI while pipeline is active
pipeline-stage         0 ready, 1 generating, 2 building, 3 programming, 4 complete, 5 error
pipeline-progress      active stage progress, 0..100
output-open            diagnostics drawer visibility
status-message         human-readable status
```

and these callbacks:

```text
generate-and-flash()
prompt-submitted()
new-project()
device-settings()
toggle-output()
```

Your existing architecture should sit behind these events. Do not move OpenOCD, GCC, Ollama, target selection, or project-template logic into `.slint` files.

A clean production shape is:

```text
Slint UI
   │
   │ callbacks / properties
   ▼
Application controller
   │
   ├── Generator / Ollama
   ├── Project generator
   ├── Builder / GCC
   ├── Programmer / OpenOCD
   └── Device manager
```

### Keep long-running work off the UI thread

Slint's Rust documentation explicitly recommends doing minimal work on the UI/event thread and sending results back with `invoke_from_event_loop`. citeturn849305search0

For the real application:

```text
Generate callback
      ↓
spawn worker/task
      ↓
Generator → project files → compiler → OpenOCD
      ↓
worker emits progress/events
      ↓
slint::invoke_from_event_loop(...)
      ↓
AppWindow setters
```

That keeps animations and input responsive while external processes are running.

## 7. Code editor strategy

`CodeView` is intentionally a read-friendly first implementation rather than a full IDE editor. Slint provides `TextEdit` and `ScrollView`, so the prompt and diagnostic surfaces are already interactive. citeturn513259search7turn461412search4

For a production editor, replace only `ui/components/code-view.slint` with the editor implementation you choose. Do not redesign the workspace around the editor.

## 8. Recommended next production step

Keep the current visual structure, then replace only these demo pieces:

```text
DemoBackend
    ↓
real application controller

hard-coded device
    ↓
device manager

hard-coded project tree
    ↓
project/file model

hard-coded code text
    ↓
generated project files
```

The visual hierarchy can remain almost unchanged as those backend pieces become real.
