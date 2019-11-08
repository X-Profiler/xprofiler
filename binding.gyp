{
  "targets": [
    {
      "target_name": "xprofiler",
      'win_delay_load_hook': 'false',
      "sources": [
        "src/xprofiler.cc",
        "src/configure.cc",
        "src/logger.cc",
        "src/utils.cc",
        "src/common.cc",
        "src/library/json.hpp",
        "src/logbypass/log.cc",
        "src/logbypass/cpu.cc",
        "src/logbypass/heap.cc",
        "src/logbypass/gc.cc",
        "src/logbypass/libuv.cc",
        "src/commands/listener.cc",
        "src/commands/send.cc",
        "src/commands/parser.cc",
      ],
      "include_dirs": [ '<!(node -e "require(\'nan\')")' ],
      'cflags_cc!': [ '-fno-exceptions' ],
      "conditions": [
        ["OS == 'linux'", {
          "defines": [ "_GNU_SOURCE" ],
          "cflags": [ "-O2", "-std=c++11" ],
          "sources": [
            "src/platform/unix/cpu.cc",
            "src/platform/unix/utils.cc"
          ]
        }],
        ["OS == 'mac'", {
          'xcode_settings': {
            'GCC_ENABLE_CPP_EXCEPTIONS': 'YES'
          },
          "sources": [
            "src/platform/unix/cpu.cc",
            "src/platform/unix/utils.cc"
          ]
        }],
        ["OS == 'win'", {
          "libraries": [ "dbghelp.lib", "Netapi32.lib", "PsApi.lib", "Ws2_32.lib" ],
          "dll_files": [ "dbghelp.dll", "Netapi32.dll", "PsApi.dll", "Ws2_32.dll" ],
          'msvs_settings': {
              'VCCLCompilerTool': {
              'ExceptionHandling': '2',
            },
          },
          "sources": [
            "src/platform/win/cpu_win.cc",
            "src/platform/win/utils_win.cc",
            "src/platform/win/ipc_win.cc",
          ]
        }],
      ],
      "defines": [
        'XPROFILER_VERSION="<!(node -p \"require(\'./package.json\').version\")"'
      ],
    },
  ],
}
