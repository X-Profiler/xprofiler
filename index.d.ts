export as namespace xprofiler;

export interface XprofilerConfig {
  log_dir?: string;
  log_interval?: number;
  log_level?: 0 | 1 | 2;
  log_type?: 0 | 1;
  log_format_alinode?: boolean;
  enable_log_uv_handles?: boolean;
  patch_http?: boolean;
  patch_http_timeout?: number;
  check_throw?: boolean;
  auto_incr_heap_limit_size?: number;
  enable_fatal_error_hook?: boolean;
  enable_fatal_error_report?: boolean;
  enable_fatal_error_coredump?: boolean;
  enable_http_profiling?: boolean;
  enable_auto_incr_heap_limit?: boolean;
}

/**
 * Start xprofiler, which contains setting config and hooks, running log thread and the receiver of the commands.
 * @example
 * // without any config
 * xprofiler.start();
 * // set your own log dir
 * xprofiler.start({log_dir: '/path/to/your/logdir'});
 * @param config Xprofiler config.
 */
export function start(config?: XprofilerConfig): void;

/**
 * Print info logs.
 * @example
 * // will out put: [2019-12-24 11:22:28] [info] [http] [57639] [1.0.0-prepare] send 13 request in last min.
 * xprofiler.info('http', 'send 13 request in last min.');
 * @param type Info log content type.
 * @param content Info log content.
 */
export function info(type: string, content: string): void;

/**
 * Print error logs.
 * @example
 * // will out put: [2019-12-24 11:24:39] [error] [parser] [57726] [1.0.0-prepare] json parse failed.
 * xprofiler.error('parser', 'json parse failed.');
 * @param type Error log content type.
 * @param content Error log content.
 */
export function error(type: string, content: string): void;

/**
 * Print debug logs.
 * @example
 * // will out put: [2019-12-24 11:25:33] [debug] [command] [57787] [1.0.0-prepare] receive command: test.
 * xprofiler.debug('command', 'receive command: test.');
 * @param type Debug log content type.
 * @param content Debug log content.
 */
export function debug(type: string, content: string): void;

/**
 * Set xprofiler config.
 * @param config Xprofiler config.
 */
export function setConfig(config?: XprofilerConfig): XprofilerConfig;

/**
 * Get xprofiler config.
 */
export function getXprofilerConfig(): XprofilerConfig;

/**
 * Start the bypass log thread via uv_thread_create().
 */
export function runLogBypass(): void;

/**
 * Start the receiver thread of the commands via uv_thread_create().
 */
export function runCommandsListener(): void;

/**
 * Enable hooks for some running states.
 */
export function setHooks(): void;
