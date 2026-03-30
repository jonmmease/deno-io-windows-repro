pub use deno_io;

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};

    #[test]
    fn test_pipe_hello_world() {
        let (mut reader, mut writer) = deno_io::pipe().unwrap();
        writer.write_all(b"hello world").unwrap();
        let mut buf = [0u8; 11];
        reader.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, b"hello world");
    }
}
