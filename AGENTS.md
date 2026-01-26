# AGENTS.md

## Project Overview

**Servo** is a parallel web browser engine written in Rust, designed for modern hardware with a focus on concurrency and performance.

- **Language**: Rust 1.92.0 (edition 2024, minimum 1.86.0)
- **Architecture**: Parallel processing with separate ScriptThreads and LayoutThreads
- **Platform**: Running on Raspberry Pi 5 (aarch64-unknown-linux-gnu, ARM64)
- **Location**: `/mnt/nvme/servo` (on NVMe SSD)
- **Key Feature**: Headless mode support for CI/automated testing

---

## Getting Help

**IMPORTANT**: For doubts, confusion, or Root Cause Analysis (RCA), use the Gemini CLI as an oracle:

```bash
gemini -m gemini-3-pro-preview --include-directories=/mnt/nvme/servo "Your question or problem description"
```

Gemini CLI is installed at: `/home/brb/.nvm/versions/node/v25.2.1/bin/gemini`

---

## Repository Structure

```
/mnt/nvme/servo/
├── ports/servoshell/        # Browser shell application (entry point)
│   ├── desktop/cli.rs         # Command-line argument parsing
│   ├── desktop/event_loop.rs  # Main event loop (headed & headless)
│   └── webdriver.rs          # WebDriver integration
├── components/               # Core browser components
│   ├── servo/               # Main entry point, initializes Constellation
│   ├── constellation/         # Central coordinator (pipeline manager)
│   ├── script/               # DOM & JavaScript execution (SpiderMonkey)
│   ├── layout/               # Parallel layout engine
│   ├── net/                 # Networking stack
│   └── webdriver_server/     # W3C WebDriver protocol implementation
├── tests/                   # All test suites
│   ├── unit/                # Cargo unit tests
│   ├── wpt/                 # Web Platform Tests (285+ directories)
│   │   ├── config.ini         # WPT configuration
│   │   └── include.ini        # Test inclusion rules
│   ├── dromaeo/             # Performance tests
│   └── html/                # HTML test pages
└── python/servo/             # Mach build system commands
```

**Architecture Pattern**:
- **Constellation**: Manages pipeline of ScriptThreads and LayoutThreads via IPC channels
- **Embedder Interface**: `components/servo/lib.rs` provides Embedder traits implemented by servoshell
- **Parallel Rendering**: ScriptThread (JS execution) and LayoutThread (reflow) run in parallel
- **Communication**: Uses `ipc-channel` crate for thread communication

---

## Build System & Compilation

Servo uses **Mach** (Python-based) that orchestrates Cargo and other build steps.

### Basic Commands

```bash
# Build release version (default for performance)
./mach build
./mach build --release

# Build for development (faster compilation, slower runtime)
./mach build --dev

# Build fully optimized production build
./mach build --prod
./mach build --production

# Clean build artifacts
./mach clean
```

### Build Configuration

**Profiles** (from `Cargo.toml` workspace):
- **`--release`**: Default, includes debug assertions (`debug-assertions = true`)
- **`--dev`**: Faster compile, optimized for development iteration
- **`--production`**: Stripped binary, LTO enabled, minimal debugging info

**Feature Flags**:
```bash
# Enable experimental features
./mach build --features layout_variable_fonts_enabled,dom_webgpu_enabled

# Use specific media stack
./mach build --media-stack gstreamer  # Linux default
./mach build --media-stack dummy
```

### Cross-Compilation for ARM64

Servo supports `aarch64-unknown-linux-gnu` target (RPi5 native).

**Dependencies**:
```bash
# Bootstrap ensures all system deps are installed
./mach bootstrap

# On RPi5/ARM64, ensure these are present:
- libssl-dev (OpenSSL)
- gstreamer1.0-plugins-base
- gstreamer1.0-plugins-good
- libclang-dev
```

