# cosmic-crypt

**Malware dev repository - Just the basics - And just for fun**
[Discord](https://discord.gg/CxjUAnVh8g)

### What I already know

- C and CPP programming basics 
- Java:star:, Python:star: and JS programming
- JS libraries: Many small ones, React, NodeJS, ExpressJS, ReactJS,
- Java libraries: Spring, Hibernate, JPA, JSP, Servlets, JSTL, JUnit, Mockito, PowerMockito, Jmeter
- Need to build my resume again ... huh?

@2024May17

- [x] C and CPP programming basics :check:
- [x] Rust programming basics :check:
- [x] Reqd tools
  - [x] VS Code
  - [x] Git
  - [x] Co-pilot
  - [x] Rust - Cargo, RustRover(JetBrains)
  - [x] x64dbg
  - [ ] Ghidra
  - [x] VirtualBox
  - [ ] OS setup - virtualized
    - [ ] Windows - Test
    - [ ] Kali - Offence
    - [ ] Kali - Test
    - [ ] Ubuntu - Test
    - [ ] Fedora - Test
  - [ ] PE Bear
  - [ ] Process Hacker
- General tools
  - [ ] Java - JDK(v8, v9, v11, v18, v21), IntelliJ
  - [ ] Python - VS Code, Python3, Anaconda
  - [ ] JS - VS Code, NodeJS, ReactJS, ExpressJS, `npm`, `nvm`
  - [ ] C and CPP - VS Code, GCC, G++, CMake
  - [ ] DB - DataGrip(Jetbrains), PostgreSQL, ES, MongoDB, Redis
  - [x] Text editor : Atom
  - [ ] Docker
  - [x] Postman
  - [x] Powershell
  - [x] Video edits: OBS, Handbrake
  - [x] Browser: Chrome, Brave, Firefox

# @2024May18

**Took CPP and Rust refresher**

### Windows memory management

- OS is 'virtually' mapped to memory, not 'physically'
- Memory mgmt: Paging, Segmentation, Virtual memory
- Page state: Free(available for use), Reserved(reserved for objects, arrays, etc but not in use currently), Committed(reserved and in use)
- Page protection: PAGE_EXECUTE_READWRITE(enables code exec, reads and writes), PAGE_READWRITE(reads and writes), PAGE_READONLY(reads only), NO_ACCESS(no access), [more](https://docs.microsoft.com/en-us/windows/win32/memory/memory-protection-constants)
- Built-in memory protection: DEP(Data Execution Prevention), ASLR(Address Space Layout Randomization), CFG(Control Flow Guard), SafeSEH(Safe Structured Exception Handling)
- x86 -> 4GB memory, x64 -> 2^64 bytes memory
- Memory alloc with **CPP**::`malloc`, `HeapAlloc`, `VirtualAlloc`, `LocalAlloc`, `GlobalAlloc`, etc., **Rust**:: `Box::new`, `Arc::new`, `Rc::new`, etc
- Write to m/m via pointers **CPP**::`*p = 10;`, `memcpy`, `memset`, etc., **Rust**:: `*p = 10;`, `ptr::write`, `ptr::write_volatile`, `ptr::copy`, `ptr::copy_nonoverlapping`, etc
- Free m/m with **CPP**::`free`, `HeapFree`, `VirtualFree`, `LocalFree`, `GlobalFree`, etc., **Rust**:: `drop`, `Box::drop`, `Arc::drop`, `Rc::drop`, etc
- ...

### WindowsAPI(CPP), WinAPI(Rust)

- It has its own impl of data types, functions, etc
- Data type pointers:: `DWORD -> *DWORD = PDWORD`, `SIZE_T -> *SIZE_T = PSIZE_T`, `HANDLE -> *HANDLE = PHANDLE`, etc
- ANSI and UNICODE strings:: `LPSTR`, `LPCSTR`, `LPWSTR`, `LPCWSTR`, etc
- IN and OUT params:: `IN`, `OUT`, `INOUT`, etc

```cpp
#include <windows.h> // For WindowsAPI - CPP
use winapi::um::winuser; // For WinAPI - Rust
```

### PE format

Generally speaking, a PE file is format used by programmes and dependencies.
**General format** :: Changes depending on OS, compiler, etc

```
---
DosHeader
---
DosStub
---
NTHeader
    ---
    NtSignature
    ---
    FileHeader
    ---
    OptionalHeader
    ---
DataDirectories
    ... other sections
---
Sections
    ... other sections
---
```

‼️To read more
- PE Overview - https://0xrick.github.io/win-internals/pe2/
- DOS Header, DOS Stub and Rich Header - https://0xrick.github.io/win-internals/pe3/
- NT Headers - https://0xrick.github.io/win-internals/pe4/
- Data Directories, Section Headers and Sections - https://0xrick.github.io/win-internals/pe5/
- PE Imports (Import Directory Table, ILT, IAT) - https://0xrick.github.io/win-internals/pe6/

### DLL :: ~ to libraries, dependendent .jar's, crates, etc

- Dynamic Link Library :: Can't run on its own, needs an exe to import and use it
- `ntdll.dll`, `kernel32.dll`, etc
- A common DLL will be used by many processes, so it's loaded into memory only once

