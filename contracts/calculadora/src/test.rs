#![cfg(test)]

use super::*;
use soroban_sdk::Env;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

   let resultado = client.sumar(&10, &20);
    
     assert_eq!(
       resultado,
       30
    ); 
    
    assert_eq!(
        client.resultado_anterior(),
        30
     ); 
}