**Build Tips for RPi5**:
```bash
# Limit parallel linker jobs to avoid OOM (8GB RAM constraint)
./mach build -j 4

# Monitor thermal (throttling starts at ~80°C)
vcgencmd measure_temp

# If linking fails with OOM, reduce to -j 2
```

**Workspace** (from `Cargo.toml`):
- Workspace includes: `components/xpath`, `ports/servoshell`, `tests/unit/*`
- Key dependency: `mozjs` (SpiderMonkey JavaScript engine)
- Graphics: `webrender` (GPU) and software fallbacks

---

## Running Servo

### Basic Execution

```bash
# Run with URL
./mach run https://servo.org

# Run in headless mode (no window)
./mach run --headless https://servo.org

# Run with software rendering (Linux only)
./mach run --software https://servo.org

# Run with output image capture
./mach run --headless --output render.png https://servo.org
```

### Command-Line Arguments

**Passed after `./mach run [args]`:

- `--headless, -z`: Launch in headless mode
- `--software, -s`: Use software rendering
- `--devtools <PORT>`: Enable remote devtools (e.g., `--devtools 6080`)
- `--userscripts <DIR>`: Load user scripts from directory
- `--debug <OPTIONS>`: Enable debug flags (e.g., `--debug layout_grid_enabled=true`)
- `--pref <PREF>`: Set preference (e.g., `--pref network.http.proxy_uri=`)
- `--webdriver <PORT>`: Start WebDriver server on specified port
- `--output <PATH>`: Output rendered image to file

**Preferences** (from `ports/servoshell/prefs.rs`):
- **Experimental features**: `dom_async_clipboard_enabled`, `dom_webgl2_enabled`, `dom_webgpu_enabled`, `layout_grid_enabled`
- Set via command line: `--pref dom.webgpu.enabled=true`

### Debugging

```bash
# Enable backtrace for panics
RUST_BACKTRACE=1 ./mach run https://example.com

# Component-specific logging
RUST_LOG=servo::constellation=debug ./mach run https://example.com
RUST_LOG=script=debug ./mach run https://example.com

# Enable multiple components
RUST_LOG=servo::constellation=debug,servo::script=info ./mach run
```

---

## Testing Infrastructure

### Unit Tests

Uses `cargo nextest` for running Rust tests.

```bash
# Run all unit tests
./mach test-unit

# Run specific package
./mach test-unit -p script
./mach test-unit -p layout

# Run specific test file
./mach test-unit tests/unit/script/test_file.rs

# Run in release mode
./mach test-unit --release

# Run with code coverage
./mach test-unit --code-coverage
```

**Test Structure** (`/mnt/nvme/servo/tests/unit/`):
- `script/` - JavaScript and DOM tests
- `style/` - CSS parsing and styling tests
- `profile/` - Profiling tests
- `malloc_size_of/` - Memory tracking tests

**Test Patterns**:
```rust
#[test]
fn test_example() {
    // Arrange
    let input = "...";

    // Act
    let result = process(input);

    // Assert
    assert_eq!(result, expected);
}
```

### Web Platform Tests (WPT)

**CRITICAL**: WPT tests run in **headless mode by default**.

```bash
# Run all WPT tests
./mach test-wpt

# Run specific test
./mach test-wpt dom/historical.html

# Run multiple tests
./mach test-wpt css/css-box/*.html

# Run with preferences
./mach test-wpt --pref dom.webgpu.enabled=true

# Update expectations
./mach test-wpt --manifest-update

# Disable headless (rare, for debugging)
./mach test-wpt --no-headless
```

**WPT Configuration**:
- `tests/wpt/config.ini`: Product definitions (servo, servodriver)
- `tests/wpt/include.ini`: Test inclusion/exclusion rules (345 lines)
- `tests/wpt/hosts`: DNS host mappings (maps test domains to 127.0.0.1)
- Test types: `testharness`, `reftest`, `wdspec`, `crashtest`

