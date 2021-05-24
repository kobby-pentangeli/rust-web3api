![](https://github.com/Web3-API/branding/blob/master/logo/Web3API_On_Black_BG.jpg?raw=true)  

---

🚨 **Web3API is in Pre-Alpha** 🚨  

## **Web3's Universal Integration Standard**  
[Web3API](https://web3api.dev) makes integrating Web3 protocols into applications seamless, without sacrificing decentralization. Interact with any blockchain, from any programming language, in seconds.  

---

## TODO
```
[ ] Create a Rust WASM runtime for the Web3API standard
  - MsgPack: Serialization & Deseralization
  - Web3ApiClient Imports: `__w3_subinvoke`, `__w3_abort`, etc
  Reference: ./packages/wasm/as/

[ ] Defining the generated code for the Web3API Schema Bindings
  - Create test case that has the predefined output, used to test the templates against
  - Create Mustache string-templates for the necessary generated code
    - [ ] Object Types
    - [ ] Enums
    - [ ] Wrapped Query Methods
    - [ ] Imported: Queries, Objects, Enums
  Reference: ./packages/schema/bind/src/bindings/wasm-as/
  Create: ./packages/schema/bind/src/bindings/wasm-rs/
  Reference: ./packages/test-cases/cases/bind/sanity/output/wasm-as/
  Create: ./packages/test-cases/cases/bind/sanity/output/wasm-rs/
```