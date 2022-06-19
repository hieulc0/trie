pub struct Trie {
    children: Vec<Option<Trie>>,
    terminal: bool,
}

const NUM_CHARS: usize = 256;

impl Trie {
    pub fn new() -> Self {
        Self {
            children: (0..NUM_CHARS).map(|_| None).collect(),
            terminal: false,
        }
    }

    pub fn insert(&mut self, string: String) -> bool {
        let bytes = string.as_bytes();

        let mut tmp = self;
        let length = bytes.len();
        let mut i = 0;

        while i < length {

            if tmp.children[bytes[i] as usize].is_none() {
                tmp.children[bytes[i] as usize] = Some(Trie::new());
            }
            tmp = if let Some(child) = &mut tmp.children[bytes[i] as usize] {
                child
            }
            else {
                panic!("can not asign to child");
            };
            i += 1;

            // match tmp.children[bytes[i] as usize] {
            //     Some(ref mut child) => {
            //         tmp = child;
            //         i += 1;
            //     }
            //     None => {
            //         new_node = Some(Trie::new());
            //     }
            // }
            // tmp.children[bytes[i] as usize] = new_node;
        }
        
        if tmp.terminal {
            return false;
        }
        tmp.terminal = true;
        return true;
    }

    fn print_recursive(&self, buf: &mut Vec<u8>) {
        if self.terminal {
            println!("word: {}", String::from_utf8_lossy(buf));
        }
        for i in 0..self.children.len() {
            if !self.children[i].is_none() {
                buf.push(i as u8);
                self.children[i].as_ref().unwrap().print_recursive(buf);
                buf.pop();
            }
        }
    }

    pub fn print(&self) {
        let mut buf: Vec<u8> = Vec::new();

        self.print_recursive(&mut buf);
    }
}