**Test Types**:
- **testharness**: JavaScript-based unit tests for web standards
- **reftest**: Reference tests comparing rendered output
- **wdspec**: WebDriver protocol conformance tests
- **crashtest**: Tests that shouldn't crash the browser

### Other Test Suites

```bash
# Smoke test (load simple page and close cleanly)
./mach smoketest

# Tidy/Format checks
./mach test-tidy

# Lint checks only (no auto-fix)
./mach fmt --check

# Format code
./mach fmt

# Performance benchmarks
./mach test-dromaeo
./mach test-speedometer
```

### Testing Workflow

```bash
# Before committing
./mach test-tidy          # Check formatting
./mach test-unit          # Run unit tests
./mach smoketest          # Quick sanity check

# After major changes
./mach test-wpt dom/      # Run affected WPT tests

# For performance changes
./mach test-speedometer --bmf-output results.json
```

---

## Headless Mode

**Headless mode** allows Servo to run without a visible window, essential for CI, automated testing, and server environments.

### How It Works

**Implementation**: `ports/servoshell/desktop/event_loop.rs`
- Uses `HeadlessEventLoop` when `--headless` flag is present
- Bypasses `winit` window creation or uses offscreen context
- Image output is supported for verification

### Running Headless

```bash
# Basic headless run
./mach run --headless https://servo.org

# Headless with image capture
./mach run --headless --output screenshot.png https://servo.org

# Headless with WebDriver
./mach run --headless --webdriver=7000 https://servo.org

# Headless with preferences
./mach run --headless --pref network.http.proxy_uri= https://example.com
```

### WPT Headless Integration

WPT runner automatically enables headless mode:
- Default behavior: `headless=True` in `python/wpt/run.py` (line 114-117)
- Override: `./mach test-wpt --no-headless`
- Binary receives: `--headless` flag via `servodriver` product

**Key Files**:
- `/mnt/nvme/servo/python/wpt/run.py` - WPT test runner
- `/mnt/nvme/servo/tests/wpt/tests/tools/wptrunner/wptrunner/browsers/servodriver.py` - Servo WebDriver browser interface

### Headless Use Cases

1. **CI/CD**: Automated testing without display server
2. **Performance Testing**: Measure rendering performance without UI overhead
3. **Screenshot Generation**: Programmatically capture page renders
4. **WebDriver Automation**: Remote browser control for E2E tests
5. **Server Environments**: Run on headless Linux servers

---

## Raspberry Pi 5 Specific Considerations

### Hardware Context

From `/home/brb/base/`:
- **CPU**: ARM Cortex-A76 (quad-core), 1.5 GHz base, up to 2.4 GHz turbo
- **RAM**: 8 GB LPDDR4-3200 with 2 GB zram swap
- **Architecture**: aarch64 (ARM64)
- **Thermal**: Throttling begins at ~80°C, current ~47°C under light load
- **Storage**: 128 GB SD card (root) + 256 GB NVMe SSD (mounted at `/mnt/nvme/servo`)

### ARM64 Build Optimizations

```bash
# Target triple (if cross-compiling, though RPi5 is native)
export CARGO_BUILD_TARGET=aarch64-unknown-linux-gnu

# Limit parallel jobs for memory constraints
./mach build -j 4  # Safe default for 8GB RAM
./mach build -j 2  # If OOM occurs

# Use dev profile for faster iteration
./mach build --dev
```

### Performance Tips

1. **Compilation Time**: Rust builds are long; expect 30+ minutes for `--release` on RPi5
2. **Thermal Management**: Monitor temperature during long builds; active cooling recommended
3. **Memory Usage**: Full LTO builds (production profile) may OOM; use `--release` instead
4. **I/O Performance**: NVMe SSD significantly speeds up linking and incremental builds

### ARM-Specific Considerations

- **Crypto Extensions**: ARM crypto extensions enabled (SHA256: 1.49 GB/s)
- **WebGL**: May need software fallback (`llvmpipe`) if hardware acceleration fails
- **ASAN/TSAN**: Supported for aarch64 target
- **Endianness**: Little-endian (standard for ARM64)

