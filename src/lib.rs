use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
#[derive(BorshDeserialize, BorshSerialize, Debug)]
#[near_bindgen]
pub struct Product {
    name: String,
    cost: i16,
    quantity: i16,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
#[near_bindgen]
pub struct Sales {
    product_name: String,
    quantity: i16,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
#[near_bindgen]
pub struct Phamacy {
    sales: Vec<Sales>,
    products: Vec<Product>,
}

impl Default for Phamacy {
    fn default() -> Self {
        Phamacy {
            sales: vec![],    
            products: vec![],
        }
    }
}
#[near_bindgen]
impl Phamacy {
    pub fn add_drug(&mut self, name: String, cost: i16, quantity: i16) {
        let pr: Product = Product {
            name: name,
            cost: cost,
            quantity: quantity,
        };
        self.products.push(pr)
    }

    pub fn sale_drug(&mut self, name: String, quantity: i16) {
        self.products.iter_mut().for_each(|item| {
            if item.name == name {
                if quantity > item.quantity {
                    let f = format!(
                        "We cannot sell you this quantity {}  as we have {}",
                        quantity, item.quantity
                    );
                   // env.log_str(f.as_string());
                   let s_slice: &str = &f[..];
                        env::log_str(s_slice);
                } else {
                    item.quantity -= quantity
                }
            }
        });
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    // TESTS HERE
    #[test]
    fn add_phamacy() {
        let mut pham = Phamacy::default();
        pham.add_drug( "insulin".to_owned(),200,  10);
        assert_eq!(pham.products.len(), 1);
    }

    #[test]
    fn sell_drug() {
        let mut pham = Phamacy::default();
        pham.add_drug( "insulin".to_owned(),200,  10);
        pham.sale_drug("insulin".to_owned(), 5);
        // 

        for x in pham.products{
            if x.name == "insulin"{
                assert_eq!(x.quantity, 5);
                break;
            }
        }
    }
   
}

