use ethers::{types::Bytes, utils::id};
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

    pub fn call(
        self,
        contract_address: &str,
        function_signature: &str,
        function_args: &Vec<Box<str>>,
    ) {
        // TODO
        // parse the function signature to extracts types.
        // check if the types are valid evm compatible types.
        // check if the number of types equal to amount of args sent by the user
        // formulate/pad the args according to the required function type.
        // extract the the first 4 bytes of the function signature.
        // formulate the data field of the TransactionObject with the above data.
        parse_function_signature(function_signature);
        let function_id = Bytes::from(id(function_signature).to_vec());
    }
}

fn parse_function_signature(function_signature: &str) {
    // TODO
    // check for the first "(" in the string and return
    // check for the last ")"in the string and return
    // split the remaining string with ","
    // parse the newString in the custom struct
    // return the custom struct

    let index = function_signature
        .find("(")
        .expect("FUNCTION_SIGNATURE_ERROR");

    let sub_string = &function_signature[index + 1..function_signature.len() - 1];
    extract_tuples(sub_string);
}

//extract tuples from the function args
fn extract_tuples(sub_string: &str) {
    let mut open_total = 0;
    let mut open_index = 0;
    let mut close_total = 0;
    let mut i = 0;
    for character in sub_string.chars() {
        if character == '(' {
            open_total += 1;
            if open_total == 1 {
                open_index = i + 1;
            }
        }

        if character == ')' {
            close_total += 1;
        }

        if open_total > 0 && close_total > 0 {
            if open_total == close_total {
                let tuple = &sub_string[open_index - 1..i + 1];

                open_total = 0;
                close_total = 0;

                println!("tuple {}", tuple);
            }
        }

        i += 1;
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