---

## WebDriver & Automation

### Built-in WebDriver Server

Servo implements W3C WebDriver protocol in `components/webdriver_server`.

### Starting WebDriver

```bash
# Start WebDriver server on port 7000
./mach run --webdriver=7000 --headless

# With preferences
./mach run --webdriver=7000 --headless --pref dom.webgpu.enabled=true
```

### WebDriver Endpoints

- Standard W3C endpoints: `/session`, `/session/{id}/url`, `/session/{id}/execute/sync`
- Servo-specific extensions: `/session/{id}/servo/prefs/set`

### WPT Integration

**Product**: `servodriver` (WebDriver-based)

```bash
# WPT uses servodriver automatically
./mach test-wpt

# Equivalent to running:
servodriver --binary=./target/release/servo --headless
```

### Automation Patterns

```bash
# Start headless Servo with WebDriver
./mach run --headless --webdriver=4444

# Use Selenium/WebDriver client to connect
# Python example:
from selenium import webdriver

driver = webdriver.Remote(
    command_executor='http://localhost:4444',
    desired_capabilities={'browserName': 'servodriver'}
)

driver.get('https://servo.org')
```

---

## Code Style & Conventions

### Rust Standards

- **Edition**: Rust 2024
- **Version**: Minimum 1.86.0, locked to 1.92.0 in `Cargo.toml`
- **Unsafe Code**: `deny(unsafe_code)` used in many modules; `unsafe` requires justification
- **Error Handling**: Use `Result<T, E>` and `?` operator; `anyhow` for contextful errors

### Formatting

```bash
# Check formatting
./mach fmt --check

# Auto-format
./mach fmt

# Included in test-tidy
./mach test-tidy  # Runs rustfmt, ruff, taplo
```

### Naming Conventions

- **Functions**: `snake_case` (`fetch_url`, `process_request`)
- **Types**: `PascalCase` (`WebViewId`, `EmbedderControl`)
- **Constants**: `SCREAMING_SNAKE_CASE` (`MAX_RETRIES`, `DEFAULT_TIMEOUT`)
- **Modules**: `snake_case` (`mod event_loop`, `mod webdriver_server`)

### Code Structure Example

```rust
// ✅ Good - descriptive names, proper error handling
pub async fn fetch_user_by_id(id: String) -> Result<User, FetchError> {
    if id.is_empty() {
        return Err(FetchError::InvalidId);
    }

    let response = api.get(&format!("/users/{}", id)).await?;
    Ok(response)
}

// ❌ Bad - vague names, no error handling
pub async fn get(x: String) -> User {
    api.get(&format!("/users/{}", x)).await.unwrap()
}
```

### Comments Policy

**Code comments should explain WHY, not WHAT**:
- ✅ Good: "We need to retry this request because the service is eventually consistent"
- ❌ Bad: "This function gets the user"

**No comments policy**: Do not add comments to generated code unless explicitly asked.

---

## Dependencies & External Systems

### Major Dependencies (from `Cargo.toml` workspace)

- **SpiderMonkey (`mozjs`)**: JavaScript engine; major build time component
- **WebRender (`webrender`)**: GPU-accelerated rendering engine
- **GStreamer**: Media playback; requires `gstreamer1.0-plugins-base`, `gstreamer1.0-plugins-good`
- **Winit**: Window creation and event handling (headed mode)
- **Surfman**: Surface management for rendering
- **EGL/GL**: Graphics bindings for Linux/Android

### Key Crates

- `base`: Common types and utilities
- `servo`: Main entry point, embedder interface
- `constellation`: Central coordinator, manages threads
- `script`: DOM and JavaScript execution
- `layout`: Parallel layout engine
- `net`: HTTP/HTTPS networking
- `webdriver_server`: WebDriver protocol implementation

### JavaScript Engine

