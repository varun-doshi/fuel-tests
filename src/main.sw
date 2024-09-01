// contract;

// abi MyContract {
//     fn test_function() -> bool;
//     fn test_number() -> u64;
// }

// impl MyContract for Contract {
//     fn test_function() -> bool {
//         true
//     }
//     fn test_number() -> u64{
//         let num:u64=64;
//         num
//     }
// }


// #[test]
// fn test_meaning_of_life() {
//     assert(6 * 7 == 42);
// }

// #[test]
// fn test_function_test_number(){
//    let caller = abi(MyContract, CONTRACT_ID);
//     let result = caller.test_number {}();
//     assert(result == 64)
// }

contract;
 
abi TestContract {
    #[storage(write)]
    fn initialize_counter(value: u64) -> u64;
 
    #[storage(read, write)]
    fn increment_counter(amount: u64) -> u64;

    #[storage(read)]
    fn get_counter() -> u64;
}
 
storage {
    counter: u64 = 0,
}
 
impl TestContract for Contract {
    #[storage(write)]
    fn initialize_counter(value: u64) -> u64 {
        storage.counter.write(value);
        value
    }
 
    #[storage(read, write)]
    fn increment_counter(amount: u64) -> u64 {
        let incremented = storage.counter.read() + amount;
        storage.counter.write(incremented);
        incremented
    }

    #[storage(read)]
    fn get_counter() -> u64{
        storage.counter.read()
    }
}
 