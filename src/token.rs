use tokenizers::models::bpe::BPE;
use tokenizers::tokenizer::{EncodeInput, Result, Tokenizer};

pub fn tokenize<'a>(message: String) -> Result<Vec<u32>> {
    let tokenizer = Tokenizer::from_pretrained("bert-base-cased", None)?;

    let encoding = tokenizer.encode(message.to_owned(), false)?;

    Ok(encoding.get_ids().to_vec())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_token_load() {
        let token_result = tokenize("Hello world !".to_string());
        assert!(token_result.is_ok());
        let token = token_result.unwrap();
        assert_eq!(token, vec![8667, 1362, 106]);
    }
}
