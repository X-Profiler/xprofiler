#ifdef _WIN32
#include <Windows.h>
#include <dbghelp.h>
#include <Lm.h>
#include <psapi.h>

#include <sstream>
#include <string>

#include "library/writer.h"

namespace xprofiler {
using std::ostringstream;
using std::string;

static const int kMaxFrams = 256;

string GetPcAddress(void* pc) {
  char buf[64];
  snprintf(buf, sizeof(buf), "0x%p", pc);
  return (string)buf;
}

void PrintNativeStack(JSONWriter* writer) {
  HANDLE hProcess = GetCurrentProcess();
  SymSetOptions(SYMOPT_LOAD_LINES | SYMOPT_UNDNAME | SYMOPT_DEFERRED_LOADS);
  SymInitialize(hProcess, nullptr, TRUE);

  void* frames[kMaxFrams];
  WORD numberOfFrames = CaptureStackBackTrace(2, kMaxFrams, frames, nullptr);

  writer->json_arraystart("nativeStacks");
  for (int i = 0; i < numberOfFrames; i++) {
    writer->json_start();
    writer->json_keyvalue("os", "win32");

    DWORD64 dwOffset64 = 0;
    DWORD64 dwAddress = reinterpret_cast<DWORD64>(frames[i]);
    char buffer[sizeof(SYMBOL_INFO) + MAX_SYM_NAME * sizeof(TCHAR)];
    PSYMBOL_INFO pSymbol = reinterpret_cast<PSYMBOL_INFO>(buffer);
    pSymbol->SizeOfStruct = sizeof(SYMBOL_INFO);
    pSymbol->MaxNameLen = MAX_SYM_NAME;

    char buf[64];
    snprintf(buf, sizeof(buf), "0x%p", reinterpret_cast<void*>(dwAddress));
    writer->json_keyvalue("pcAddress", buf);

    if (SymFromAddr(hProcess, dwAddress, &dwOffset64, pSymbol)) {
      DWORD dwOffset = 0;
      IMAGEHLP_LINE64 line;
      line.SizeOfStruct = sizeof(line);

      writer->json_keyvalue("symbolName", pSymbol->Name);

      if (SymGetLineFromAddr64(hProcess, dwAddress, &dwOffset, &line)) {
        writer->json_keyvalue("offset", dwOffset);
        writer->json_keyvalue("url", line.FileName);
        writer->json_keyvalue("lineNumber", line.LineNumber);
      } else {
        writer->json_keyvalue("offset", dwOffset64);
      }
    }

    writer->json_end();
  }
  writer->json_arrayend();
}

void PrintSystemEnv(JSONWriter* writer) {
  writer->json_arraystart("env");

  LPTSTR lpszVariable;
  LPTCH lpvEnv;

  lpvEnv = GetEnvironmentStrings();
  if (lpvEnv != nullptr) {
    lpszVariable = reinterpret_cast<LPTSTR>(lpvEnv);
    while (*lpszVariable) {
      writer->json_element(lpszVariable);
      lpszVariable += lstrlen(lpszVariable) + 1;
    }
    FreeEnvironmentStrings(lpvEnv);
  }

  writer->json_arrayend();
}

void PrintResourceLimits(JSONWriter* writer) {
  writer->json_arraystart("resourceLimits");
  writer->json_arrayend();
}

void PrintLoadedLibraries(JSONWriter* writer) {
  writer->json_arraystart("loadedLibraries");

  HANDLE process_handle =
      OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE,
                  GetCurrentProcessId());
  if (process_handle == NULL) return;
  DWORD size_1 = 0, size_2 = 0;
  if (EnumProcessModules(process_handle, NULL, 0, &size_1)) {
    HMODULE* modules = (HMODULE*)malloc(size_1);
    if (modules == NULL) return;
    if (EnumProcessModules(process_handle, modules, size_1, &size_2)) {
      for (int i = 0;
           i < (size_1 / sizeof(HMODULE)) && i < (size_2 / sizeof(HMODULE));
           i++) {
        TCHAR module_name[MAX_PATH];
        if (GetModuleFileNameEx(process_handle, modules[i], module_name,
                                sizeof(module_name) / sizeof(TCHAR))) {
          writer->json_element(module_name);
        }
      }
    }
    free(modules);
  }

  writer->json_arrayend();
}

string GetOsVersion() {
  ostringstream data;

  const DWORD level = 101;
  LPSERVER_INFO_101 os_info = NULL;
  NET_API_STATUS nStatus = NetServerGetInfo(NULL, level, (LPBYTE*)&os_info);
  if (nStatus == NERR_Success) {
    std::string os_name = "Windows";
    const DWORD major = os_info->sv101_version_major & MAJOR_VERSION_MASK;
    const DWORD type = os_info->sv101_type;
    const bool isServer = (type & SV_TYPE_DOMAIN_CTRL) ||
                          (type & SV_TYPE_DOMAIN_BAKCTRL) ||
                          (type & SV_TYPE_SERVER_NT);
    switch (major) {
      case 5:
        switch (os_info->sv101_version_minor) {
          case 0:
            os_name = "Windows 2000";
            break;
          default:
            os_name = (isServer ? "Windows Server 2003" : "Windows XP");
        }
        break;
      case 6:
        switch (os_info->sv101_version_minor) {
          case 0:
            os_name = (isServer ? "Windows Server 2008" : "Windows Vista");
            break;
          case 1:
            os_name = (isServer ? "Windows Server 2008 R2" : "Windows 7");
            break;
          case 2:
            os_name = (isServer ? "Windows Server 2012" : "Windows 8");
            break;
          case 3:
            os_name = (isServer ? "Windows Server 2012 R2" : "Windows 8.1");
            break;
          default:
            os_name = (isServer ? "Windows Server" : "Windows Client");
        }
        break;
      case 10:
        os_name = (isServer ? "Windows Server 2016" : "Windows 10");
        break;
      default:
        os_name = (isServer ? "Windows Server" : "Windows Client");
    }
    data << os_name.c_str();

    // Convert and print the machine name and comment fields (these are LPWSTR
    // types)
    size_t count;
    char name_buf[256];
    wcstombs_s(&count, name_buf, sizeof(name_buf), os_info->sv101_name,
               _TRUNCATE);
    if (os_info->sv101_comment != NULL) {
      char comment_buf[256];
      wcstombs_s(&count, comment_buf, sizeof(comment_buf),
                 os_info->sv101_comment, _TRUNCATE);
      data << " / " << name_buf << " " << comment_buf;
    } else {
      data << " / " << name_buf;
    }

    if (os_info != NULL) {
      NetApiBufferFree(os_info);
    }
  } else {
    // NetServerGetInfo() call failed, fallback to use GetComputerName() instead
    TCHAR machine_name[256];
    DWORD machine_name_size = 256;
    data << "Windows";
    if (GetComputerName(machine_name, &machine_name_size)) {
      data << " / " << machine_name;
    }
  }

  string detail = data.str();
  return detail;
}

}  // namespace xprofiler

#endif