**SpiderMonkey** integration:
- Located in `third_party/mozjs/`
- Exposed to Rust via `mozjs_sys` bindings
- Manages garbage collection, JIT compilation

---

## Common Issues & Troubleshooting

### Build Failures

**Issue**: Missing system dependencies
```bash
# Solution: Run bootstrap to install deps
./mach bootstrap

# Manually install common deps (Debian)
sudo apt install libssl-dev libclang-dev gstreamer1.0-plugins-base gstreamer1.0-plugins-good
```

**Issue**: `mozjs` build errors
- SpiderMonkey is complex; vendored version usually works
- If using system SpiderMonkey, set environment variables

**Issue**: Linker OOM on RPi5
```bash
# Solution: Reduce parallel jobs
./mach build -j 2  # Instead of default -j$(nproc)
```

### Runtime Issues

**Issue**: Panic on startup
```bash
# Enable backtrace
RUST_BACKTRACE=1 ./mach run

# Check logs
RUST_LOG=servo=debug ./mach run

# Common: Headless mode requires software GL backend
LIBGL_ALWAYS_SOFTWARE=1 ./mach run --headless
```

**Issue**: Slow page loads
```bash
# Check network
ping servo.org

# Enable debug logging
RUST_LOG=net=debug ./mach run https://example.com
```

### Test Failures

**Issue**: WPT test timeouts
- Increase timeout: `./mach test-wpt --timeout-multiplier 2.0`
- Run fewer tests in parallel: `./mach test-wpt --processes 2`

**Issue**: Intermittent test failures
- Use `./mach test-wpt --retry-unexpected 1` to retry
- Check thermal: High temps can cause throttling

### Performance Bottlenecks on RPi5

**Issue**: Long compilation times
- Use `--dev` profile for iteration
- Enable ccache if available
- Build on NVMe SSD (already configured)

**Issue**: Slow rendering
- Use software rendering if GPU issues: `./mach run --software`
- Check GPU acceleration: `RUST_LOG=webrender=debug`

---

## Key Commands Reference

### Build & Run
```bash
./mach build              # Build release version
./mach build --dev         # Build dev version
./mach run <URL>           # Run browser
./mach run --headless <URL> # Run headless
./mach clean               # Clean artifacts
```

### Testing
```bash
./mach test-unit           # Run unit tests
./mach test-unit -p <pkg>  # Run specific package tests
./mach test-wpt            # Run WPT tests
./mach test-wpt <path>     # Run specific test
./mach smoketest           # Quick sanity test
./mach test-tidy           # Lint/format check
./mach fmt                 # Auto-format code
```

### Debugging
```bash
RUST_BACKTRACE=1 ./mach run           # Enable backtraces
RUST_LOG=servo=debug ./mach run       # Debug logging
RUST_LOG=net=debug ./mach run         # Network debug
./mach run --devtools 6080 <URL>  # Enable devtools
```

### WebDriver
```bash
./mach run --webdriver=7000 --headless  # Start WebDriver
./mach test-ohos-wpt --test <path>     # OHOS WebDriver tests
```

---

## Boundaries & Guidelines

### ✅ Always Do
- Run `./mach fmt` before committing changes
- Run `./mach test-tidy` to check formatting
- Write unit tests for new features
- Use headless mode for automated testing
- Follow Rust naming conventions (snake_case for functions, PascalCase for types)
- Handle errors with `Result<T, E>` types
- Check thermal during long builds on RPi5
- Use `RUST_BACKTRACE=1` when debugging panics

### ⚠️ Ask First
- Major refactors affecting multiple components
- Changes to `constellation` architecture (central coordinator)
- Experimental features (prefixed with `dom_` or `layout_`)
- Adding new dependencies to workspace
- Changes to SpiderMonkey integration

### 🚫 Never Do
- Commit secrets or API keys
- Modify `target/` or `.venv/` directories
- Remove failing tests without fixing them
- Disable safety checks (`unsafe`) without justification
- Commit unformatted code
- Run full production builds on limited RAM systems (8GB) without `-j` limit

