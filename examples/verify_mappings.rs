//! Test character mappings against actual standards

use unikey_encoding::standard_mappings::StandardMappings;

fn main() {
    println!("🔍 Testing Standard Vietnamese Character Mappings");
    println!("{}", "=".repeat(50));
    
    let mappings = StandardMappings::new();
    
    // Test TCVN3 mappings
    println!("\n📋 TCVN3 Character Mappings:");
    test_encoding_mappings("TCVN3", &mappings.tcvn3);
    
    // Test VNI-Win mappings
    println!("\n📋 VNI-Win Character Mappings:");
    test_encoding_mappings("VNI-Win", &mappings.vni_win);
    
    // Test VISCII mappings
    println!("\n📋 VISCII Character Mappings:");
    test_encoding_mappings("VISCII", &mappings.viscii);
    
    // Test VPS mappings
    println!("\n📋 VPS Character Mappings:");
    test_encoding_mappings("VPS", &mappings.vps);
    
    // Test BKHCM mappings
    println!("\n📋 BKHCM Character Mappings:");
    test_encoding_mappings("BKHCM", &mappings.bkhcm);
    
    // Test CP1258 mappings
    println!("\n📋 CP1258 Character Mappings:");
    test_encoding_mappings("CP1258", &mappings.cp1258);
    
    println!("\n✅ Character mapping verification completed!");
}

fn test_encoding_mappings(name: &str, mappings: &std::collections::HashMap<u32, u8>) {
    println!("  {} mappings: {} characters", name, mappings.len());
    
    // Test some common Vietnamese characters
    let test_chars = vec![
        ('á', 0x00E1),
        ('à', 0x00E0),
        ('ả', 0x1EA3),
        ('ã', 0x00E3),
        ('ạ', 0x1EA1),
        ('ă', 0x0103),
        ('â', 0x00E2),
        ('đ', 0x0111),
        ('ê', 0x00EA),
        ('ô', 0x00F4),
        ('ơ', 0x01A1),
        ('ư', 0x01B0),
    ];
    
    println!("  Testing common characters:");
    for (char_display, unicode) in test_chars {
        if let Some(&byte) = mappings.get(&unicode) {
            println!("    {} (U+{:04X}) -> 0x{:02X}", char_display, unicode, byte);
        } else {
            println!("    {} (U+{:04X}) -> NOT FOUND", char_display, unicode);
        }
    }
}
