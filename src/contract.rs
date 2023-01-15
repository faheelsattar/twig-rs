use ethers::types::Bytes;
pub struct Contract {
    pub bytecode: Bytes,
}

impl Contract {
    pub fn new(mut self, bytecode: Bytes) -> Self {
        self.bytecode = bytecode;
        self
    }

    pub fn extract_dispatcher(self, pattern: Vec<u8>) {
        search(pattern, self.bytecode);
    }
}

// Todo: prolly move to some helper file
/// helper function to search function signatures from the bytecode
fn search(pattern: Vec<u8>, bytecode: Bytes) {
    let m = pattern.iter().len();
    let n = bytecode.iter().len();

    let mut i = 0;
    let mut j = 0;

    let mut signature: Bytes = Bytes::from(bytecode[0 as usize..1 as usize].to_vec());

    let lps: Vec<i32> = compute_lps_array(&pattern, m);

    loop {
        if n - i < m - j {
            break;
        }

        if pattern[j] == bytecode[i] {
            if pattern[j] == 0x63 {
                let data_size = (0x63 - 0x60) + 1;
                signature = Bytes::from(bytecode[i + 1 as usize..i + data_size + 1].to_vec());
                i = i + data_size + 1
            } else if pattern[j] == 0x61 {
                let data_size = (0x61 - 0x60) + 1;
                i = i + data_size + 1;
            } else {
                i += 1;
            }
            j += 1;
        }

        if j == m {
            println!("Signature {}", signature);
            j = lps[j - 1] as usize;
        } else if i < n && pattern[j] != bytecode[i] {
            if j != 0 {
                j = lps[j - 1] as usize;
            } else {
                i += 1;
            }
        }
    }
}

fn compute_lps_array(pattern: &Vec<u8>, m: usize) -> Vec<i32> {
    let mut len = 0;
    let mut i = 1;
    let mut lps: Vec<i32> = vec![0];

    loop {
        if i >= m {
            return lps;
        }
        if pattern[i] == pattern[len] {
            len += 1;
            lps.push(len as i32);
            i += 1;
        } else {
            if len != 0 {
                len = lps[len - 1 as usize] as usize;
            } else {
                lps.push(len as i32);
                i += 1;
            }
        }
    }
}
