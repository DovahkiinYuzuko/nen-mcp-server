use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use anyhow::Result;
use super::validate_and_canonicalize;

pub fn read_hex(path: &str, offset: Option<u64>, length: Option<usize>) -> Result<String> {
    let path_buf = validate_and_canonicalize(path)?;
    let mut file = File::open(&path_buf).map_err(|e| {
        anyhow::anyhow!("Open Error: Failed to open '{}' for binary reading. Details: {}", path, e)
    })?;
    let start_offset = offset.unwrap_or(0);
    let read_len = length.unwrap_or(256);
    
    file.seek(SeekFrom::Start(start_offset)).map_err(|e| {
        anyhow::anyhow!("Seek Error: Failed to seek to offset {} in file '{}'. Details: {}", start_offset, path, e)
    })?;
    
    let mut buffer = vec![0u8; read_len];
    let bytes_read = file.read(&mut buffer).map_err(|e| {
        anyhow::anyhow!("Read Error: Failed to read binary data from '{}' at offset {}. Details: {}", path, start_offset, e)
    })?;
    buffer.truncate(bytes_read);
    
    let mut output = String::new();
    for (i, chunk) in buffer.chunks(16).enumerate() {
        let current_offset = start_offset + (i * 16) as u64;
        
        // Address
        output.push_str(&format!("{:08x}: ", current_offset));
        
        // Hex values
        for j in 0..16 {
            if j < chunk.len() {
                output.push_str(&format!("{:02x} ", chunk[j]));
            } else {
                output.push_str("   ");
            }
            if j == 7 {
                output.push(' ');
            }
        }
        
        output.push_str(" ");
        
        // ASCII values
        for &b in chunk {
            if b >= 32 && b <= 126 {
                output.push(b as char);
            } else {
                output.push('.');
            }
        }
        
        output.push('\n');
    }
    
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::fs;

    #[test]
    fn test_read_hex_logic() -> Result<()> {
        let test_file = "test_hex.tmp";
        let data = b"Hello, World!\x01\x02\x03\x04";
        let mut file = fs::File::create(test_file)?;
        file.write_all(data)?;
        
        let result = read_hex(test_file, Some(0), Some(17))?;
        println!("Hex output:\n{}", result);
        
        // クリーンアップ
        let _ = fs::remove_file(test_file);

        // 検証
        assert!(result.contains("00000000:"));
        // Check for specific hex parts
        assert!(result.contains("48 65 6c 6c 6f"));
        assert!(result.contains("57  6f 72 6c 64 21")); // Note the double space after 57 (index 7)
        assert!(result.contains("Hello, World!"));
        Ok(())
    }
}
