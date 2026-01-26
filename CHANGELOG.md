# Changelog

## [Unreleased - 2026-01-26]

### Added
- Initial dev build and testing on RPi5 (ARM64)
- Screenshot testing workflow with zai-mcp-server integration
- Screenshot directory structure at `/mnt/nvme/Pictures/` organized by category

### Tested
- **Build Process**
  - Dev mode build completed successfully with `--dev --jobs 4`
  - Build time: ~3 minutes (after restart, cached artifacts)
  - Binary size: 1.9GB (debug, unoptimized)
  - Platform: aarch64-unknown-linux-gnu (RPi5)
  - Rust version: 1.91.0
  - Build artifacts: 7.4GB in `/mnt/nvme/servo/target/debug/`

- **Screenshot Testing (Headless Mode)**
  - Local HTML test pages:
    - `tests/html/hello.html` - Basic layout test
    - `tests/html/demo.html` - CSS features and styling
  - Popular websites:
    - `https://servo.org` - Official project website
    - `https://example.com` - Placeholder domain
  - WPT test pages:
    - `tests/wpt/tests/css/css-box/test_box_display.html` - CSS box model test

- **Rendering Quality Assessment** (via zai-mcp-server)
  - **hello.html**: Excellent rendering quality. Text is clear, legible, and properly positioned. No visual artifacts or rendering issues.
  - **demo.html**: Excellent rendering quality. Layout is correct, CSS properties rendered accurately, text is sharp. No visual artifacts.
  - **servo.org**: Clean and professional rendering. Layout is modern, navigation is clear, content is highly readable. No obvious rendering issues.
  - **example.com**: Successful rendering. Text, link, and content are properly rendered and highly readable. Matches expected placeholder page.
  - **WPT CSS box test**: Connection error (requires proper WPT server/host setup)

### System Configuration
- **Hardware**: Raspberry Pi 5
  - CPU: ARM Cortex-A76 (quad-core), 1.5 GHz base, up to 2.4 GHz turbo
  - RAM: 8 GB LPDDR4-3200 with 2 GB zram swap
  - Storage: 256 GB NVMe SSD (mounted at `/mnt/nvme/servo`)
  - Temperature during build: 60-67°C (safe, throttles at ~80°C)

- **Software**:
  - OS: Debian 13 (Trixie), aarch64
  - Rust: 1.91.0
  - Python: 3.13.5
  - Mach build system: Python-based orchestrator

### Build Details
- **Profile**: Development (--dev)
- **Parallel Jobs**: 4 (to avoid OOM on 8GB RAM)
- **Total Build Time**: ~30-90 minutes estimated (actual ~3 min after restart due to caching)
- **Binary**: `/mnt/nvme/servo/target/debug/servo` (1.9GB)
- **Compilation Warnings**: Non-critical (surfman lifetime warnings, background_hang_monitor dead_code)

### Known Issues
- **Headless Rendering**: Some tests required `--software` flag and `LIBGL_ALWAYS_SOFTWARE=1` to avoid GPU context issues
- **WPT Tests**: Direct local WPT test pages require proper server/host setup; connection errors occurred without WPT runner
- **Process Management**: Servo processes in headless mode run indefinitely and must be killed explicitly with `kill -9` or `pkill -9`

### Documentation Updates
- Updated `AGENTS.md` with "Screenshot Testing & zai-mcp-server Integration" section
- Added screenshot capture and analysis workflow examples
- Documented test pages and testing tips for headless mode

### Screenshots Captured
All screenshots saved to `/mnt/nvme/Pictures/` organized by category:

```
/mnt/nvme/Pictures/
├── basic-layout/
│   └── hello.png (19KB)
├── css-features/
│   └── demo.png (156KB)
├── popular-websites/
│   ├── servo-org.png (172KB)
│   └── example-com.png (37KB)
└── wpt-tests/
    └── css-box-model.png (26KB)
```

All screenshots are PNG format at 1024x740 resolution.

### Testing Commands Used

```bash
# Build
./mach build --dev --jobs 4

# Run with headless mode and software rendering
./mach run --headless --software --output /path/to/screenshot.png <URL>

# Example: local test
./mach run --headless --software --output /mnt/nvme/Pictures/basic-layout/hello.png tests/html/hello.html

# Example: website
./mach run --headless --software --output /mnt/nvme/Pictures/popular-websites/servo-org.png https://servo.org
```

### Analysis Commands Used

```bash
# Analyze rendering quality
zai-mcp-server_analyze_image \
  --image_source /mnt/nvme/Pictures/basic-layout/hello.png \
  --prompt "Evaluate rendering quality, layout correctness, and any visual artifacts"

# Extract text from screenshots
zai-mcp-server_extract_text_from_screenshot \
  --image_source /mnt/nvme/Pictures/basic-layout/hello.png \
  --prompt "Extract all visible text"
```

### Performance Observations
- **Build Performance**: Good for ARM64, ~3 minutes for cached rebuild (initial build much longer)
- **Thermal Management**: Stable at 60-67°C during builds and testing
- **Memory Usage**: ~2-3GB during development builds (well within 8GB limit)
- **Rendering Speed**: 30-60 seconds for page load and screenshot capture in headless mode

### Conclusion
Servo browser engine successfully built and tested on Raspberry Pi 5. Rendering quality is excellent for basic HTML, CSS features, and modern websites. The headless mode with software rendering works reliably for screenshot capture. The zai-mcp-server integration provides effective automated analysis of rendering quality and layout correctness.
