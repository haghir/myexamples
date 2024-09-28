use std::collections::HashMap;

/// Symbols table that holds each symbol uniquely.
/// This table doesn't distinguish each symbol from others by where it appears.
/// Symbols are distinguished just by the texts.
/// A token that references a symbol has the index of the "symbol" field
/// instead of the String value.
pub struct SymTbl {
    symbols: Vec<String>,
    mapping: HashMap<String, usize>,
}

impl SymTbl {
    pub fn insert(&mut self, symbol: &String) -> usize {
        if self.mapping.contains_key(symbol) {
            self.mapping[symbol]
        } else {
            self.symbols.push(String::from(symbol));
            let i = self.symbols.len() - 1;
            self.mapping.insert(String::from(symbol), i);
            i
        }
    }

    pub fn get_index(&self, symbol: &String) -> Option<usize> {
        if self.mapping.contains_key(symbol) {
            Some(self.mapping[symbol])
        } else {
            None
        }
    }

    pub fn get_symbol(&self, index: usize) -> String {
        String::from(&self.symbols[index])
    }
}
