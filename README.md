# cosmic-crypt

‼️Windows or any other OS will attempt to flag this repo as malicious. Please use a VM or a sandbox to run this code after hardening. Above all, use it at your own risk.‼️

**Malware dev repository - Just the basics - And just for fun**
[Discord](https://discord.gg/CxjUAnVh8g)

## Table of contents

- [General info](./README.md)
- [Payload encryption](./payload_encryption.md)
- [Payload obfuscation](./payload_obfuscation.md)
- 

### Kali m/c

```
hostname: kali
username: goku
password: vegetam3
```

### Windows m/c

```
hostname: mando
username: mandolorian
password: babyYoda❌
```

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

https://itehax.com/blog/portable-executable-explained-throught-rust-code

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
- A common DLL will be used by many processes, so it's loaded into memory only once.
  - System-wide DLLs and libs
  - Process-specific DLLs and libs

(Nothin' I didn't know)

### Detection mechanisms and evasion techniques

- Static signature-based detection - Hashing, pattern matching, etc
  - Evasion via modding shellcode -> obfuscation, hashing, etc
- Heuristic-based detection 
  - Decompiling and signature matching - static
  - Behavioural-based detection - dynamic
- Behaviour-based detection - Monitoring system calls, network traffic, etc - Monitored in sandbox
  - Evasion via modding behaviour -> sleep, delay, parallel-exec, re-ordering op to match some genuine app's behaviour, etc
- API hooking - Monitoring commonly used APIs
  - Evasion via modding API call patterns -> hooking, detouring, etc
  - IAT checks - Import Address Table checks
    - Evasion via **API hashing and obfuscation?**

### Windows Process

- Process, Threads, Memory
- M/M
  - Virtual and physical
  - Types
    - Private - process-specific
    - Shared/Mapped - shared between processes - restricts W
    - Image - executable file
- PEB(Process Environment Block)
  - Process info, m/m info, etc
  - Imp flags(name is diff in Rust)
    - BeingDebugged - Is debugger attached?
    - Ldr(`_PEB_LDR_DATA`) - List of .dll dependencies. These PIDs can be found and the .dll's modified to load malicious shellcode and threads
    - ProcessParameters(`RTL_USER_PROCESS_PARAMETERS`) - List of args passed to process - can be modified to load malicious shellcode or specific paths/params
    - `PostProcessInitRoutine` - Callback that runs after all threads and process init completes
    - `AtlThunkSListPtr` - Pointer to ATL thunk list
    - `SessionId` - Session ID per process per user used for **Tracking user activity**
- TEB(Thread Environment Block)
  - Thread info, m/m info, etc
  - Imp flags
    - `TlsSlots` - Thread Local Storage slots - thread specific data
    - `TlsExpansionSlots` - Expansion slots for TLS - thread specific data for associated .dll's
- Closing handles(of either process or thread) is essential to avoid memory leaks

### Using un-official docs and sources

> One may encounter several reserved members within the structure. These reserved members are often presented as arrays 
> of BYTE or PVOID data types. This practice is implemented by Microsoft to maintain confidentiality and prevent users
> from understanding the structure to avoid modifications to these reserved members.

👆👆 for Linux and other OS's

[undocumented.ninternals](https://web.archive.org/web/20230401045934/http://undocumented.ntinternals.net/)
[Process hacker](https://github.com/winsiderss/systeminformer/tree/master/phnt/include)
[ReactOSs docs](https://doxygen.reactos.org/globals_type.html)
[Vergilius project](https://www.vergiliusproject.com/)

- Remember m/c arch x64/x86/ARM/etc. specific defn.s may be used
- These defn.s have to be implemented ourselves at times, at times have to be imported specifically
- Multiple defn.s or custom defn.s may be used for the same structure, and have to be specifically imported/created

### Payload inclusion

- Several sections available in a PE file
  - `.text` - Code
    - Specific instructions have to be provided
    - `-x` permission applicable on data stored in this section
    - Allows shellcode to be run directly here
    - Limited space
```rust
extern crate winapi;

use winapi::um::memoryapi::{VirtualAlloc};
use winapi::um::winnt::{PAGE_EXECUTE_READWRITE, MEM_COMMIT, MEM_RESERVE};
use std::mem::transmute;

fn main() {
    // Define the shellcode
    let shellcode: [u8; 5] = [0x90, 0x90, 0x90, 0x90, 0xC3]; // Replace with your shellcode
  
    // Allocate memory for the shellcode
    let mem = unsafe {
        VirtualAlloc(
            std::ptr::null_mut(),
            shellcode.len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        )
    };

    // Copy the shellcode into the newly allocated memory
    unsafe {
        std::ptr::copy_nonoverlapping(shellcode.as_ptr(), mem as *mut u8, shellcode.len());
    }

    // Cast the pointer to a function pointer and call it
    let func: extern "system" fn() = unsafe { transmute(mem) };
    func();
}
```
  - `.data` - Data - R/W
    - contains global and static vars
    - store shell code(encrypted, obfuscated, etc) and decrypt it at runtime
  - `.rdata` - Read-only data `const`
    - `.rdata` and `.data` are merged in some cases within `.data` or even `.text`
  - `.rsrc` - Resources

## Table of contents

- [General info](./README.md)
- [Payload encryption](./payload_encryption.md)
- [Payload obfuscation](./payload_obfuscation.md)
- 

### Kali m/c

```
hostname: kali
username: goku
password: vegetam3
```

### Windows m/c

```
hostname: mando
username: mandolorian
password: babyYoda❌
```