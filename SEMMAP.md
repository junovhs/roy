# project -- Semantic Map

**Purpose:** Terminal

## Legend

`[ENTRY]` Application entry point

`[CORE]` Core business logic

`[TYPE]` Data structures and types

`[UTIL]` Utility functions

`[HOTSPOT]` High fan-in file imported by 4+ others - request this file early in any task

`[GLOBAL-UTIL]` High fan-in utility imported from 3+ distinct domains

`[DOMAIN-CONTRACT]` Shared contract imported mostly by one subsystem

`[ROLE:model]` Primary domain model or state-holding data structure.

`[ROLE:controller]` Coordinates commands, events, or request handling.

`[ROLE:rendering]` Produces visual output or drawing behavior.

`[ROLE:view]` Represents a reusable UI view or presentation component.

`[ROLE:dialog]` Implements dialog-oriented interaction flow.

`[ROLE:config]` Defines configuration loading or configuration schema behavior.

`[ROLE:os-integration]` Bridges the application to OS-specific APIs or services.

`[ROLE:utility]` Provides cross-cutting helper logic without owning core flow.

`[ROLE:bootstrap]` Initializes the application or wires subsystem startup.

`[ROLE:build-only]` Supports the build toolchain rather than runtime behavior.

`[COUPLING:pure]` Logic stays within the language/runtime without external surface coupling.

`[COUPLING:mixed]` Blends pure logic with side effects or boundary interactions.

`[COUPLING:ui-coupled]` Depends directly on UI framework, rendering, or windowing APIs.

`[COUPLING:os-coupled]` Depends directly on operating-system services or platform APIs.

`[COUPLING:build-only]` Only relevant during build, generation, or compilation steps.

`[BEHAVIOR:owns-state]` Maintains durable in-memory state for a subsystem.

`[BEHAVIOR:mutates]` Changes application or model state in response to work.

`[BEHAVIOR:renders]` Produces rendered output, drawing commands, or visual layout.

`[BEHAVIOR:dispatches]` Routes commands, events, or control flow to other units.

`[BEHAVIOR:observes]` Listens to callbacks, notifications, or external signals.

`[BEHAVIOR:persists]` Reads from or writes to durable storage.

`[BEHAVIOR:spawns-worker]` Creates background workers, threads, or async jobs.

`[BEHAVIOR:sync-primitives]` Coordinates execution with locks, channels, or wait primitives.

`[SURFACE:filesystem]` Touches filesystem paths, files, or directory traversal.

`[SURFACE:ntfs]` Uses NTFS-specific filesystem semantics or metadata.

`[SURFACE:win32]` Touches Win32 platform APIs or Windows-native handles.

`[SURFACE:shell]` Integrates with shell commands, shell UX, or command launch surfaces.

`[SURFACE:clipboard]` Reads from or writes to the system clipboard.

`[SURFACE:gdi]` Uses GDI drawing primitives or related graphics APIs.

`[SURFACE:control]` Represents or manipulates widget/control surfaces.

`[SURFACE:view]` Represents a view-level presentation surface.

`[SURFACE:dialog]` Represents a dialog/window interaction surface.

`[SURFACE:document]` Represents document-oriented editing or display surfaces.

`[SURFACE:frame]` Represents application frame/window chrome surfaces.

`[BEHAVIOR:async]` Uses async/await patterns for concurrent execution.

`[BEHAVIOR:panics-on-error]` Contains unwrap/expect/panic patterns that abort on failure.

`[BEHAVIOR:logs-and-continues]` Logs errors and continues without propagating or aborting.

`[BEHAVIOR:returns-nil-on-error]` Returns nil/null/None on error instead of propagating.

`[BEHAVIOR:swallows-errors]` Catches errors without re-raising or propagating them.

`[BEHAVIOR:propagates-errors]` Propagates errors to callers via Result, throw, or raise.

`[SURFACE:http-handler]` Implements HTTP request handling or web endpoint logic.

`[SURFACE:database]` Interacts with database services or ORMs.

`[SURFACE:external-api]` Makes outbound calls to external HTTP APIs or services.

`[SURFACE:template]` Uses template engines for rendering output.

`[QUALITY:undocumented]` Has public symbols without documentation.

`[QUALITY:complex-flow]` Contains functions with high cognitive complexity.

`[QUALITY:error-boundary]` Concentrated error handling — many panic, swallow, or propagation sites.

`[QUALITY:concurrency-heavy]` Uses multiple concurrency primitives (async, locks, spawn).

`[QUALITY:syntax-degraded]` Parse errors detected — semantic analysis may be incomplete.

## Layer 0 -- Config

`root/` (8 files: 4 .md, 4 .toml)
Representative: AGENTS.md, ARCHITECTURE.md

## Subprojects

### alacritty

`alacritty/CHANGELOG.md`
Release history and notable changes.

`alacritty/Cargo.toml`
Workspace configuration.

`alacritty/README.md`
Project overview and usage guide.

`alacritty/src/config/bell.rs`
Visual bell animation function. [HOTSPOT] [GLOBAL-UTIL] [COUPLING:pure] [QUALITY:undocumented]
Exports: BellAnimation, BellConfig, BellConfig.default, BellConfig.duration
Semantic: pure computation

`alacritty/src/config/bindings.rs`
Describes a state and action to take in that state. [HOTSPOT] [DOMAIN-CONTRACT] [COUPLING:pure] [BEHAVIOR:propagates-errors] [QUALITY:undocumented,complex-flow,error-boundary,syntax-degraded]
Exports: Binding<T>.is_triggered_by, SerdeViMotion, SerdeViMotion.deserialize, default_mouse_bindings
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation that propagates errors

`alacritty/src/config/color.rs`
Implements hint start colors.default. [COUPLING:pure] [BEHAVIOR:propagates-errors] [QUALITY:undocumented]
Exports: Colors.footer_bar_background, Colors.footer_bar_foreground, InvertedCellColors.default, FocusedMatchColors.default
Semantic: pure computation that propagates errors

`alacritty/src/config/cursor.rs`
The minimum blink interval value in milliseconds. [HOTSPOT] [GLOBAL-UTIL] [COUPLING:mixed] [BEHAVIOR:owns-state] [QUALITY:undocumented]
Exports: Cursor.vi_mode_style, ConfigCursorStyle.default, VteCursorShape.from, VteCursorStyle.from
Semantic: side-effecting stateful module

`alacritty/src/config/debug.rs`
Implements renderer preference. [COUPLING:pure]
Exports: RendererPreference, Debug, Debug.default
Semantic: pure computation

`alacritty/src/config/font.rs`
Defines shared font for the config subsystem. [HOTSPOT] [DOMAIN-CONTRACT] [COUPLING:pure] [QUALITY:undocumented]
Exports: SecondaryFontDescription.desc, SecondaryFontDescription, Font.with_size, NumVisitor.expecting
Semantic: pure computation

`alacritty/src/config/general.rs`
Miscellaneous configuration options. [COUPLING:pure]
Exports: General, General.default
Semantic: pure computation

`alacritty/src/config/monitor.rs`
The fallback for `RecommendedWatcher` polling. [HOTSPOT] [GLOBAL-UTIL] [COUPLING:mixed] [BEHAVIOR:owns-state,persists,sync-primitives] [QUALITY:complex-flow]
Exports: ConfigMonitor.needs_restart, ConfigMonitor, ConfigMonitor.new, ConfigMonitor.shutdown
Semantic: synchronized side-effecting stateful adapter

`alacritty/src/config/mouse.rs`
Implements mouse bindings.deserialize. [COUPLING:pure] [BEHAVIOR:propagates-errors] [QUALITY:undocumented]
Exports: MouseBindings, MouseBindings.default, MouseBindings.deserialize, Mouse
Semantic: pure computation that propagates errors

`alacritty/src/config/scrolling.rs`
Implements scrolling history.deserialize. [COUPLING:mixed] [BEHAVIOR:owns-state,propagates-errors] [QUALITY:undocumented]
Exports: MAX_SCROLLBACK_LINES, ScrollingHistory.default, ScrollingHistory.deserialize, Scrolling.default
Semantic: side-effecting stateful module that propagates errors

`alacritty/src/config/selection.rs`
Implements Selection functionality. [COUPLING:pure]
Exports: Selection, Selection.default
Semantic: pure computation

`alacritty/src/config/serde_utils.rs`
Serde helpers. [COUPLING:pure]
Exports: merge
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`alacritty/src/config/terminal.rs`
OSC52 support mode. [COUPLING:pure] [BEHAVIOR:propagates-errors] [QUALITY:undocumented]
Exports: SerdeOsc52, SerdeOsc52.deserialize, Terminal
Semantic: pure computation that propagates errors

