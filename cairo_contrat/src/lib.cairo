#[starknet::interface]
trait MyContractInterface<T> {
    fn name_get(self: @T) -> felt252;
    fn name_set(ref self: T, name: felt252);
    // fn my_is_oasis_found(self: @T) -> bool;
    fn my_find_oasis(ref self: T);
}

#[starknet::interface]
trait IDesert<TContractState> {
    fn is_oasis_found(self: @TContractState) -> bool;
    fn find_oasis(ref self: TContractState);
}

#[starknet::contract]
mod my_contract {
    use starknet::ContractAddress;
    use core::fmt::Display;

    use super::IDesert;
    use super::IDesertDispatcherTrait;
    use super::IDesertDispatcher;

    #[storage]
    struct Storage {
        name: felt252,
        desertContractAddress: ContractAddress,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        NameChanged: NameChanged,
    }

    #[derive(Drop, starknet::Event)]
    struct NameChanged {
        previous: felt252,
        current: felt252,
    }

    #[constructor]
    fn constructor(ref self: ContractState, name: felt252, desertContractAddress: ContractAddress) {
        self.name.write(name);
        self.desertContractAddress.write(desertContractAddress);
    }


    #[abi(embed_v0)]
    impl MyContract of super::MyContractInterface<ContractState> {
        fn name_get(self: @ContractState) -> felt252 {
            self.name.read()
        }

        fn name_set(ref self: ContractState, name: felt252) {
            let previous = self.name.read();
            self.name.write(name);
            self.emit(NameChanged { previous, current: name });
        }

        // fn my_is_oasis_found(self: @ContractState) -> bool {
        //     let desert_address = self.desertContractAddress.read();
        //     let desert_dispatcher = IDesertDispatcher { contract_address: desert_address };
        //     desert_dispatcher.is_oasis_found()
        // }

        fn my_find_oasis(ref self: ContractState) {
            // let desert_address = self.desertContractAddress.read();
            // let desert_address_str = 
            // println!("Desert address: {}", desert_address);
            IDesertDispatcher { contract_address: self.desertContractAddress.read() }.find_oasis()
        // desert_dispatcher.find_oasis();
        }
    }
}

