use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
//allows hashing of the serialized and deserialized elements
use near_sdk::collections::{LookupMap, Vector};
//A non-iterable implementation of a map that stores its content directly on the trie(digital tree to obtain specific keys within a set)
use near_sdk::Promise;
//the structure represents a result of the scheduled execution on another contract and in this case allows transactions with multiple actions
use near_sdk::serde::{Serialize, Deserialize};
//the framework serde is used for serializing(converting the data into a byte stream) and deserializing(vice versa from byte stream to the initia; state) Rust data structures efficiently and generically.
use near_sdk::{env, near_bindgen, AccountId};
//imported to builds a full smart contract

fn _one_near() -> u128 {
    u128::from_str_radix("1000000000000000000000000", 10).unwrap()
    //Function to assign the value of a near coin to a u128 bit value
    //the .unwrap gives the result of the computation, and if there was an error, panic and stop the program(handles errors quickly)
}
//the near_bindgen macro on the struct and function implementations automatically generates the neessary code to be valid NEAR contract to allow external calling of the functions
#[near_bindgen]
//the derive attribute is a macro that allows new items to be automatically generated for data structures and for a suitable debugging.
#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize,)]
//#[serde(crate = "near_sdk::serde")] points the serializing/deserializing frameowrk to the correct crate9compilation unit)
#[serde(crate = "near_sdk::serde")]
//the public struct product below takes the name of the product/drug and assign it to a string and assigns the cost and quantities to u128 and i16 respectively
pub struct Product {
    name: String,
    cost: u128,
    quantity: i16,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Serialize, Deserialize,)]
// the additional clone macro is a procedural macro, which means that it takes a stream of tokens and returns a stream of tokens.
#[serde(crate = "near_sdk::serde")]
pub struct Sales {
    product_name: String,
    quantity: i16,
    //the public struct for sales defines the product name as a string and assigns an i16 value to the quantity.
    //the struct is a data type that contains fields which define its particular instance. It helps to implement abstract ideas in a more understandable fashion
}

#[derive(BorshDeserialize, BorshSerialize)]
#[near_bindgen]
pub struct Phamacy {
    sales_history: LookupMap<AccountId, Vec<Sales>>,
 //the LookupMap is a non-iterable implementation of a map that stores its content directly on the trie
    products: Vector<Product>,
    //the product vector in this case is a container that stores the product string
}

impl Default for Phamacy {
    // implements the default function
    fn default() -> Self {
        // the trait gives the type a useful default value as our case has a set of options.
        Phamacy {
            sales_history: LookupMap::new(b"r".to_vec()),
            //we use the LookupMap to_vec to collect in the usual case where we want these containers.
            products: Vector::new(b"r".to_vec()),
        }
    }
}

#[near_bindgen]
impl Phamacy {

    pub fn get_products(&self)-> Vec<Product>{
        //the function gets the product from the default self and stores it in the Vec container
        self.products.to_vec()
    }

    pub fn get_sale_history(&self)-> Option<Vec<Sales>>{
        // the space optimized version stores the discriminants separately for the sales history in the vec container.
        self.sales_history.get(&env::current_account_id())
        // the env module is imported to allow the inspection and manipulation of the process's environment, which is the current_account_id.
        //the current_account_id is an instance of the buyer getting the specific product. A buyer has an account created at the pharmacy.
    }


    pub fn add_drug(&mut self, name: String, cost: u128, quantity: i16) {
        //the public funcion add_drug adds the mut keyword from the default self to push values as we have the name as a string, the cost and quantity as u128 and i16 respectively
        let pr: Product = Product {
            name: name,
            cost: cost,
            quantity: quantity,
        };
        self.products.push(&pr)
        //the function above pushes the product to the keyword pr
    }

    #[payable]
    // we are allowing a payable method for a token transfer with the function call for the contracts to define a fee in the token needed to be paid.
    // the program will panic supposing there is a token transfer during the invocation. Hence, this is done for safety reasons. 
    pub fn sale_drug(&mut self, name: String, quantity: u128) {
        let mut index_drug: Option<u64> = None;
        
        for (index, drug) in self.products.iter().enumerate() {
            if drug.name == name {
                if drug.quantity < quantity as i16 {
                    let f = format!(
                        "We cannot sell you this quantity {}  as we have {}",
                        quantity, drug.quantity
                    );
                    env::log_str(f.as_str())
                } else {
                    let total_cost = drug.cost * quantity;

                    if total_cost > env::attached_deposit() {
                        env::panic_str("You have insufficient funds ")
                    } else {
                        index_drug = Some(index as u64);
                    }
                }
            }
        }
        match index_drug {
            Some(idx) => {
                let mut product = self.products.get(idx).unwrap();
                product.quantity -= quantity as i16;

                let total_cost = product.cost * quantity;

                self.products.replace(idx, &product);
                Promise::new(env::current_account_id()).transfer(total_cost as u128);

                let user_history = self.sales_history.get(&env::current_account_id());

                let sale = Sales{
                    product_name : name.clone(),
                    quantity:quantity as i16
                };
                

                match user_history {
                    Some(mut history) => {

                        history.push(sale);

                        self.sales_history.insert(&env::current_account_id(), &history);

                    }
                    None => {
                        let  mut drugs:Vec<Sales> =  vec![];
                        
                        drugs.push(Sales{
                            product_name : "sample".to_string(),
                            quantity:122
                        });

                        self.sales_history.insert(&env::current_account_id(), &drugs);
                    }
                }
            }
            None => {}
        }
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
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn add_phamacy() {
        let mut pham = Phamacy::default();
        pham.add_drug("insulin".to_owned(), 200, 10);
        assert_eq!(pham.products.len(), 1);
    }

    #[test]
    fn sell_drug() {
        let user = AccountId::new_unchecked("erastus.testnet".to_string());
        let mut _context = get_context(user.clone());
        let bal = _one_near() * 2;
        _context.attached_deposit(bal);
        _context.account_balance(bal);

        testing_env!(_context.build());

        let mut pham = Phamacy::default();
        pham.add_drug("insulin".to_owned(), _one_near(), 10);
        pham.sale_drug("insulin".to_owned(), 1);

        for x in pham.products.iter() {
            if x.name == "insulin" {
                assert_eq!(x.quantity, 9);
                break;
            }
        }
    }
}