`alacritty/src/config/ui_config.rs`
Regex used for the default URL hint. [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:owns-state,panics-on-error,propagates-errors] [QUALITY:undocumented,error-boundary]
Exports: UiConfig.live_config_reload, LazyRegexVariant, LazyRegexVariant.eq, HintInternalAction
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting stateful module that panics on error

`alacritty/src/config/window.rs`
Defines shared window for the config subsystem. [HOTSPOT] [DOMAIN-CONTRACT] [COUPLING:mixed] [BEHAVIOR:owns-state,propagates-errors] [QUALITY:undocumented]
Exports: OptionAsAlt, WindowConfig.option_as_alt, WinitWindowLevel.from, DEFAULT_NAME
Semantic: side-effecting stateful module that propagates errors

`alacritty/build.rs`
utility for build via file I/O. [COUPLING:mixed] [BEHAVIOR:panics-on-error]
Semantic: side-effecting that panics on error

`alacritty/src/clipboard.rs`
Provides shared clipboard used across multiple domains. [HOTSPOT] [GLOBAL-UTIL] [COUPLING:pure] [QUALITY:undocumented]
Exports: Clipboard.new_nop, Clipboard, Clipboard.default, Clipboard.load
Semantic: pure computation

`alacritty/src/daemon.rs`
utility for daemon via file I/O. [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:persists,propagates-errors] [QUALITY:error-boundary]
Exports: foreground_process_path, spawn_daemon
Semantic: side-effecting adapter that propagates errors

`alacritty/src/display/bell.rs`
Visual bell animation. [HOTSPOT] [COUPLING:pure] [QUALITY:undocumented]
Exports: VisualBell.intensity_at_instant, VisualBell.update_config, VisualBell, VisualBell.completed
Semantic: pure computation

`alacritty/src/display/color.rs`
Factor for automatic computation of dim colors. [COUPLING:mixed] [BEHAVIOR:owns-state,propagates-errors] [QUALITY:undocumented,complex-flow]
Exports: List.fill_gray_ramp, CellRgbVisitor.expecting, DIM_FACTOR, Rgb.as_tuple
Semantic: side-effecting stateful module that propagates errors

`alacritty/src/display/content.rs`
Implements renderable content<'a>.cursor. [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:owns-state,propagates-errors] [QUALITY:undocumented,complex-flow]
Exports: MIN_CURSOR_CONTRAST, RenderableCellExtra, HintMatches<'_>.deref, RenderableContent<'_>.next
Semantic: side-effecting stateful module that propagates errors

`alacritty/src/display/cursor.rs`
Convert a cursor into an iterator of rects. [HOTSPOT] [COUPLING:pure] [BEHAVIOR:propagates-errors] [QUALITY:undocumented]
Exports: RenderableCursor.rects, IntoRects, CursorRects.from, CursorRects.next
Semantic: pure computation that propagates errors

`alacritty/src/display/damage.rs`
State of the damage tracking for the [`Display`]. [HOTSPOT] [COUPLING:pure] [BEHAVIOR:propagates-errors] [QUALITY:undocumented]
Exports: damage_y_to_viewport_y, viewport_y_to_damage_y, FrameDamage.mark_fully_damaged, DamageTracker.damage_vi_cursor
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation that propagates errors

`alacritty/src/display/hint.rs`
Implements hint match.hyperlink. [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:owns-state,panics-on-error,propagates-errors] [QUALITY:undocumented,complex-flow,error-boundary]
Exports: visible_unique_hyperlinks_iter, visible_regex_match_iter, MAX_SEARCH_LINES, HintMatch.should_highlight
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting stateful module that panics on error

`alacritty/src/display/meter.rs`
Rendering time meter. [COUPLING:mixed] [BEHAVIOR:owns-state]
Exports: Sampler<'_>.drop, Meter, Meter.average, Meter.sampler
Semantic: side-effecting stateful module

`alacritty/src/display/window.rs`
Window icon for `_NET_WM_ICON` property. [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:owns-state,panics-on-error,propagates-errors] [QUALITY:undocumented,syntax-degraded]
Exports: Window.select_tab_at_index, Window.set_option_as_alt, Window.pre_present_notify, Window.update_ime_position
Semantic: side-effecting stateful module that panics on error

`alacritty/src/input/keyboard.rs`
Implements sequence modifiers.from. [COUPLING:pure] [BEHAVIOR:propagates-errors] [QUALITY:undocumented,complex-flow]
Exports: SequenceModifiers.encode_esc_sequence, Processor<T, A>.key_input, SequenceBase, SequenceBuilder
Semantic: pure computation that propagates errors

`alacritty/src/logging.rs`
Logging for Alacritty. [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:owns-state,persists,sync-primitives,propagates-errors] [QUALITY:undocumented]
Exports: OnDemandLogFile.flush, OnDemandLogFile.write, LOG_TARGET_IPC_CONFIG, LOG_TARGET_WINIT
Semantic: synchronized side-effecting stateful adapter that propagates errors

`alacritty/src/macos/locale.rs`
Sets the locale environment. [COUPLING:mixed] [BEHAVIOR:owns-state]
Exports: set_locale_environment
Semantic: side-effecting stateful module

`alacritty/src/macos/proc.rs`
Error during working directory retrieval. [TYPE] [COUPLING:mixed] [BEHAVIOR:owns-state,propagates-errors] [QUALITY:undocumented,syntax-degraded]
Exports: vnode_info_path, vinfo_stat, PROC_PIDVNODEPATHINFO, proc_vnodepathinfo
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting stateful module that propagates errors

`alacritty/src/message_bar.rs`
Message for display in the MessageBuffer. [HOTSPOT] [GLOBAL-UTIL] [COUPLING:mixed] [BEHAVIOR:owns-state,panics-on-error] [QUALITY:complex-flow]
Exports: CLOSE_BUTTON_TEXT, Message.set_target, MessageBuffer.is_empty, MessageBuffer.is_queued
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting stateful module that panics on error

`alacritty/src/migrate/yaml.rs`
Migration of legacy YAML files to TOML. [COUPLING:pure] [BEHAVIOR:propagates-errors] [QUALITY:error-boundary]
Exports: migrate
Semantic: pure computation that propagates errors

`alacritty/src/polling/ipc.rs`
Alacritty socket IPC. [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:owns-state,persists,propagates-errors] [QUALITY:error-boundary]
Exports: IpcListener, IpcListener.new, IpcListener.process_message, send_message
Semantic: side-effecting stateful adapter that propagates errors

`alacritty/src/polling/signal.rs`
Unix signal listener. [COUPLING:pure] [BEHAVIOR:propagates-errors] [QUALITY:error-boundary]
Exports: SignalListener, SignalListener.new, SignalListener.process_signal
Semantic: pure computation that propagates errors

`alacritty/src/scheduler.rs`
Scheduler for emitting events at a specific time in the future. [COUPLING:pure] [BEHAVIOR:propagates-errors]
Exports: Scheduler.unschedule_window, TimerId, TimerId.new, Scheduler
Semantic: pure computation that propagates errors

`alacritty/src/string.rs`
The action performed by [`StrShortener`]. [COUPLING:pure] [BEHAVIOR:propagates-errors]
Exports: ShortenDirection, StrShortener<'_>.next, TextAction, StrShortener
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation that propagates errors

`alacritty/src/event.rs`
Process window events. [CORE] [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:owns-state,persists,propagates-errors] [QUALITY:undocumented,complex-flow,error-boundary]
Exports: ActionContext<'a, N, T>.on_terminal_input_start, ActionContext<'a, N, T>.selection_is_empty, ActionContext<'a, N, T>.write_to_pty, Processor.about_to_wait
Semantic: side-effecting stateful adapter that propagates errors

`alacritty/src/panic.rs`
Implements attach handler. [CORE] [COUPLING:pure]
Exports: attach_handler
Semantic: pure computation

`alacritty/src/renderer/platform.rs`
The graphics platform that is used by the renderer. [UTIL] [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:persists,propagates-errors] [QUALITY:undocumented]
Exports: pick_gl_config, create_gl_context, create_gl_display, create_gl_surface
Semantic: side-effecting adapter that propagates errors

`alacritty/src/renderer/rects.rs`
Formats lines.update for output. [UTIL] [COUPLING:mixed] [BEHAVIOR:owns-state,propagates-errors] [QUALITY:undocumented,error-boundary]
Exports: RectShaderProgram, RectShaderProgram.new, RectShaderProgram.update_uniforms, RectKind
Semantic: side-effecting stateful module that propagates errors

`alacritty/src/renderer/shader.rs`
Implements shader program.drop. [UTIL] [HOTSPOT] [COUPLING:pure] [BEHAVIOR:propagates-errors] [QUALITY:undocumented]
Exports: ShaderProgram.get_uniform_location, ShaderError, ShaderError.fmt, ShaderProgram
Semantic: pure computation that propagates errors

