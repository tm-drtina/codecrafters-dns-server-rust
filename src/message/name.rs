#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name {
    parts: Vec<Vec<u8>>,
    pointer: Option<u16>,
}

impl Name {
    pub fn write(&self, buf: &mut Vec<u8>) {
        for part in &self.parts {
            buf.push(part.len() as u8);
            buf.extend(part);
        }
        if let Some(pointer) = self.pointer {
            let bytes = pointer.to_be_bytes();
            buf.push(0b1100_0000 | bytes[0]);
            buf.push(bytes[1]);
        } else {
            buf.push(0);
        }
    }

    pub fn read(buf: &mut &[u8]) -> Self {
        let mut name = Self {
            parts: Vec::new(),
            pointer: None,
        };
        loop {
            if buf[0] == 0 {
                *buf = &buf[1..];
                break;
            } else if buf[0] & 0b1100_0000 == 0b1100_0000 {
                let index = u16::from_be_bytes([buf[0] & 0b0011_1111, buf[1]]);
                *buf = &buf[2..];
                name.pointer = Some(index);
                break;
            } else {
                let len = buf[0] as usize;
                name.parts.push(buf[1..1 + len].to_vec());
                *buf = &buf[1 + len..];
            }
        }

        name
    }
}
