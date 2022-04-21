#![no_std]

elrond_wasm::imports!();

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[elrond_wasm::derive::contract]
pub trait Adder {
    #[view(getSum)]
    #[storage_mapper("sum")]
    fn sum(&self) -> SingleValueMapper<BigUint>;

    #[init]
    fn init(&self, initial_value: BigUint) {
        self.sum().set(initial_value);
    }

    /// Add desired amount to the storage variable.
    #[endpoint]
    fn add(&self, value: BigUint) {
        self.sum().update(|sum| *sum += value);
    }

    // As an owner, claim Smart Contract balance - temporary solution for royalities, the SC has to be payable to be able to get royalties
    #[only_owner]
    #[endpoint(claimScFunds)]
    fn claim_sc_funds(&self) {

        let balance_egld = self.blockchain().get_sc_balance(&TokenIdentifier::egld(), 0);

        let payment_team = balance_egld*(BigUint::from(85_u32)/BigUint::from(100_u32));

        let payment_dev = balance_egld-payment_team;
        
        self.send().direct_egld(&self.dev_address().get(), &payment_dev, &[]);
        self.send().direct_egld(&self.blockchain().get_caller(), &payment_team,
            &[],
        );
    }

    #[storage_mapper("devAddress")]
    fn dev_address(&self) -> SingleValueMapper<ManagedAddress>;

}
