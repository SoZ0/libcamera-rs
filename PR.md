# Support for v0.6.0 and parity with libcamera

# V0.6.0 Additions

- Regenerated control/property metadata from libcamera tags up to v0.6.0 (including 0.5.2), adding new core WDR/LensDewarp controls and RPi sync controls. Added versioned_files/0.5.2 and 0.6.0.

- Bumped crate versions: libcamera-sys to 0.6.0 and libcamera to 0.6.0; libcamera now depends on libcamera-sys 0.6.0.

- Updated framebuffer FFI for the v0.6 API change: FrameBuffer::planes() returns a span; C wrapper now returns an owned planes wrapper with destroy helper, and Rust FrameBufferPlanesRef drops it correctly. FrameMetadataStatus already includes Startup.

- Verified workspace builds against libcamera v0.6; examples run successfully when linked against the installed v0.6 libs.

## Colour Space now exposed

- Expose color space negotiation for streams: added a C shim for libcamera::ColorSpace and stream config helpers, then wrapped in Rust (color_space module) with enums and conversions.

- StreamConfigurationRef now has get_color_space/set_color_space, and debug output includes the color space.

- Added string parsing/validation helpers for ColorSpace (to/from string, adjust for PixelFormat) and PixelFormat (from_str/is_valid), plus a `color_space_parse` example.

- CameraConfiguration now exposes sensor configuration readback (including binning/skipping/crop) and PixelFormat helpers gained modifier convenience; FrameBuffer gets an `is_contiguous()` helper. Added `sensor_config_readback` example.

- Generated `formats::*` PixelFormat constants from libcamera/formats.h so callers don’t have to hard-code fourcc/modifier strings.
- Added async-friendly channel helpers for requestCompleted/bufferCompleted subscriptions; BufferCompletedEvent now carries stream/cookie/sequence safely.
- PixelFormatInfo shim now exposes plane/V4L2 metadata plus stride/size helpers; PixelFormatInfo includes v4l2 mappings. Added `pixel_format_roundtrip` and `color_space_adjust` examples.
- PixelFormatInfo no longer hard-depends on libcamera internal headers when building against a system libcamera; it will opportunistically use them when available. Safe API now surfaces stride/plane_size/frame_size and V4L2 codes from the shim.

## Vendor feature flag exposed

- Build script now scans libcamera/control_ids.h and generates vendor_features.rs with LIBCAMERA_HAS_* constants for available vendor controls.

- Added a new vendor_features module in the safe crate to surface those flags, so downstream code can compile-time gate vendor controls.

- Introduced vendor_features module structure (flat + macros), exposing generated LIBCAMERA_HAS_* consts and a vendor_has! macro for cleaner use-sites.

## Fences

- Added a fence C shim (libcamera_fence_*) and plumbed libcamera_request_add_buffer_with_fence to take a libcamera_fence_t; framebuffer release now returns a fence handle so the fd isn’t closed out from under callers.

- Rust gains a Fence wrapper; Request::add_buffer_with_fence takes Option<Fence>, and AsFrameBuffer::release_fence returns a Fence with helpers to dup the fd when needed.

## FrameBuffer import & cookies

- Added a C shim and Rust `OwnedFrameBuffer` to construct framebuffers from user-provided DMABUF planes (fd, offset, length) with optional cookie, enabling zero-copy import without the allocator.

- AsFrameBuffer now exposes cookie getter/setter for tagging buffers regardless of origin.

- New `owned_framebuffer` example shows creating a DMABUF-backed framebuffer and mutating cookies.

## Camera signals

- C shim and Rust bindings now expose `bufferCompleted` and `disconnected` signals: `ActiveCamera::on_buffer_completed` delivers per-buffer events with the matching `Stream`, and `ActiveCamera::on_disconnected` notifies on camera unplug.

- New `buffer_completed` example wires the callbacks and recycles requests.

## Camera lookup helper

- CameraManager now exposes `get(id)` to fetch a camera by ID without iterating.

- Added `get_camera` example demonstrating lookup by ID and reading the Model property.

## Import/export & request buffers

- FrameBufferAllocator buffers now expose cookie get/set; imported `OwnedFrameBuffer` initializes metadata sentinel.

- Request now exposes a buffer map iterator (uses libcamera buffer map) and callbacks no longer depend on pointer hacks; bindings added for find_buffer/has_pending_buffers/to_string. New `request_introspect_capture` demonstrates live capture + introspection.

- Added `import_dmabuf_capture` example that imports a memfd-backed DMABUF, captures one frame, and reads metadata; `request_introspect` example demonstrates buffer map/find_buffer/to_string helpers.

- Added `sensor_config_readback` example and `color_space_parse` example; PixelFormat/ColorSpace helpers extended (modifiers, parsing, adjust).

## Stream/transform/logging parity

- Exposed camera streams accessor via C shim/Rust so active streams can be enumerated after configuration.
- Stream now exposes a `configuration()` accessor so adjusted stream configurations can be inspected after configure().