`alacritty/src/renderer/text/atlas.rs`
Implements atlas insert error. [UTIL] [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:owns-state,propagates-errors] [QUALITY:undocumented]
Exports: Atlas.room_in_row, AtlasInsertError, Atlas.load_glyph, Atlas.advance_row
Semantic: side-effecting stateful module that propagates errors

`alacritty/src/renderer/text/builtin_font.rs`
Hand-rolled drawing of unicode characters that need to fully cover their character area. [UTIL] [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:owns-state,propagates-errors] [QUALITY:complex-flow]
Exports: builtin_glyph, Pixel.add, Pixel.div
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting stateful module that propagates errors

`alacritty/src/renderer/text/gles2.rs`
Implements renderapi<' >.render batch. [UTIL] [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:owns-state,propagates-errors] [QUALITY:undocumented]
Exports: RenderApi<'_>.load_glyph, TextShaderProgram, TextShaderProgram.id, TextShaderProgram.new
Semantic: side-effecting stateful module that propagates errors

`alacritty/src/renderer/text/glsl3.rs`
Implements renderapi<' >.render batch. [UTIL] [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:owns-state,propagates-errors] [QUALITY:undocumented,error-boundary]
Exports: RenderApi<'_>.load_glyph, RenderApi<'_>.render_batch, TEXT_SHADER_F, TextShaderProgram
Semantic: side-effecting stateful module that propagates errors

