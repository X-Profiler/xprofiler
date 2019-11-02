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
        "src/logbypass/log.cc",
        "src/logbypass/cpu.cc",
        "src/logbypass/heap.cc"
      ],
      "include_dirs": [ '<!(node -e "require(\'nan\')")' ],
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
          "sources": [
            "src/platform/unix/cpu.cc",
            "src/platform/unix/utils.cc"
          ]
        }],
        ["OS == 'win'", {
          "libraries": [ "dbghelp.lib", "Netapi32.lib", "PsApi.lib", "Ws2_32.lib" ],
          "dll_files": [ "dbghelp.dll", "Netapi32.dll", "PsApi.dll", "Ws2_32.dll" ],
          "sources": [
            "src/platform/win/cpu_win.cc",
            "src/platform/win/utils_win.cc",
          ]
        }],
      ],
      "defines": [
        'XPROFILER_VERSION="<!(node -p \"require(\'./package.json\').version\")"'
      ],
    },
  ],
}
