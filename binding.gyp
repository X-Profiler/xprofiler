{
  "targets": [
    {
      "target_name": "xprofiler",
      'win_delay_load_hook': 'false',
      "sources": [
        "src/xprofiler.cc",
        "src/configure.cc",
        "src/logger.cc"
      ],
      "include_dirs": [ '<!(node -e "require(\'nan\')")' ],
      "conditions": [
        ["OS=='linux'", {
          "defines": [ "_GNU_SOURCE" ],
          "cflags": [ "-O2", "-std=c++11" ],
        }],
        ["OS=='win'", {
          "libraries": [ "dbghelp.lib", "Netapi32.lib", "PsApi.lib", "Ws2_32.lib" ],
          "dll_files": [ "dbghelp.dll", "Netapi32.dll", "PsApi.dll", "Ws2_32.dll" ],
        }],
        ["OS=='zos'", {
          "cflags!": [ "-O2", "-O3" ],
          "cflags": [ "-qascii" ],
        }],
      ],
      "defines": [
        'XPROFILER_VERSION="<!(node -p \"require(\'./package.json\').version\")"'
      ],
    },
  ],
}
