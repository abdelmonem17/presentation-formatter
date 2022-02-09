#Learn rust with NEAR
Smart Contracts are the back-end of your application that runs code and stores data on the blockchain. All smart contracts on NEAR must be compiled to WebAssemble or simply WASM. Currently, we support two languages AssembleScript and Rust with custom software development kits (SDKs) to assist in their creation but you can use any programming language and compile it to wasm. But here we will use Rust as it is a powerful language with a great developer experience.
#Your first contract

[status-message](https://github.com/near/near-sdk-rs/tree/master/examples/status-message):
records the status messages of the accounts that call this contract.
```rust=
pub fn set_status(&mut self, message: String) {
        let account_id = env::signer_account_id();
        log!("{} set_status with message {}", account_id, message);
        self.records.insert(account_id, message);
    }
```
This is a defination for [function](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) in rust, so here specifing a function name and signatures, set_status is a function with [String](https://doc.rust-lang.org/std/string/struct.String.html) type as input and no return type .
First line in the [function](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) here calls function and creates [variable](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html) finally assigning the result to this variable, creating a varible in rust like any other language except the [mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html).
```rust=
pub struct StatusMessage {
    records: HashMap<AccountId, String>,
}
```
Rust is like any other other language has primative types like bool,i32,u32, [more types](https://doc.rust-lang.org/book/ch03-02-data-types.html), in addition to [user defined types](https://doc.rust-lang.org/rust-by-example/custom_types.html) like [struct](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html).
these lines define structure with name 'StatusMessage' with records as a [hashmap](https://doc.rust-lang.org/std/collections/struct.HashMap.html) member variable, hashmap is a custom variable that is built in the standard library of rust.
```rust=
impl StatusMessage {
    #[payable]
    pub fn set_status(&mut self, message: String) {
        let account_id = env::signer_account_id();
        log!("{} set_status with message {}", account_id, message);
        self.records.insert(account_id, message);
    }

    pub fn get_status(&self, account_id: AccountId) -> Option::<String> {
        log!("get_status for account_id {}", account_id);
        self.records.get(&account_id).cloned()
    }
}
```
in rust we can attach functions to the defined struct using [impl](https://doc.rust-lang.org/rust-by-example/generics/impl.html) so the [function](https://doc.rust-lang.org/rus-by-example/fn/methods.html) might be called from the instance or from the type itself, to make instance function you must use [self](https://doc.rust-lang.org/std/keyword.self.html), &mut self or &self to refer to the current object.
Rust introduces new concept of [ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) and [borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) for example when you make the function take 'self' as a prameter so here you give the function the ownership for the current object instead of take a copy of it, but when you pass '&self' or '&mut self' here you make the function borrow the current object for lifetime of the function, as we mentioned before declarations of the variable determines the [mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html) of the variable  so '&self' borrows the current object immutable so that it can't be changed and in this case called view method in smart contract code but '&mut self' borrows the current object so that it can't be changed so it is called change in the smart contract code.
for example at line 6 we call 'insert' function in 'records' member of the current StatusMessage object so we understand that 'insert' function is member function for type of 'records', also we see that at this line we pass 'message' and 'account_id' to the function so here we give this function the ownership of these variable so you can't use them after this line but in other hand at line 11 we pass '&account_id' so it is called borrowing the variable to the function. 

anther user define type here is the [enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html),enum in rust is like in other languages which represents named values except it can hold data, at line 9 we return '[Option](https://doc.rust-lang.org/std/option/)<[String](https://doc.rust-lang.org/std/string/struct.String.html)>' from the function so the function might return 'None' and it is equivalent to 'null' or return 'Some<String>' that has data.
Rust introduces new concept of [ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) and [borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) for example when you make the function take 'self' as a prameter so here you give the function the ownership for the current object instead of take a copy of it, but when you pass '&self' or '&mut self' here you make the function borrow the current object for lifetime of the function, as we mentioned before declarations of the variable determines the [mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html) of the variable  so '&self' borrows the current object immutable so that it can't be changed and in this case called view method in smart contract code but '&mut self' borrows the current object so that it can't be changed so it is called change in the smart contract code.
for example at line 6 we call 'insert' function in 'records' member of the current StatusMessage object so we understand that 'insert' function is member function for type of 'records', also we see that at this line we pass 'message' and 'account_id' to the function so here we give this function the ownership of these variable so you can't use them after this line but in other hand at line 11 we pass '&account_id' so it is called borrowing the variable to the function. 
anther user define type here is the [enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html),enum in rust is like in other languages which represents named values except it can hold data, at line 9 we return '[Option](https://doc.rust-lang.org/std/option/)<[String](https://doc.rust-lang.org/std/string/struct.String.html)>' from the function so the function might return 'None' and it is equavelant to 'null' or return 'Some<String>' that has data.