- Added transform helpers bindings and a custom logging stream hook; logging can target any `std::ostream`.
- Fixed Transform FFI layout/bitops so orientation helpers no longer crash; `orientation_helpers` example runs. Added `logging_stdout` example for stdout targets.

- SensorConfiguration output_size now returns Size to match libcamera; added stream set helpers to C shim.

- Added orientation_from_rotation helper to map degrees to EXIF orientations via libcamera’s logic.

- Transform gains bitmask-style ops/constants (hflip/vflip/transpose and bitwise ops) to mirror libcamera’s Transform arithmetic.

## Control/property docs surfacing

- Updated the generator to emit a description(&self) -> &'static str method on generated ControlId/PropertyId enums, with proper feature gating for vendor controls.

- Regenerated versioned_files so all supported libcamera versions now carry these description methods.

- Enhanced the camera_control_id_info example to print the control descriptions, letting users discover capabilities at runtime.

- ControlList parity: added contains/clear/size/is_empty/merge (with MergePolicy) and accessors to the underlying info map; ControlValue now supports string arrays, closing gaps for multi-string controls. queue_request now returns the Request on error so callers can retry/fix, FrameBufferAllocator tracks per-stream lifetimes to avoid double free, framebuffer_map accepts DMA-BUFs with st_size == 0, and vendor_rpi/vendor_draft features now error out if headers lack vendor controls.
- ControlIdMap iteration/id_map exposure wired through the safe layer; ControlList can now be built from id maps. MemoryMappedFrameBuffer reports mapped lengths and keeps writable mappings; new `control_list_ops`, `mmap_info`, and `multi_stream_clone` examples exercise the APIs.

## Hot plugging

- CameraManager now exposes safe callbacks for camera hotplug: on_camera_added and on_camera_removed wrap libcamera’s signals, manage callback handles, and deliver Camera handles to user closures.

- C FFI extended with connect/disconnect hooks for cameraAdded/cameraRemoved signals.

- CameraManager now offers subscribe_hotplug_events() returning an mpsc::Receiver<HotplugEvent> so you can handle hotplug on your own thread. Callbacks warn not to block the libcamera thread.

## Logging

- Logging helper added: configure_stderr(category, level, color) sets up stderr logging and category level in one call; LoggingTarget now includes a Stream placeholder for completeness.

- C logging enum now includes File and Stream targets to match libcamera.

- Rust LoggingTarget now has File and maps Stream correctly to LIBCAMERA_LOGGING_TARGET_STREAM (no longer hits Syslog).

## Camera Configuration

- Added Camera::generate_first_supported_configuration(&[StreamRole]) -> Option<(CameraConfiguration, StreamRole)> to try roles in order and pick the first that yields a config.

- StreamConfigurationRef stride/frame size setters now carry doc notes that they’re intended post-validate() (advanced use).

- SensorConfiguration now exposes analog_crop, binning, and skipping setters (C shim + Rust) to match libcamera 0.6 fields.

- CameraConfiguration exposes orientation getter/setter backed by libcamera::Orientation, enabling applications to request rotated outputs.

- Added CameraConfiguration::add_configuration() to build configs from scratch, plus to_string() diagnostics on camera/stream configurations (sensor_config_readback example prints both). New CameraConfiguration::add_configuration_like() and C shim libcamera_camera_configuration_add_configuration_from clone an existing StreamConfiguration, so multi-stream configs append real configs instead of empty placeholders.

## Version info

- New version module exports compile-time libcamera Version { major, minor, patch } based on LIBCAMERA_VERSION_* macros for downstream feature gating without parsing strings.

## ControlList & metadata parity

- ControlList now exposes contains/len/clear/merge/info_map/id_map, matching libcamera for working with dynamic/vendor controls; string controls now serialize as byte arrays to avoid truncation. New control_list_ops example exercises these helpers.

- Exposed buffer mapper helpers: `MemoryMappedFrameBuffer` now reports writability and per-fd mapped lengths, plus a new `mmap_info` example.
- PixelFormat gained `to_raw`/`from_raw_parts` round-trip helpers (example: `pixel_format_roundtrip`).
- ColorSpace now offers `with_adjusted_for_format` to clone-and-adjust without mutating (example: `color_space_adjust`).
- Logging helpers now include `configure_stdout` and `set_category_level` without constructing a CameraManager (example: `logging_stdout`).
- Multi-stream config helpers: `CameraConfiguration::add_configurations_like` clones multiple stream configs at once; new `multi_stream_clone` example shows composing multi-stream configs without empty placeholders.

## Examples & capture timeouts

- `jpeg_capture` and `video_capture` now wait up to 5s for request completions to avoid spurious timeouts on slower UVC devices; both examples verified writing outputs to `/tmp/libcamera_rust_capture.jpg` and `/tmp/libcamera_rust_video.mjpeg` respectively.
