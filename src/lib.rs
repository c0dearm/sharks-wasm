use wasm_bindgen::prelude::*;

use sharks::{Share, Sharks};

#[wasm_bindgen]
pub struct SharksJS(Sharks);

#[wasm_bindgen]
impl SharksJS {
    pub fn new(threshold: u8) -> SharksJS {
        SharksJS(Sharks(threshold))
    }

    pub fn deal(&self, secret: &[u8], n_shares: usize) -> JsValue {
        let shares: Vec<Vec<u8>> = self
            .0
            .dealer(secret)
            .take(n_shares)
            .map(|s| (&s).into())
            .collect();
        JsValue::from_serde(&shares).unwrap()
    }

    pub fn recover(&self, shares: JsValue) -> Result<Vec<u8>, JsValue> {
        let shares: Vec<Vec<u8>> = shares.into_serde().map_err(|e| e.to_string())?;
        let shares: Vec<Share> = shares.iter().map(|s| s.as_slice().into()).collect();

        self.0.recover(shares.as_slice()).map_err(JsValue::from)
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen::prelude::JsValue;
    use wasm_bindgen_test::*;

    use super::SharksJS;

    #[wasm_bindgen_test]
    fn test_recover_errors() {
        let sharks = SharksJS::new(3);
        let secret = sharks.recover(JsValue::from(1));
        assert!(secret.is_err());

        let shares = sharks.deal(&[1], 2);
        let secret = sharks.recover(shares);
        assert!(secret.is_err());
    }

    #[wasm_bindgen_test]
    fn test_integration_works() {
        let sharks = SharksJS::new(3);
        let shares = sharks.deal(&[1, 2, 3, 4], 255);
        let secret = sharks.recover(shares).unwrap();
        assert_eq!(secret, vec![1, 2, 3, 4]);
    }
}
