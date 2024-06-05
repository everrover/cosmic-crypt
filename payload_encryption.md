# cosmic-crypt

https://www.freecodecamp.org/news/how-to-install-c-and-cpp-compiler-on-windows/

**Malware dev repository - Just the basics - And just for fun**
[Discord](https://discord.gg/CxjUAnVh8g)

- XOR encryption
- AES encryption
  /*
   * 
   * Using AES here ... AES128, AES192, AES256, etc.
   * * Symmetric encryption - same key for both encryption and decryption
   * * Block cipher modes possible - ECB, GCM, etc. - Needs init. vector(IV)
   * * Key exchange required
   * * Used `bCrypt` and `Tiny Aes Project` libraries
   * 
   * [Most commonly used]
   */
- RC4 encryption
  /*
   * 
   * Using RC4 here ...
   * * Fast, bidirectional stream cipher
   * * Doesn't require a key exchange/any other overhead
   * 
   * [Now obsolete]
   */



Similar techniques

- Relay based function-stack or function-pointer encryption
- Function obfuscation
- Using personal/less-commonly used enc/obfuscation algorithms
- Using fetching payload at runtime/dynamically/over network/via obfuscation