---

## Getting Help

### Use Gemini CLI as Oracle

For doubts, confusion, Root Cause Analysis, or complex problems, consult Gemini:

```bash
# Basic query
gemini -m gemini-3-pro-preview --include-directories=/mnt/nvme/servo "How do I debug layout thread crashes?"

# RCA assistance
gemini -m gemini-3-pro-preview --include-directories=/mnt/nvme/servo "Help me analyze why this WPT test is failing: tests/wpt/tests/dom/historical.html"

# Architecture questions
gemini -m gemini-3-pro-preview --include-directories=/mnt/nvme/servo "Explain how the Constellation manages ScriptThreads and LayoutThreads"
```

### Documentation

- **Servo Book**: https://book.servo.org
- **Getting Started**: https://book.servo.org/building/building.html
- **Hacking Guide**: https://book.servo.org/hacking/
- **Zulip Community**: https://servo.zulipchat.com/
- **GitHub Issues**: https://github.com/servo/servo/issues

### System Documentation

Local system documentation is at `/home/brb/base/`:
- `hardware.md` - Raspberry Pi 5 specs
- `os.md` - Debian 13 (Trixie) details
- `software.md` - Installed tools (Rust 1.92.0, Node v25.2.1)
- `network.md` - WiFi and DNS configuration
- `storage.md` - SD card and NVMe SSD setup

---

## Quick Reference

### Typical Development Workflow

```bash
# 1. Pull latest changes
git pull

# 2. Build
./mach build

# 3. Run tests
./mach test-unit
./mach test-wpt tests/wpt/tests/dom/

# 4. Test headless
./mach run --headless --output test.png https://servo.org

# 5. Check formatting
./mach fmt

# 6. Commit (if all tests pass)
git add .
git commit -m "feat: add new feature"
```

### Headless Testing Workflow

```bash
# 1. Build Servo
./mach build --release

# 2. Run WPT tests (headless by default)
./mach test-wpt css/css-grid/

# 3. Run specific test with image output
./mach run --headless --output grid-render.png tests/html/test-css-grid.html

# 4. Start WebDriver for automation
./mach run --headless --webdriver=4444

# 5. Use WebDriver client to control
# (connect to http://localhost:4444 with Selenium/other client)
```

### Raspberry Pi 5 Optimization Checklist

- [ ] Use NVMe SSD for build artifacts (`/mnt/nvme/servo`)
- [ ] Limit parallel jobs: `./mach build -j 4`
- [ ] Monitor thermal: `vcgencmd measure_temp`
- [ ] Use `--dev` profile for iteration, `--release` for final builds
- [ ] Enable active cooling during long builds
- [ ] Use `--software` rendering if hardware acceleration fails
- [ ] Set `RUST_BACKTRACE=1` for debugging panics
- [ ] Run WPT tests in headless mode (default)

---

## Summary

This AGENTS.md provides comprehensive guidance for AI agents working with Servo browser engine on Raspberry Pi 5.

**Key Points**:
1. **Headless Mode**: WPT tests run headless by default; use `--headless` flag for manual runs
2. **Testing**: Run `./mach test-tidy`, `./mach test-unit`, `./mach smoketest` before committing
3. **Build**: Use `./mach build` (release), limit `-j 4` on RPi5 to avoid OOM
4. **Architecture**: Parallel model with Constellation, ScriptThread, LayoutThread
5. **Help**: Use Gemini CLI (`gemini -m gemini-3-pro-preview`) for complex questions and RCA
6. **Format**: Always run `./mach fmt` before committing; comments explain WHY not WHAT

For questions beyond this guide, consult:
- **Gemini CLI**: Primary oracle for this codebase
- **Servo Book**: https://book.servo.org
- **GitHub Issues**: https://github.com/servo/servo/issues
- **Zulip**: https://servo.zulipchat.com/