`alacritty/src/renderer/text/glyph_cache.rs`
LoadGlyph` allows for copying a rasterized glyph into graphics memory. [UTIL] [COUPLING:pure] [BEHAVIOR:propagates-errors] [QUALITY:undocumented,error-boundary]
Exports: GlyphCache.update_font_size, GlyphCache.load_common_glyphs, GlyphCache.reset_glyph_cache, GlyphCache.font_metrics
Semantic: pure computation that propagates errors

`alacritty/src/window_context.rs`
Terminal window context. [CORE] [COUPLING:mixed] [BEHAVIOR:persists,sync-primitives,propagates-errors] [SURFACE:template] [QUALITY:error-boundary]
Exports: WindowContext.write_ref_test_results, WindowContext.add_window_config, WindowContext.reset_window_config, WindowContext.handle_event
Semantic: synchronized side-effecting adapter with template surface that propagates errors

`alacritty/src/cli.rs`
CLI options for the main Alacritty executable. [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:persists,panics-on-error,propagates-errors] [QUALITY:undocumented,error-boundary]
Exports: IpcGetConfig, ParsedOptions.override_config_rc, TerminalOptions.override_pty_config, WindowIdentity.override_identity_config
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting adapter that panics on error

`alacritty/src/config/mod.rs`
Defines shared mod for the config subsystem. [ENTRY] [HOTSPOT] [DOMAIN-CONTRACT] [COUPLING:mixed] [BEHAVIOR:owns-state,persists,panics-on-error,propagates-errors] [QUALITY:undocumented,error-boundary]
Exports: IMPORT_RECURSION_LIMIT, serde_utils, normalize_import, deserialize_config
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting stateful adapter that panics on error

`alacritty/src/display/mod.rs`
The display subsystem including window management, font rasterization, and GPU drawing. [ENTRY] [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:owns-state,panics-on-error,propagates-errors] [QUALITY:undocumented,complex-flow,error-boundary]
Exports: Display.update_highlighted_hints, Display.make_not_current, Display.process_renderer_update, DisplayUpdate.set_cursor_dirty
Semantic: side-effecting stateful module that panics on error

`alacritty/src/input/mod.rs`
Handle input from winit. [ENTRY] [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:owns-state] [QUALITY:undocumented,complex-flow]
Exports: on_terminal_input_start, ActionContext<'_, T>.selection_is_empty, ActionContext<'_, T>.inline_search_state, Processor<T, A>.reset_mouse_cursor
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting stateful module

`alacritty/src/macos/mod.rs`
Implements disable autofill. [ENTRY] [COUPLING:pure]
Exports: disable_autofill, locale, proc
Semantic: pure computation

`alacritty/src/main.rs`
Alacritty - The GPU Enhanced Terminal. [ENTRY] [COUPLING:mixed] [BEHAVIOR:persists,propagates-errors] [QUALITY:error-boundary]
Exports: TemporaryFiles.drop
Semantic: side-effecting adapter that propagates errors

`alacritty/src/migrate/mod.rs`
Configuration file migration. [ENTRY] [COUPLING:mixed] [BEHAVIOR:persists,panics-on-error,propagates-errors] [QUALITY:complex-flow,error-boundary]
Exports: migrate
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting adapter that panics on error

`alacritty/src/polling/mod.rs`
Unix I/O event polling. [ENTRY] [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:owns-state,propagates-errors] [QUALITY:error-boundary]
Exports: IoListenerHandle, IoListener.drop, IoListener.spawn, IoListener
Semantic: side-effecting stateful module that propagates errors

`alacritty/src/renderer/mod.rs`
Implements renderer.draw string. [UTIL] [COUPLING:mixed] [BEHAVIOR:owns-state,propagates-errors] [QUALITY:undocumented,error-boundary]
Exports: GL_FUNS_LOADED, Renderer.was_context_reset, Renderer.set_viewport, Renderer.with_loader
Semantic: side-effecting stateful module that propagates errors

`alacritty/src/renderer/text/mod.rs`
Rendering passes, for both GLES2 and GLSL3 renderer. [UTIL] [COUPLING:pure] [QUALITY:undocumented]
Exports: add_render_item, LoaderApi<'_>.load_glyph, TextRenderBatch, TextRenderApi
Semantic: pure computation

### alacritty_config

`alacritty_config/Cargo.toml`
Workspace configuration.

`alacritty_config/src/lib.rs`
Provides shared lib used across multiple domains. [ENTRY] [HOTSPOT] [GLOBAL-UTIL] [COUPLING:mixed] [BEHAVIOR:panics-on-error,propagates-errors] [QUALITY:undocumented,error-boundary]
Exports: HashMap<String, T>.replace, SerdeReplace, Option<T>.replace, Vec<T>.replace
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting that panics on error

### alacritty_config_derive

`alacritty_config_derive/Cargo.toml`
Workspace configuration.

`alacritty_config_derive/src/config_deserialize/de_enum.rs`
Implements derive deserialize. [COUPLING:pure]
Exports: derive_deserialize
Semantic: pure computation

`alacritty_config_derive/src/config_deserialize/de_struct.rs`
Use this crate's name as log target. [COUPLING:mixed] [BEHAVIOR:owns-state,panics-on-error] [QUALITY:complex-flow]
Exports: derive_deserialize
Semantic: side-effecting stateful module that panics on error

`alacritty_config_derive/src/serde_replace.rs`
Error if the derive was used on an unsupported type. [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:owns-state,panics-on-error,propagates-errors] [QUALITY:undocumented]
Exports: derive_direct, derive_recursive, derive
Semantic: side-effecting stateful module that panics on error

`alacritty_config_derive/src/config_deserialize/mod.rs`
Error if the derive was used on an unsupported type. [ENTRY] [COUPLING:mixed] [BEHAVIOR:owns-state]
Exports: derive
Semantic: side-effecting stateful module

`alacritty_config_derive/src/lib.rs`
Error message when attempting to flatten multiple fields. [ENTRY] [COUPLING:mixed] [BEHAVIOR:owns-state,propagates-errors] [QUALITY:undocumented]
Exports: derive_config_deserialize, derive_serde_replace, Attr.parse
Semantic: side-effecting stateful module that propagates errors

`alacritty_config_derive/tests/config.rs`
Logger storing all messages for later validation. [COUPLING:mixed]
Exports: TestEnum.default, Logger.enabled, Logger.flush, Logger.log
Semantic: side-effecting

### alacritty_terminal

`alacritty_terminal/CHANGELOG.md`
Release history and notable changes.

`alacritty_terminal/Cargo.toml`
Workspace configuration.

`alacritty_terminal/src/event.rs`
Implements event listener. [HOTSPOT] [COUPLING:pure] [QUALITY:undocumented]
Exports: OnResize, WindowSize, VoidListener, send_event
Semantic: pure computation

`alacritty_terminal/src/event_loop.rs`
The main event loop which performs I/O on the pseudoterminal. [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:owns-state,persists,sync-primitives,panics-on-error,propagates-errors] [QUALITY:undocumented,complex-flow]
Exports: EventLoopSendError, EventLoopSendError.fmt, EventLoopSendError.source, Notifier.on_resize
Semantic: synchronized side-effecting stateful adapter that panics on error

`alacritty_terminal/src/grid/resize.rs`
Grid resize and reflow. [HOTSPOT] [COUPLING:pure] [QUALITY:complex-flow]
Exports: Grid<T>.resize
Semantic: pure computation

`alacritty_terminal/src/grid/row.rs`
Defines the Row type which makes up lines in the grid. [HOTSPOT] [GLOBAL-UTIL] [COUPLING:pure] [QUALITY:undocumented]
Exports: Row<T>.front_split_off, Row<T>.from_vec, Row<T>.is_clear, &'a Row<T>.into_iter
Semantic: pure computation

`alacritty_terminal/src/grid/storage.rs`
Implements storage<t>.shrink visible lines. [COUPLING:mixed] [BEHAVIOR:owns-state] [QUALITY:undocumented]
Exports: Storage<T>.grow_visible_lines, Storage<T>.shrink_visible_lines, Storage<T>.replace_inner, Storage<T>.take_all
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting stateful module

`alacritty_terminal/src/index.rs`
Line and Column newtypes for strongly typed tty/grid/terminal APIs. [HOTSPOT] [COUPLING:pure] [QUALITY:undocumented]
Exports: Line.grid_clamp, Line.partial_cmp, Point.grid_clamp, Point<L, C>.partial_cmp
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`alacritty_terminal/src/selection.rs`
State management for a selection in the grid. [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:panics-on-error] [QUALITY:undocumented,complex-flow,error-boundary]
Exports: Selection.include_all, Selection.is_empty, SelectionRange.contains_cell, Selection.intersects_range
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting that panics on error

`alacritty_terminal/src/sync.rs`
Synchronization types. [COUPLING:mixed] [BEHAVIOR:sync-primitives]
Exports: FairMutex<T>.try_lock_unfair, FairMutex<T>.lock_unfair, FairMutex, FairMutex<T>.lease
Semantic: synchronized side-effecting

`alacritty_terminal/src/term/cell.rs`
Counter for hyperlinks without explicit ID. [COUPLING:mixed] [BEHAVIOR:owns-state,propagates-errors] [QUALITY:undocumented]
Exports: Cell.set_underline_color, Cell.clear_wide, Cell.is_empty, Cell.push_zerowidth
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting stateful module that propagates errors

`alacritty_terminal/src/term/color.rs`
Implements colors.index mut. [COUPLING:mixed] [BEHAVIOR:owns-state] [QUALITY:undocumented]
Exports: Colors.index_mut, COUNT, Colors, Colors.default
Semantic: side-effecting stateful module

`alacritty_terminal/src/thread.rs`
Like `thread::spawn`, but with a `name` argument. [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:panics-on-error]
Exports: spawn_named
Semantic: side-effecting that panics on error

`alacritty_terminal/src/tty/unix.rs`
TTY related functionality. [COUPLING:mixed] [BEHAVIOR:owns-state,persists,sync-primitives,propagates-errors] [QUALITY:undocumented,error-boundary]
Exports: Pty.next_child_event, Pty.on_resize, from_fd, ToWinsize
Touch: Contains inline Rust tests alongside runtime code.
Semantic: synchronized side-effecting stateful adapter that propagates errors

`alacritty_terminal/src/tty/windows/blocking.rs`
Code for running a reader/writer on another thread while driving it through `polling`. [COUPLING:mixed] [BEHAVIOR:sync-primitives,panics-on-error] [QUALITY:undocumented,error-boundary]
Exports: ThreadWaker.wake_by_ref, Registration.wake_by_ref, ThreadWaker.wake, UnblockedReader<R>.try_read
Semantic: synchronized side-effecting that panics on error

`alacritty_terminal/src/tty/windows/child.rs`
WinAPI callback to run when child process exits. [COUPLING:mixed] [BEHAVIOR:sync-primitives,panics-on-error] [QUALITY:undocumented]
Exports: event_is_emitted_when_child_exits, ChildExitWatcher.event_rx, ChildExitWatcher.raw_handle, ChildExitWatcher
Touch: Contains inline Rust tests alongside runtime code.
Semantic: synchronized side-effecting that panics on error

`alacritty_terminal/src/vi_mode.rs`
Possible vi mode motion movements. [HOTSPOT] [COUPLING:pure] [QUALITY:complex-flow]
Exports: ViModeCursor, ViModeCursor.motion, ViModeCursor.new, ViModeCursor.scroll
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`alacritty_terminal/src/term/search.rs`
Implements regex iter<'a, t>.new. [UTIL] [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:owns-state,panics-on-error,propagates-errors] [QUALITY:complex-flow,error-boundary]
Exports: Term<T>.inline_search_left, Term<T>.inline_search_right, Term<T>.line_search_left, Term<T>.line_search_right
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting stateful module that panics on error

`alacritty_terminal/src/tty/windows/conpty.rs`
Implements COORD.from functionality. [CORE] [COUPLING:mixed] [BEHAVIOR:owns-state,propagates-errors] [QUALITY:undocumented,error-boundary]
Exports: Conpty.on_resize, COORD.from, Conpty, Conpty.drop
Semantic: side-effecting stateful module that propagates errors

`alacritty_terminal/src/grid/mod.rs`
A specialized 2D grid implementation optimized for use in a terminal. [ENTRY] [HOTSPOT] [GLOBAL-UTIL] [COUPLING:pure] [QUALITY:undocumented,complex-flow]
Exports: Grid<T>.initialize_all, Grid<T>.iter_from, GridIterator<'_, T>.prev, GridIterator<'a, T>.size_hint
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`alacritty_terminal/src/lib.rs`
Alacritty - The GPU Enhanced Terminal. [ENTRY] [HOTSPOT]
Exports: vi_mode, event_loop, index, selection

`alacritty_terminal/src/term/mod.rs`
Exports the `Term` type which is a high-level API for the Grid. [CORE] [COUPLING:mixed] [BEHAVIOR:owns-state,panics-on-error,propagates-errors] [QUALITY:undocumented,complex-flow,error-boundary]
Exports: Term<T>.text_area_size_pixels, Term<T>.text_area_size_chars, Term<T>.move_down_and_cr, Term<T>.move_up_and_cr
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting stateful module that panics on error

`alacritty_terminal/src/tty/mod.rs`
TTY related functionality. [ENTRY] [COUPLING:pure]
Exports: EventedReadWrite, ChildEvent, setup_env, EventedPty
Semantic: pure computation

`alacritty_terminal/src/tty/windows/mod.rs`
Implements pty child event token. [CORE] [COUPLING:mixed] [BEHAVIOR:owns-state] [QUALITY:undocumented]
Exports: PTY_READ_WRITE_TOKEN, PTY_CHILD_EVENT_TOKEN, Pty.next_child_event, Pty.child_watcher
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting stateful module

`alacritty_terminal/src/grid/tests.rs`
Tests for the Grid. [HOTSPOT] [COUPLING:pure] [QUALITY:undocumented]
Exports: usize.is_empty, usize.flags_mut, usize.reset, usize.flags
Semantic: pure computation

`alacritty_terminal/tests/ref.rs`
config for ref via file I/O. [COUPLING:mixed]
Exports: Mock.send_event
Semantic: side-effecting

`alacritty_terminal/tests/ref/alt_reset/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/alt_reset/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/alt_reset/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/clear_underline/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/clear_underline/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/clear_underline/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/colored_reset/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/colored_reset/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/colored_reset/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/colored_underline/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/colored_underline/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/colored_underline/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/csi_rep/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/csi_rep/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/csi_rep/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/decaln_reset/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/decaln_reset/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/decaln_reset/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/deccolm_reset/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/deccolm_reset/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/deccolm_reset/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/delete_chars_reset/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/delete_chars_reset/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/delete_chars_reset/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/delete_lines/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/delete_lines/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/delete_lines/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/erase_chars_reset/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/erase_chars_reset/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/erase_chars_reset/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/erase_in_line/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/erase_in_line/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/erase_in_line/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/fish_cc/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/fish_cc/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/fish_cc/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/grid_reset/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/grid_reset/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/grid_reset/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/history/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/history/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/history/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/hyperlinks/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/hyperlinks/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/hyperlinks/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/indexed_256_colors/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/indexed_256_colors/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/indexed_256_colors/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/insert_blank_reset/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/insert_blank_reset/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/insert_blank_reset/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/issue_855/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/issue_855/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/issue_855/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/ll/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/ll/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/ll/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/newline_with_cursor_beyond_scroll_region/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/newline_with_cursor_beyond_scroll_region/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/newline_with_cursor_beyond_scroll_region/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/origin_goto/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/origin_goto/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/origin_goto/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/region_scroll_down/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/region_scroll_down/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/region_scroll_down/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/row_reset/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/row_reset/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/row_reset/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/saved_cursor/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/saved_cursor/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/saved_cursor/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/saved_cursor_alt/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/saved_cursor_alt/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/saved_cursor_alt/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/scroll_in_region_up_preserves_history/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/scroll_in_region_up_preserves_history/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/scroll_in_region_up_preserves_history/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/scroll_up_reset/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/scroll_up_reset/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/scroll_up_reset/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/selective_erasure/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/selective_erasure/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/selective_erasure/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/sgr/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/sgr/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/sgr/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/tab_rendering/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/tab_rendering/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/tab_rendering/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/tmux_git_log/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/tmux_git_log/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/tmux_git_log/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/tmux_htop/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/tmux_htop/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/tmux_htop/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/underline/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/underline/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/underline/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/vim_24bitcolors_bce/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/vim_24bitcolors_bce/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/vim_24bitcolors_bce/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/vim_large_window_scroll/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/vim_large_window_scroll/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/vim_large_window_scroll/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/vim_simple_edit/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/vim_simple_edit/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/vim_simple_edit/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/vttest_cursor_movement_1/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/vttest_cursor_movement_1/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/vttest_cursor_movement_1/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/vttest_insert/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/vttest_insert/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/vttest_insert/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/vttest_origin_mode_1/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/vttest_origin_mode_1/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/vttest_origin_mode_1/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/vttest_origin_mode_2/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/vttest_origin_mode_2/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/vttest_origin_mode_2/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/vttest_scroll/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/vttest_scroll/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/vttest_scroll/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/vttest_tab_clear_set/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/vttest_tab_clear_set/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/vttest_tab_clear_set/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/wrapline_alt_toggle/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/wrapline_alt_toggle/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/wrapline_alt_toggle/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/zerowidth/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/zerowidth/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/zerowidth/size.json`
Implements size functionality. data.

`alacritty_terminal/tests/ref/zsh_tab_completion/config.json`
Implements config functionality. data.

`alacritty_terminal/tests/ref/zsh_tab_completion/grid.json`
Implements grid functionality. data.

`alacritty_terminal/tests/ref/zsh_tab_completion/size.json`
Implements size functionality. data.

### roy

`roy/Cargo.toml`
Workspace configuration.

`roy/src/config.rs`
Top-level roy.toml configuration. [COUPLING:mixed] [BEHAVIOR:owns-state,persists,panics-on-error,propagates-errors] [QUALITY:undocumented]
Exports: RoyConfig.default, RuleConfig.into_rule, load_default, ActionConfig
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting stateful adapter that panics on error

`roy/src/denial.rs`
Structured denial event — printed inline to the terminal, written to session log. [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:panics-on-error] [SURFACE:template] [QUALITY:undocumented]
Exports: DenialResponse.with_rule_id, DenialResponse.with_alternative, DenialResponse, DenialResponse.new
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting with template surface that panics on error

`roy/src/interceptor.rs`
Outcome returned by the interceptor for each PTY write. [HOTSPOT] [GLOBAL-UTIL] [COUPLING:mixed] [BEHAVIOR:panics-on-error] [QUALITY:undocumented]
Exports: strip_bracketed_paste, RoyInterceptor, LineBuffer, LineBuffer.clear
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting that panics on error

`roy/src/policy.rs`
Provides shared policy used across multiple domains. [HOTSPOT] [GLOBAL-UTIL] [COUPLING:mixed] [BEHAVIOR:panics-on-error] [QUALITY:undocumented]
Exports: PolicyEngine, PolicyEngine.empty, PolicyEngine.evaluate, PolicyEngine.new
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting that panics on error

`roy/src/session_log.rs`
Append-only JSON-lines log of denial events for a session. [HOTSPOT] [GLOBAL-UTIL] [COUPLING:mixed] [BEHAVIOR:persists,panics-on-error,propagates-errors] [QUALITY:error-boundary]
Exports: DenialLog, DenialLog.append, DenialLog.open
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting adapter that panics on error

`roy/src/session.rs`
Concrete ROY interceptor bound to a session. [CORE] [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:sync-primitives,panics-on-error] [QUALITY:undocumented]
Exports: RoySession.take_pending_denials, session_from_env, RoySession.intercept, RoySession.new
Touch: Contains inline Rust tests alongside runtime code.
Semantic: synchronized side-effecting that panics on error

`roy/src/lib.rs`
Re-exports the public API surface. [ENTRY]
Exports: PolicyEngine, session_log, DenialResponse, DenialLog


## DependencyGraph

```yaml
DependencyGraph:
  # --- Layer 0 -- Config ---
  AGENTS.md, ARCHITECTURE.md, CLAUDE.md, Cargo.toml, SEMMAP.md, neti.toml, roy.toml, rustfmt.toml:
    Imports: []
    ImportedBy: []
  # --- Subproject -- alacritty ---
  alacritty/CHANGELOG.md, alacritty/Cargo.toml, alacritty/README.md:
    Imports: []
    ImportedBy: []
  alacritty/src/event.rs:
    Imports: [alacritty_config/src/lib.rs, alacritty_terminal/src/event.rs, builtin_font.rs, cli.rs, clipboard.rs, config/bell.rs, content.rs, daemon.rs, damage.rs, display/bell.rs, display/mod.rs, font.rs, gles2.rs, grid/mod.rs, index.rs, input/mod.rs, interceptor.rs, keyboard.rs, message_bar.rs, monitor.rs, scheduler.rs, search.rs, session.rs, session_log.rs, src/selection.rs, terminal.rs, ui_config.rs, vi_mode.rs, window_context.rs]
    ImportedBy: [input/mod.rs, keyboard.rs, main.rs]
  atlas.rs:
    Imports: [interceptor.rs, monitor.rs]
    ImportedBy: [gles2.rs, glsl3.rs, glyph_cache.rs, text/mod.rs]
  bindings.rs:
    Imports: [config/bell.rs, event_loop.rs, message_bar.rs, monitor.rs, mouse.rs, policy.rs]
    ImportedBy: [alacritty_config/src/lib.rs, config/color.rs, config/mod.rs, config/window.rs, display/color.rs, input/mod.rs, keyboard.rs, mouse.rs, scrolling.rs, terminal.rs, ui_config.rs]
  build.rs:
    Imports: [config/cursor.rs, monitor.rs]
    ImportedBy: []
  builtin_font.rs:
    Imports: [config/bell.rs, config/cursor.rs, content.rs, monitor.rs]
    ImportedBy: [alacritty/src/event.rs, glyph_cache.rs, hint.rs, index.rs, input/mod.rs, polling/mod.rs, search.rs, storage.rs, text/mod.rs, vi_mode.rs]
  cli.rs:
    Imports: [alacritty_config/src/lib.rs, alacritty_config_derive/src/lib.rs, config/bell.rs, config/cursor.rs, message_bar.rs, monitor.rs, session_log.rs]
    ImportedBy: [alacritty/src/event.rs, ipc.rs, main.rs, window_context.rs]
  clipboard.rs:
    Imports: [config/bell.rs, monitor.rs]
    ImportedBy: [alacritty/src/event.rs, child.rs, input/mod.rs, main.rs, src/config.rs]
  config/bell.rs:
    Imports: [monitor.rs]
    ImportedBy: [alacritty/src/event.rs, alacritty_config_derive/src/lib.rs, bindings.rs, builtin_font.rs, cell.rs, cli.rs, clipboard.rs, config/color.rs, config/cursor.rs, config/mod.rs, config/selection.rs, config/window.rs, content.rs, damage.rs, de_struct.rs, debug.rs, display/bell.rs, display/color.rs, display/cursor.rs, display/mod.rs, display/window.rs, event_loop.rs, font.rs, general.rs, glyph_cache.rs, grid/mod.rs, hint.rs, input/mod.rs, ipc.rs, message_bar.rs, monitor.rs, mouse.rs, rects.rs, ref.rs, resize.rs, row.rs, scrolling.rs, search.rs, serde_replace.rs, src/config.rs, src/selection.rs, term/mod.rs, tests.rs, tests/config.rs, ui_config.rs, vi_mode.rs, window_context.rs, windows/mod.rs]
  config/color.rs:
    Imports: [bindings.rs, config/bell.rs, monitor.rs]
    ImportedBy: [config/mod.rs, display/mod.rs]
  config/cursor.rs:
    Imports: [config/bell.rs, monitor.rs]
    ImportedBy: [blocking.rs, build.rs, builtin_font.rs, child.rs, cli.rs, config/mod.rs, config/window.rs, content.rs, daemon.rs, de_enum.rs, display/bell.rs, display/mod.rs, display/window.rs, glyph_cache.rs, hint.rs, input/mod.rs, interceptor.rs, ipc.rs, keyboard.rs, main.rs, message_bar.rs, meter.rs, platform.rs, proc.rs, renderer/mod.rs, serde_utils.rs, shader.rs, src/config.rs, term/mod.rs, tests/config.rs, tty/mod.rs, ui_config.rs, unix.rs]
  config/mod.rs:
    Imports: [bindings.rs, config/bell.rs, config/color.rs, config/cursor.rs, config/selection.rs, config/window.rs, debug.rs, event_loop.rs, font.rs, general.rs, message_bar.rs, monitor.rs, mouse.rs, scrolling.rs, serde_utils.rs, terminal.rs, ui_config.rs]
    ImportedBy: [main.rs, migrate/mod.rs, yaml.rs]
  config/selection.rs, debug.rs, general.rs:
    Imports: [config/bell.rs]
    ImportedBy: [config/mod.rs]
  config/window.rs:
    Imports: [bindings.rs, config/bell.rs, config/cursor.rs, monitor.rs]
    ImportedBy: [config/mod.rs, display/window.rs, keyboard.rs, window_context.rs]
  content.rs:
    Imports: [alacritty_config/src/lib.rs, config/bell.rs, config/cursor.rs, hint.rs, message_bar.rs, monitor.rs, src/selection.rs]
    ImportedBy: [alacritty/src/event.rs, builtin_font.rs, damage.rs, display/mod.rs, hint.rs, keyboard.rs, platform.rs, search.rs, string.rs, tests.rs]
  daemon.rs:
    Imports: [config/cursor.rs, monitor.rs, polling/mod.rs]
    ImportedBy: [alacritty/src/event.rs, input/mod.rs, main.rs]
  damage.rs:
    Imports: [config/bell.rs, content.rs, interceptor.rs, monitor.rs, resize.rs, row.rs, tests.rs]
    ImportedBy: [alacritty/src/event.rs, display/mod.rs, term/mod.rs]
  display/bell.rs:
    Imports: [config/bell.rs, config/cursor.rs]
    ImportedBy: [alacritty/src/event.rs, display/mod.rs, window_context.rs]
  display/color.rs:
    Imports: [alacritty_terminal/src/lib.rs, bindings.rs, config/bell.rs, monitor.rs]
    ImportedBy: [display/mod.rs, rects.rs]
  display/cursor.rs:
    Imports: [config/bell.rs, monitor.rs]
    ImportedBy: [display/mod.rs, rects.rs]
  display/mod.rs:
    Imports: [config/bell.rs, config/color.rs, config/cursor.rs, content.rs, damage.rs, display/bell.rs, display/color.rs, display/cursor.rs, display/window.rs, event_loop.rs, font.rs, gles2.rs, glyph_cache.rs, hint.rs, interceptor.rs, message_bar.rs, meter.rs, monitor.rs, platform.rs, renderer/mod.rs, resize.rs, row.rs]
    ImportedBy: [alacritty/src/event.rs, main.rs, window_context.rs]
  display/window.rs:
    Imports: [config/bell.rs, config/cursor.rs, config/window.rs, event_loop.rs, gles2.rs, message_bar.rs, monitor.rs]
    ImportedBy: [display/mod.rs, input/mod.rs, window_context.rs]
  font.rs:
    Imports: [config/bell.rs, monitor.rs]
    ImportedBy: [alacritty/src/event.rs, config/mod.rs, display/mod.rs, window_context.rs]
  gles2.rs:
    Imports: [atlas.rs, glsl3.rs, interceptor.rs, message_bar.rs, monitor.rs, policy.rs, shader.rs]
    ImportedBy: [alacritty/src/event.rs, cell.rs, display/mod.rs, display/window.rs, glsl3.rs, input/mod.rs, keyboard.rs, rects.rs, shader.rs, text/mod.rs, unix.rs, window_context.rs]
  glsl3.rs:
    Imports: [atlas.rs, gles2.rs, interceptor.rs, message_bar.rs, monitor.rs, policy.rs, shader.rs]
    ImportedBy: [gles2.rs, text/mod.rs]
  glyph_cache.rs:
    Imports: [atlas.rs, builtin_font.rs, config/bell.rs, config/cursor.rs, interceptor.rs, monitor.rs]
    ImportedBy: [display/mod.rs, text/mod.rs]
  hint.rs:
    Imports: [builtin_font.rs, config/bell.rs, config/cursor.rs, content.rs, grid/mod.rs, interceptor.rs, message_bar.rs, monitor.rs, resize.rs, search.rs]
    ImportedBy: [content.rs, display/mod.rs, input/mod.rs, ui_config.rs]
  input/mod.rs:
    Imports: [alacritty/src/event.rs, alacritty_terminal/src/lib.rs, bindings.rs, builtin_font.rs, clipboard.rs, config/bell.rs, config/cursor.rs, daemon.rs, display/window.rs, gles2.rs, hint.rs, keyboard.rs, message_bar.rs, monitor.rs, session_log.rs, src/selection.rs, vi_mode.rs]
    ImportedBy: [alacritty/src/event.rs, keyboard.rs, main.rs]
  ipc.rs:
    Imports: [alacritty_terminal/src/event.rs, cli.rs, config/bell.rs, config/cursor.rs, interceptor.rs, logging.rs, monitor.rs]
    ImportedBy: [main.rs, polling/mod.rs]
  keyboard.rs:
    Imports: [alacritty/src/event.rs, bindings.rs, config/cursor.rs, config/window.rs, content.rs, gles2.rs, input/mod.rs, message_bar.rs, monitor.rs, policy.rs]
    ImportedBy: [alacritty/src/event.rs, input/mod.rs]
  locale.rs:
    Imports: [monitor.rs]
    ImportedBy: [macos/mod.rs, main.rs]
  logging.rs:
    Imports: [alacritty_terminal/src/event.rs, message_bar.rs, monitor.rs, session_log.rs, tests/config.rs]
    ImportedBy: [event_loop.rs, ipc.rs, main.rs, migrate/mod.rs, row.rs]
  macos/mod.rs:
    Imports: [locale.rs, proc.rs]
    ImportedBy: [main.rs]
  main.rs:
    Imports: [alacritty/src/event.rs, cli.rs, clipboard.rs, config/cursor.rs, config/mod.rs, daemon.rs, display/mod.rs, input/mod.rs, ipc.rs, locale.rs, logging.rs, macos/mod.rs, message_bar.rs, migrate/mod.rs, monitor.rs, panic.rs, polling/mod.rs, renderer/mod.rs, scheduler.rs, string.rs, tty/mod.rs, window_context.rs]
    ImportedBy: []
  message_bar.rs:
    Imports: [config/bell.rs, config/cursor.rs, monitor.rs]
    ImportedBy: [alacritty/src/event.rs, bindings.rs, blocking.rs, cell.rs, cli.rs, config/mod.rs, conpty.rs, content.rs, de_struct.rs, display/mod.rs, display/window.rs, event_loop.rs, gles2.rs, glsl3.rs, grid/mod.rs, hint.rs, input/mod.rs, keyboard.rs, logging.rs, main.rs, migrate/mod.rs, monitor.rs, rects.rs, renderer/mod.rs, row.rs, scheduler.rs, serde_replace.rs, src/selection.rs, string.rs, term/mod.rs, text/mod.rs, unix.rs, window_context.rs, windows/mod.rs]
  meter.rs:
    Imports: [config/cursor.rs, monitor.rs]
    ImportedBy: [display/mod.rs]
  migrate/mod.rs:
    Imports: [alacritty_config/src/lib.rs, config/mod.rs, logging.rs, message_bar.rs, row.rs, yaml.rs]
    ImportedBy: [main.rs]
  monitor.rs:
    Imports: [alacritty_terminal/src/event.rs, config/bell.rs, message_bar.rs, row.rs, thread.rs]
    ImportedBy: [alacritty/src/event.rs, atlas.rs, bindings.rs, blocking.rs, build.rs, builtin_font.rs, cell.rs, child.rs, cli.rs, clipboard.rs, config/bell.rs, config/color.rs, config/cursor.rs, config/mod.rs, config/window.rs, config_deserialize/mod.rs, conpty.rs, content.rs, daemon.rs, damage.rs, de_enum.rs, de_struct.rs, denial.rs, display/color.rs, display/cursor.rs, display/mod.rs, display/window.rs, event_loop.rs, font.rs, gles2.rs, glsl3.rs, glyph_cache.rs, grid/mod.rs, hint.rs, index.rs, input/mod.rs, interceptor.rs, ipc.rs, keyboard.rs, locale.rs, logging.rs, main.rs, message_bar.rs, meter.rs, panic.rs, platform.rs, policy.rs, polling/mod.rs, rects.rs, ref.rs, renderer/mod.rs, resize.rs, scheduler.rs, search.rs, serde_replace.rs, serde_utils.rs, session.rs, session_log.rs, shader.rs, signal.rs, src/config.rs, src/selection.rs, storage.rs, sync.rs, term/mod.rs, tests.rs, tests/config.rs, thread.rs, ui_config.rs, unix.rs, vi_mode.rs, window_context.rs, windows/mod.rs, yaml.rs]
  mouse.rs:
    Imports: [bindings.rs, config/bell.rs]
    ImportedBy: [bindings.rs, config/mod.rs]
  panic.rs:
    Imports: [monitor.rs]
    ImportedBy: [main.rs]
  platform.rs:
    Imports: [config/cursor.rs, content.rs, monitor.rs]
    ImportedBy: [display/mod.rs, renderer/mod.rs, window_context.rs]
  polling/mod.rs:
    Imports: [builtin_font.rs, interceptor.rs, ipc.rs, monitor.rs, signal.rs, thread.rs]
    ImportedBy: [child.rs, daemon.rs, main.rs, thread.rs, unix.rs, window_context.rs]
  proc.rs:
    Imports: [config/cursor.rs, event_loop.rs]
    ImportedBy: [macos/mod.rs]
  rects.rs:
    Imports: [config/bell.rs, display/color.rs, display/cursor.rs, gles2.rs, interceptor.rs, message_bar.rs, monitor.rs, shader.rs]
    ImportedBy: [renderer/mod.rs]
  renderer/mod.rs:
    Imports: [config/cursor.rs, event_loop.rs, message_bar.rs, monitor.rs, platform.rs, policy.rs, rects.rs, resize.rs, shader.rs, text/mod.rs]
    ImportedBy: [display/mod.rs, main.rs]
  scheduler.rs:
    Imports: [alacritty_terminal/src/event.rs, message_bar.rs, monitor.rs]
    ImportedBy: [alacritty/src/event.rs, main.rs]
  scrolling.rs:
    Imports: [bindings.rs, config/bell.rs]
    ImportedBy: [config/mod.rs]
  serde_utils.rs:
    Imports: [config/cursor.rs, monitor.rs, session_log.rs]
    ImportedBy: [config/mod.rs]
  shader.rs:
    Imports: [config/cursor.rs, gles2.rs, monitor.rs]
    ImportedBy: [gles2.rs, glsl3.rs, rects.rs, renderer/mod.rs]
  signal.rs:
    Imports: [alacritty_terminal/src/event.rs, monitor.rs]
    ImportedBy: [polling/mod.rs]
  string.rs:
    Imports: [content.rs, message_bar.rs]
    ImportedBy: [main.rs]
  terminal.rs:
    Imports: [bindings.rs]
    ImportedBy: [alacritty/src/event.rs, config/mod.rs]
  text/mod.rs:
    Imports: [atlas.rs, builtin_font.rs, gles2.rs, glsl3.rs, glyph_cache.rs, message_bar.rs]
    ImportedBy: [renderer/mod.rs]
  ui_config.rs:
    Imports: [bindings.rs, config/bell.rs, config/cursor.rs, hint.rs, monitor.rs]
    ImportedBy: [alacritty/src/event.rs, config/mod.rs, grid/mod.rs, index.rs]
  window_context.rs:
    Imports: [alacritty_config/src/lib.rs, alacritty_terminal/src/event.rs, cli.rs, config/bell.rs, config/window.rs, display/bell.rs, display/mod.rs, display/window.rs, font.rs, gles2.rs, grid/mod.rs, interceptor.rs, message_bar.rs, monitor.rs, platform.rs, polling/mod.rs, session.rs, session_log.rs]
    ImportedBy: [alacritty/src/event.rs, main.rs]
  yaml.rs:
    Imports: [config/mod.rs, monitor.rs]
    ImportedBy: [migrate/mod.rs]
  # --- Subproject -- alacritty_config ---
  alacritty_config/Cargo.toml:
    Imports: []
    ImportedBy: []
  alacritty_config/src/lib.rs:
    Imports: [bindings.rs]
    ImportedBy: [alacritty/src/event.rs, cli.rs, content.rs, interceptor.rs, migrate/mod.rs, resize.rs, search.rs, term/mod.rs, tests/config.rs, window_context.rs]
  # --- Subproject -- alacritty_config_derive ---
  alacritty_config_derive/Cargo.toml:
    Imports: []
    ImportedBy: []
  alacritty_config_derive/src/lib.rs:
    Imports: [config/bell.rs, config_deserialize/mod.rs, serde_replace.rs]
    ImportedBy: [cli.rs]
  config_deserialize/mod.rs:
    Imports: [de_enum.rs, de_struct.rs, monitor.rs]
    ImportedBy: [alacritty_config_derive/src/lib.rs]
  de_enum.rs:
    Imports: [config/cursor.rs, monitor.rs, serde_replace.rs]
    ImportedBy: [config_deserialize/mod.rs]
  de_struct.rs:
    Imports: [config/bell.rs, message_bar.rs, monitor.rs, serde_replace.rs]
    ImportedBy: [config_deserialize/mod.rs]
  serde_replace.rs:
    Imports: [config/bell.rs, message_bar.rs, monitor.rs]
    ImportedBy: [alacritty_config_derive/src/lib.rs, de_enum.rs, de_struct.rs]
  tests/config.rs:
    Imports: [alacritty_config/src/lib.rs, config/bell.rs, config/cursor.rs, monitor.rs]
    ImportedBy: [logging.rs]
  # --- Subproject -- alacritty_terminal ---
  alacritty_terminal/CHANGELOG.md, alacritty_terminal/Cargo.toml, alt_reset/config.json, alt_reset/grid.json, alt_reset/size.json, clear_underline/config.json, clear_underline/grid.json, clear_underline/size.json, colored_reset/config.json, colored_reset/grid.json, colored_reset/size.json, colored_underline/config.json, colored_underline/grid.json, colored_underline/size.json, csi_rep/config.json, csi_rep/grid.json, csi_rep/size.json, decaln_reset/config.json, decaln_reset/grid.json, decaln_reset/size.json, deccolm_reset/config.json, deccolm_reset/grid.json, deccolm_reset/size.json, delete_chars_reset/config.json, delete_chars_reset/grid.json, delete_chars_reset/size.json, delete_lines/config.json, delete_lines/grid.json, delete_lines/size.json, erase_chars_reset/config.json, erase_chars_reset/grid.json, erase_chars_reset/size.json, erase_in_line/config.json, erase_in_line/grid.json, erase_in_line/size.json, fish_cc/config.json, fish_cc/grid.json, fish_cc/size.json, grid_reset/config.json, grid_reset/grid.json, grid_reset/size.json, history/config.json, history/grid.json, history/size.json, hyperlinks/config.json, hyperlinks/grid.json, hyperlinks/size.json, indexed_256_colors/config.json, indexed_256_colors/grid.json, indexed_256_colors/size.json, insert_blank_reset/config.json, insert_blank_reset/grid.json, insert_blank_reset/size.json, issue_855/config.json, issue_855/grid.json, issue_855/size.json, ll/config.json, ll/grid.json, ll/size.json, newline_with_cursor_beyond_scroll_region/config.json, newline_with_cursor_beyond_scroll_region/grid.json, newline_with_cursor_beyond_scroll_region/size.json, origin_goto/config.json, origin_goto/grid.json, origin_goto/size.json, region_scroll_down/config.json, region_scroll_down/grid.json, region_scroll_down/size.json, row_reset/config.json, row_reset/grid.json, row_reset/size.json, saved_cursor/config.json, saved_cursor/grid.json, saved_cursor/size.json, saved_cursor_alt/config.json, saved_cursor_alt/grid.json, saved_cursor_alt/size.json, scroll_in_region_up_preserves_history/config.json, scroll_in_region_up_preserves_history/grid.json, scroll_in_region_up_preserves_history/size.json, scroll_up_reset/config.json, scroll_up_reset/grid.json, scroll_up_reset/size.json, selective_erasure/config.json, selective_erasure/grid.json, selective_erasure/size.json, sgr/config.json, sgr/grid.json, sgr/size.json, tab_rendering/config.json, tab_rendering/grid.json, tab_rendering/size.json, tmux_git_log/config.json, tmux_git_log/grid.json, tmux_git_log/size.json, tmux_htop/config.json, tmux_htop/grid.json, tmux_htop/size.json, underline/config.json, underline/grid.json, underline/size.json, vim_24bitcolors_bce/config.json, vim_24bitcolors_bce/grid.json, vim_24bitcolors_bce/size.json, vim_large_window_scroll/config.json, vim_large_window_scroll/grid.json, vim_large_window_scroll/size.json, vim_simple_edit/config.json, vim_simple_edit/grid.json, vim_simple_edit/size.json, vttest_cursor_movement_1/config.json, vttest_cursor_movement_1/grid.json, vttest_cursor_movement_1/size.json, vttest_insert/config.json, vttest_insert/grid.json, vttest_insert/size.json, vttest_origin_mode_1/config.json, vttest_origin_mode_1/grid.json, vttest_origin_mode_1/size.json, vttest_origin_mode_2/config.json, vttest_origin_mode_2/grid.json, vttest_origin_mode_2/size.json, vttest_scroll/config.json, vttest_scroll/grid.json, vttest_scroll/size.json, vttest_tab_clear_set/config.json, vttest_tab_clear_set/grid.json, vttest_tab_clear_set/size.json, wrapline_alt_toggle/config.json, wrapline_alt_toggle/grid.json, wrapline_alt_toggle/size.json, zerowidth/config.json, zerowidth/grid.json, zerowidth/size.json, zsh_tab_completion/config.json, zsh_tab_completion/grid.json, zsh_tab_completion/size.json:
    Imports: []
    ImportedBy: []
  alacritty_terminal/src/event.rs:
    Imports: []
    ImportedBy: [alacritty/src/event.rs, alacritty_terminal/src/lib.rs, event_loop.rs, ipc.rs, logging.rs, monitor.rs, scheduler.rs, signal.rs, term/mod.rs, window_context.rs]
  alacritty_terminal/src/lib.rs:
    Imports: [alacritty_terminal/src/event.rs, event_loop.rs, grid/mod.rs, index.rs, src/selection.rs, sync.rs, term/mod.rs, thread.rs, tty/mod.rs, vi_mode.rs]
    ImportedBy: [display/color.rs, input/mod.rs]
  blocking.rs:
    Imports: [config/cursor.rs, message_bar.rs, monitor.rs, thread.rs]
    ImportedBy: [windows/mod.rs]
  cell.rs:
    Imports: [config/bell.rs, gles2.rs, message_bar.rs, monitor.rs, policy.rs]
    ImportedBy: [term/mod.rs]
  child.rs:
    Imports: [clipboard.rs, config/cursor.rs, monitor.rs, polling/mod.rs]
    ImportedBy: [windows/mod.rs]
  conpty.rs:
    Imports: [message_bar.rs, monitor.rs, resize.rs]
    ImportedBy: [windows/mod.rs]
  event_loop.rs:
    Imports: [alacritty_terminal/src/event.rs, config/bell.rs, interceptor.rs, logging.rs, message_bar.rs, monitor.rs, sync.rs, thread.rs, unix.rs]
    ImportedBy: [alacritty_terminal/src/lib.rs, bindings.rs, config/mod.rs, display/mod.rs, display/window.rs, proc.rs, renderer/mod.rs, windows/mod.rs]
  grid/mod.rs:
    Imports: [config/bell.rs, message_bar.rs, monitor.rs, resize.rs, row.rs, storage.rs, tests.rs, ui_config.rs]
    ImportedBy: [alacritty/src/event.rs, alacritty_terminal/src/lib.rs, hint.rs, ref.rs, search.rs, tests.rs, window_context.rs]
  index.rs:
    Imports: [builtin_font.rs, monitor.rs, ui_config.rs]
    ImportedBy: [alacritty/src/event.rs, alacritty_terminal/src/lib.rs, resize.rs, search.rs, src/selection.rs, term/mod.rs, vi_mode.rs]
  ref.rs:
    Imports: [config/bell.rs, grid/mod.rs, monitor.rs, session_log.rs]
    ImportedBy: []
  resize.rs:
    Imports: [alacritty_config/src/lib.rs, config/bell.rs, index.rs, monitor.rs, row.rs, session_log.rs, storage.rs, tests.rs]
    ImportedBy: [conpty.rs, damage.rs, display/mod.rs, grid/mod.rs, hint.rs, renderer/mod.rs, term/mod.rs, tests.rs]
  row.rs:
    Imports: [config/bell.rs, logging.rs, message_bar.rs, session_log.rs, tests.rs]
    ImportedBy: [damage.rs, display/mod.rs, grid/mod.rs, migrate/mod.rs, monitor.rs, resize.rs, session.rs, term/mod.rs, tests.rs, vi_mode.rs]
  search.rs:
    Imports: [alacritty_config/src/lib.rs, builtin_font.rs, config/bell.rs, content.rs, grid/mod.rs, index.rs, monitor.rs]
    ImportedBy: [alacritty/src/event.rs, hint.rs, src/selection.rs, term/mod.rs]
  src/selection.rs:
    Imports: [config/bell.rs, index.rs, message_bar.rs, monitor.rs, search.rs, tests.rs]
    ImportedBy: [alacritty/src/event.rs, alacritty_terminal/src/lib.rs, content.rs, input/mod.rs, term/mod.rs]
  storage.rs:
    Imports: [builtin_font.rs, monitor.rs, session_log.rs]
    ImportedBy: [grid/mod.rs, resize.rs]
  sync.rs:
    Imports: [monitor.rs]
    ImportedBy: [alacritty_terminal/src/lib.rs, event_loop.rs]
  term/color.rs:
    Imports: []
    ImportedBy: [term/mod.rs]
  term/mod.rs:
    Imports: [alacritty_config/src/lib.rs, alacritty_terminal/src/event.rs, cell.rs, config/bell.rs, config/cursor.rs, damage.rs, index.rs, interceptor.rs, message_bar.rs, monitor.rs, policy.rs, resize.rs, row.rs, search.rs, src/selection.rs, term/color.rs, tests.rs, vi_mode.rs]
    ImportedBy: [alacritty_terminal/src/lib.rs]
  tests.rs:
    Imports: [config/bell.rs, content.rs, grid/mod.rs, monitor.rs, resize.rs, row.rs]
    ImportedBy: [damage.rs, grid/mod.rs, resize.rs, row.rs, src/selection.rs, term/mod.rs, vi_mode.rs]
  thread.rs:
    Imports: [monitor.rs, polling/mod.rs]
    ImportedBy: [alacritty_terminal/src/lib.rs, blocking.rs, event_loop.rs, monitor.rs, polling/mod.rs]
  tty/mod.rs:
    Imports: [config/cursor.rs, unix.rs, windows/mod.rs]
    ImportedBy: [alacritty_terminal/src/lib.rs, main.rs]
  unix.rs:
    Imports: [config/cursor.rs, gles2.rs, message_bar.rs, monitor.rs, polling/mod.rs]
    ImportedBy: [event_loop.rs, tty/mod.rs]
  vi_mode.rs:
    Imports: [builtin_font.rs, config/bell.rs, index.rs, monitor.rs, row.rs, tests.rs]
    ImportedBy: [alacritty/src/event.rs, alacritty_terminal/src/lib.rs, input/mod.rs, term/mod.rs]
  windows/mod.rs:
    Imports: [blocking.rs, child.rs, config/bell.rs, conpty.rs, event_loop.rs, message_bar.rs, monitor.rs]
    ImportedBy: [tty/mod.rs]
  # --- Subproject -- roy ---
  denial.rs:
    Imports: [monitor.rs]
    ImportedBy: [policy.rs, roy/src/lib.rs, session_log.rs]
  interceptor.rs:
    Imports: [alacritty_config/src/lib.rs, config/cursor.rs, monitor.rs]
    ImportedBy: [alacritty/src/event.rs, atlas.rs, damage.rs, display/mod.rs, event_loop.rs, gles2.rs, glsl3.rs, glyph_cache.rs, hint.rs, ipc.rs, polling/mod.rs, rects.rs, roy/src/lib.rs, session.rs, term/mod.rs, window_context.rs]
  policy.rs:
    Imports: [denial.rs, monitor.rs]
    ImportedBy: [bindings.rs, cell.rs, gles2.rs, glsl3.rs, keyboard.rs, renderer/mod.rs, roy/src/lib.rs, session.rs, term/mod.rs]
  roy/Cargo.toml:
    Imports: []
    ImportedBy: []
  roy/src/lib.rs:
    Imports: [denial.rs, interceptor.rs, policy.rs, session.rs, session_log.rs, src/config.rs]
    ImportedBy: []
  session.rs:
    Imports: [interceptor.rs, monitor.rs, policy.rs, row.rs, src/config.rs]
    ImportedBy: [alacritty/src/event.rs, roy/src/lib.rs, window_context.rs]
  session_log.rs:
    Imports: [denial.rs, monitor.rs]
    ImportedBy: [alacritty/src/event.rs, cli.rs, input/mod.rs, logging.rs, ref.rs, resize.rs, row.rs, roy/src/lib.rs, serde_utils.rs, storage.rs, window_context.rs]
  src/config.rs:
    Imports: [clipboard.rs, config/bell.rs, config/cursor.rs, monitor.rs]
    ImportedBy: [roy/src/lib.rs, session.rs]
```
