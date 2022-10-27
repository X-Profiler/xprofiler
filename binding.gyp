{
    "targets": [
        {
            "target_name": "xprofiler",
            "win_delay_load_hook": "false",
            "sources": [
                "src/environment_data.cc",
                "src/environment_registry.cc",
                "src/process_data.cc",
                "src/xpf_thread.cc",
                "src/xprofiler.cc",
                "src/logger.cc",
                "src/util.cc",
                "src/library/json.hpp",
                "src/library/error.cc",
                "src/library/common.cc",
                "src/library/utils.cc",
                "src/library/writer.cc",
                "src/logbypass/log.cc",
                "src/logbypass/cpu.cc",
                "src/logbypass/heap.cc",
                "src/logbypass/gc.cc",
                "src/logbypass/libuv.cc",
                "src/logbypass/http.cc",
                "src/commands/listener.cc",
                "src/commands/send.cc",
                "src/commands/parser.cc",
                "src/commands/dump.cc",
                "src/commands/simple/version.cc",
                "src/commands/simple/registry.cc",
                "src/commands/simple/config.cc",
                "src/commands/cpuprofiler/cpu_profiler.cc",
                "src/commands/cpuprofiler/cpu_profile.cc",
                "src/commands/cpuprofiler/cpu_profile_node.cc",
                "src/commands/heapdump/heap_profiler.cc",
                "src/commands/heapdump/heap_snapshot.cc",
                "src/commands/heapprofiler/sampling_heap_profiler.cc",
                "src/commands/gcprofiler/gc_profiler.cc",
                "src/commands/report/node_report.cc",
                "src/commands/report/javascript_stack.cc",
                "src/commands/report/native_stack.cc",
                "src/commands/report/heap_statistics.cc",
                "src/commands/report/uv_statistics.cc",
                "src/commands/report/system_statistics.cc",
                "src/commands/coredumper/coredumper.cc",
                "src/hooks/set_hooks.cc",
                "src/hooks/fatal_error.cc",
                "src/jsapi/configure.cc",
                "src/jsapi/export_logger.cc",
                "src/jsapi/http.cc",
            ],
            "include_dirs": [
                'src',
                '<!(node -e "require(\'nan\')")'
            ],
            "cflags_cc!": ["-fno-exceptions"],
            "conditions": [
                ["OS == 'linux'", {
                    "cflags": [
                        "-O3",
                        "-std=c++17",
                        "-Wno-sign-compare",
                        "-Wno-cast-function-type",
                    ],
                    "defines": [
                        'XPROFILER_IPC_PATH="<!(node -p \"require(\'./package.json\').xctlIpcPath.unix\")"',
                    ],
                    "sources": [
                        "src/platform/unix/cpu.cc",
                        "src/platform/unix/utils.cc",
                        "src/platform/unix/ipc.cc",
                        "src/platform/unix/report.cc",
                        "src/platform/unix/core/linux/dump.cc",
                        "src/platform/unix/core/linux/coredumper.cc",
                        "src/platform/unix/core/linux/elfcore.cc",
                        "src/platform/unix/core/linux/linuxthreads.cc",
                        "src/platform/unix/core/linux/thread_lister.cc"
                    ]
                }],
                ["OS == 'mac'", {
                    "xcode_settings": {
                        "GCC_ENABLE_CPP_EXCEPTIONS": "YES",
                        "GCC_OPTIMIZATION_LEVEL": "3",
                        "OTHER_CFLAGS": [
                            "-std=c++17",
                            "-Wconversion",
                            "-Wno-sign-conversion",
                        ]
                    },
                    "defines": [
                        'XPROFILER_IPC_PATH="<!(node -p \"require(\'./package.json\').xctlIpcPath.unix\")"',
                    ],
                    "sources": [
                        "src/platform/unix/cpu.cc",
                        "src/platform/unix/utils.cc",
                        "src/platform/unix/ipc.cc",
                        "src/platform/unix/report.cc",
                        "src/platform/unix/core/darwin.cc",
                    ]
                }],
                ["OS == 'win'", {
                    "libraries": ["dbghelp.lib", "Netapi32.lib", "PsApi.lib", "Ws2_32.lib"],
                    "dll_files": ["dbghelp.dll", "Netapi32.dll", "PsApi.dll", "Ws2_32.dll"],
                    "msvs_settings": {
                        "VCCLCompilerTool": {
                            "ExceptionHandling": "2",
                            "Optimization": "3",
                        },
                    },
                    "defines": [
                        'XPROFILER_IPC_PATH="<!(node -p \"require(\'./package.json\').xctlIpcPath.win32\")"',
                        "WIN32_LEAN_AND_MEAN=1",
                    ],
                    "sources": [
                        "src/platform/win/cpu_win.cc",
                        "src/platform/win/utils_win.cc",
                        "src/platform/win/ipc_win.cc",
                        "src/platform/win/report_win.cc",
                        "src/platform/win/core_win.cc"
                    ]
                }],
            ],
            "defines": [
                'XPROFILER_VERSION="<!(node -p \"require(\'./package.json\').version\")"',
                'XPROFILER_BLURRY_TAG="<!(node -p \"require(\'./package.json\').blurryTag\")"',
            ],
        },
        {
            "target_name": "action_after_build",
            "type": "none",
            "dependencies": ["<(module_name)"],
            "copies": [
                {
                    "files": ["<(PRODUCT_DIR)/<(module_name).node"],
                    "destination": "<(module_path)"
                }
            ]
        },
    ],
}
