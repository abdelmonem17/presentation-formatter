<style>
    .present p {
        text-align: left;

    }
</style>

# Learn rust with NEAR

[fungable token](https://github.com/near/near-sdk-rs/tree/master/examples/fungible-token): Example implementation of a Fungible Token contract which uses [near-contract-standards](https://github.com/near/near-sdk-rs/tree/master/near-contract-standards) and [simulation](https://github.com/near/near-sdk-rs/tree/master/near-sdk-sim).

---

## Variables and constant

Rust has variables like any other language such as primative and non-primative and there are some special features we will disscus them one by one.

----

### [struct](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)

```rs=32
pub struct Contract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
}
```

Rust doen't have classes but instead struct to define a container for data like in these line of code we define a struct with two variables 'token' and 'metadata'.

----

### [enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)

```rs=40
enum StorageKey {
    FungibleToken,
    Metadata,
}
```

Rust have enums to give names to const variables in additions other featurs we will explore them in next lessons like in this code we define new type called 'StorageKey' with two possibilities 'FungibleToken' and 'Metadata'.

----

### [const](https://doc.rust-lang.org/std/keyword.const.html)

```rs=37
const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";
```

Rust has compile time const that recommended to be used with values that used many times in addition to it gives readablity for the code, for example here we define a const for image csv value that might be used many times in the program.

---

## functions

like any other language you can define some reusable code and it is called function.

----

### function definition and calling

```rs=109
 fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
     let mut builder = VMContextBuilder::new();
     builder
         .current_account_id(accounts(0))
         .signer_account_id(predecessor_account_id.clone())
         .predecessor_account_id(predecessor_account_id);
     builder
 }
```

here we are defining a function with name 'get_context', parameter and return type, also at lines 111th to 114 we call functions in a variable called 'builder' and they are [instance functions](https://turreta.com/2019/10/14/static-and-instance-methods-in-struct/) but we will learn them in the next section.

---

## impl section, and static and instance functions

Rust is not object oriented language but you can define structs as data container and attach function to operate on this data, also you can do the same for enums.

----

### [impl](https://doc.rust-lang.org/std/keyword.impl.html)

```rs=46
impl Contract {
```

here we start to write function to manipulate the dervied type

----

### [static function](https://turreta.com/2019/10/14/static-and-instance-methods-in-struct/)

```rs=69
 pub fn new(owner_id: AccountId, total_supply: U128, metadata: FungibleTokenMetadata) -> Self {
```

inside impl block we can define a static function like this so it can be called from the type itself, here we return [Self](https://stackoverflow.com/questions/32304595/whats-the-difference-between-self-and-self) from the function so here we refere to the current type.

----

### [static function](https://turreta.com/2019/10/14/static-and-instance-methods-in-struct/) usage

```rs=110
 let mut builder = VMContextBuilder::new();
```

here we call a static function 'new' in the type 'VMContextBuilder' and this is widly used method to create a new instance because Rust doesn't have constractors.

----

### [instance function](https://turreta.com/2019/10/14/static-and-instance-methods-in-struct/)

```rs=81
 fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
     log!("Closed @{} with {}", account_id, balance);
 }
```

inside impl block we can define instance function like this code define function that takes parameters in addation to '& mut [self](https://doc.rust-lang.org/std/keyword.self.html)', rust introduce new concept cold [mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html) so here we borrow mutable refrence for the current object so that the function can edit on it.

----

### instance function and immutable reference

```rs=95
 fn ft_metadata(&self) -> FungibleTokenMetadata {
     self.metadata.get().unwrap()
 }
```

here we define an intance function with '&self' this means the function [borrows](https://doc.rust-lang.org/rust-by-example/scope/borrow.html) the current object for readonly like in 96th line we use it to get data from 'metadata' variable.

----

### instance function calling

```rs=110
 let mut builder = VMContextBuilder::new();
 builder
     .current_account_id(accounts(0))
     .signer_account_id(predecessor_account_id.clone())
     .predecessor_account_id(predecessor_account_id);
```

line 112 to 114 calling instance function on the variable 'builder' and these lines follows [builder pattern](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html)

---

## [ownership](https://doc.rust-lang.org/rust-by-example/scope/move.html) and [borrowing](https://doc.rust-lang.org/rust-by-example/scope/borrow.html)

rust has unique concept called ownership and borrowing so when you assign variables to anther so most of the time instead of copying, the ownership taken by the second variable, so we can use borrowing and other things so keep the origin variable

----

### [Clone](https://doc.rust-lang.org/std/clone/trait.Clone.html) and ownership

```rs=113
 .signer_account_id(predecessor_account_id.clone())
 .predecessor_account_id(predecessor_account_id);
```

as we mention when we use a varable in passing to function or to assign anther variable so the ownership is transfered and the origin variable move and this exist with non-primative types unless they implement [Copy trait](https://doc.rust-lang.org/std/marker/trait.Copy.html), so in line 13 we use clone function to make a copy of data and pass it so that we could use the original variable in the next line to be passed to the function

---

## [testing in rust](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)

Tests are Rust functions that verify that the non-test code is functioning in the expected manner.

----

### unit test

```rs=118
 #[test]
 fn test_new() {
     let mut context = get_context(accounts(1));
     testing_env!(context.build());
     let contract = Contract::new_default_meta(accounts(1).into(), TOTAL_SUPPLY.into());
     testing_env!(context.is_view(true).build());
     assert_eq!(contract.ft_total_supply().0, TOTAL_SUPPLY);
     assert_eq!(contract.ft_balance_of(accounts(1)).0, TOTAL_SUPPLY);
 }
```

so here we writing a function and anntotate it as test like in line 118, and we use assertion funtions like at lines 124 and 125

---

## [modules](https://doc.rust-lang.org/rust-by-example/mod.html) and [imports](https://doc.rust-lang.org/reference/items/use-declarations.html)

Rust structure it's code in modules

----

### importing from other crates

```rs=18
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
```

these lines imports struct types from other crates that must be mentioned as a dependence in [Cargo.toml](https://doc.rust-lang.org/cargo/reference/manifest.html)

----

### modules

```rs=100
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
```

we can define new module in rust to structure our code, here we declaring a module at line 101 ,and at line 100 we annotate that it is a test module and use a [compile time switch](https://doc.rust-lang.org/reference/conditional-compilation.html) to compile this module only at non-WASM platform

