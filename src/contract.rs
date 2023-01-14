use ethers::types::Bytes;

pub struct Contract {
    pub bytecode: Bytes,
}

impl Contract {
    pub fn new(mut self, bytecode: Bytes) ->  Self {
        self.bytecode = bytecode;
        self
    }

    pub fn get_dispatcher(&self) {
        
    }
}
