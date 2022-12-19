# 📜 A Trait Contract

Collections (of NFTs) often are associated with a set/combination of traits pulled from a pool of traits. This soroban contract aims at allowing to store the initial pool of traits and then generating a random combination of trait-options from the remaining pool until all traits / options have been drawn from the pool.

## 🙏 Thank you!
Major thank yous to the stellar community (especially stellar-quest, stellar-developers) for all the support and help.

Thank you, [@vinamo](https://github.com/vinamogit) for submitting a [`rnd`-implementation](https://github.com/vinamogit/soroban-rand) to ***Sorobanathon: First Light 🔭***.

## TL;DR
If you want to cut right to the chase here's all you need to do. Let's build a collection of 15 characters with differences in eyes, hair and age:

*The following assumes you have the CLI and rust-pipeline all [set up](https://soroban.stellar.org/docs/getting-started/setup).*


#### Clone this repo:
```shell
git clone https://github.com/hanseartic/sorobanathon.git hanseartic_sorobanathon
cd hanseartic_sorobanathon
```

#### Build and deploy the contract to sandbox
```shell
cargo build --target wasm32-unknown-unknown --release
CID=$(soroban deploy --wasm target/wasm32-unknown-unknown/release/trait_contract.wasm)
```

#### Now let's initialize the contract for a collection of 15
```shell
soroban invoke --id $CID --fn init --arg 527573746c696e6773 --arg 15
```
<details><summary >... and define the options for <b>eyes</b>...</summary>

```shell
soroban invoke --id $CID --fn add_trait \
  --arg "eyes" \
  --arg 436f6c6f72206f66207468652065796573
soroban invoke --id $CID --fn add_option --arg "eyes" \
  --arg "black" \
  --arg '{"object":{"vec":[{"symbol":"Characters"},{"object":{"bytes":"626c61636b"}}]}}'
soroban invoke --id $CID --fn add_option --arg "eyes" \
  --arg "brown" \
  --arg '{"object":{"vec":[{"symbol":"Characters"},{"object":{"bytes":"62726f776e"}}]}}'
soroban invoke --id $CID --fn add_option \
  --arg "eyes" \
  --arg "blue" \
  --arg '{"object":{"vec":[{"symbol":"Characters"},{"object":{"bytes":"626c7565"}}]}}'
soroban invoke --id $CID --fn add_option \
  --arg "eyes" \
  --arg "green" \
  --arg '{"object":{"vec":[{"symbol":"Characters"},{"object":{"bytes":"677265656e"}}]}}'
soroban invoke --id $CID --fn add_option \
  --arg "eyes" \
  --arg "yellow" \
  --arg '{"object":{"vec":[{"symbol":"Characters"},{"object":{"bytes":"79656c6c6f77"}}]}}'
```
</details>
<details><summary >... <b>hair</b>...</summary>

```shell
soroban invoke --id $CID --fn add_trait \
  --arg "hair" \
  --arg 68616972636f6c6f72
soroban invoke --id $CID --fn add_option \
  --arg "hair" \
  --arg "black" \
  --arg '{"object":{"vec":[{"symbol":"Characters"},{"object":{"bytes":"626c61636b"}}]}}'
soroban invoke --id $CID --fn add_option \
  --arg "hair" \
  --arg "blonde" \
  --arg '{"object":{"vec":[{"symbol":"Characters"},{"object":{"bytes":"626c6f6e6465"}}]}}'
soroban invoke --id $CID --fn add_option \
  --arg "hair" \
  --arg "brown" \
  --arg '{"object":{"vec":[{"symbol":"Characters"},{"object":{"bytes":"62726f776e"}}]}}'
soroban invoke --id $CID --fn add_option \
  --arg "hair" \
  --arg "grey" \
  --arg '{"object":{"vec":[{"symbol":"Characters"},{"object":{"bytes":"67726579"}}]}}'
soroban invoke --id $CID --fn add_option \
  --arg "hair" \
  --arg "red" \
  --arg '{"object":{"vec":[{"symbol":"Characters"},{"object":{"bytes":"726564"}}]}}'
```
</details>
<details ><summary >... and <b>age</b>...</summary>

```shell
soroban invoke --id $CID --fn add_trait \
  --arg "age" \
  --arg 61676520696e207374657073206f662037207965617273
soroban invoke --id $CID --fn add_option \
  --arg "age" \
  --arg "0" \
  --arg '{"object":{"vec":[{"symbol":"Numeric"},{"u32":0}]}}'
soroban invoke --id $CID --fn add_option \
  --arg "age" \
  --arg "7" \
  --arg '{"object":{"vec":[{"symbol":"Numeric"},{"u32":7}]}}'
soroban invoke --id $CID --fn add_option \
  --arg "age" \
  --arg "14" \
  --arg '{"object":{"vec":[{"symbol":"Numeric"},{"u32":14}]}}'
soroban invoke --id $CID --fn add_option \
  --arg "age" \
  --arg "21" \
  --arg '{"object":{"vec":[{"symbol":"Numeric"},{"u32":21}]}}'
soroban invoke --id $CID --fn add_option \
  --arg "age" \
  --arg "28" \
  --arg '{"object":{"vec":[{"symbol":"Numeric"},{"u32":28}]}}'
soroban invoke --id $CID --fn add_option \
  --arg "age" \
  --arg "35" \
  --arg '{"object":{"vec":[{"symbol":"Numeric"},{"u32":35}]}}'
soroban invoke --id $CID --fn add_option \
  --arg "age" \
  --arg "42" \
  --arg '{"object":{"vec":[{"symbol":"Numeric"},{"u32":42}]}}'
soroban invoke --id $CID --fn add_option \
  --arg "age" \
  --arg "49" \
  --arg '{"object":{"vec":[{"symbol":"Numeric"},{"u32":49}]}}'
soroban invoke --id $CID --fn add_option \
  --arg "age" \
  --arg "56" \
  --arg '{"object":{"vec":[{"symbol":"Numeric"},{"u32":56}]}}'
soroban invoke --id $CID --fn add_option \
  --arg "age" \
  --arg "63" \
  --arg '{"object":{"vec":[{"symbol":"Numeric"},{"u32":63}]}}'
soroban invoke --id $CID --fn add_option \
  --arg "age" \
  --arg "70" \
  --arg '{"object":{"vec":[{"symbol":"Numeric"},{"u32":70}]}}'
soroban invoke --id $CID --fn add_option \
  --arg "age" \
  --arg "77" \
  --arg '{"object":{"vec":[{"symbol":"Numeric"},{"u32":77}]}}'
soroban invoke --id $CID --fn add_option \
  --arg "age" \
  --arg "84" \
  --arg '{"object":{"vec":[{"symbol":"Numeric"},{"u32":84}]}}'
soroban invoke --id $CID --fn add_option \
  --arg "age" \
  --arg "91" \
  --arg '{"object":{"vec":[{"symbol":"Numeric"},{"u32":91}]}}'
soroban invoke --id $CID --fn add_option \
  --arg "age" \
  --arg "98" \
  --arg '{"object":{"vec":[{"symbol":"Numeric"},{"u32":98}]}}'
```
</details>

```shell
soroban invoke --id $CID --fn finalize
```

Now let's draw trait-sets for 15 different IDs. For the IDs we use the `sha256` of and asset-identifier (e.g. `RUSTLING:GCJL24NRWVOEC6H3FTBGY4HWOBVISRMPKSE76TBUUMD7542Y7VTYQO3F`).

<details ><summary >drawing the trait-sets</summary>

```shell
soroban invoke --id $CID --fn draw \
  --arg a727cb9392ed7adf9fe2c956acd61fc10b48424f238eda51631a6aae38a2f94c
soroban invoke --id $CID --fn draw \
  --arg c1d9646e7c8d0f914b3ad68f250f5366d9e136f888009e3c8858ec2719a5af22
soroban invoke --id $CID --fn draw \
  --arg 9a89368346742d4ccf159a48c66171cbc325971efd063a1a98ec49171e076b0a
soroban invoke --id $CID --fn draw \
  --arg 5b316880d7ea1aec888b49761454e8e8be256546b39678a6c4eb0eeb4b452216
soroban invoke --id $CID --fn draw \
  --arg 27dd8e017c9309ce8e2263a051668362d4aaf439c3b4f54cb9d3043fbaeb3d76
soroban invoke --id $CID --fn draw \
  --arg 89e92f382bc8591a8815128e1e7506befb40e44f6cc63aefa27a1433270588d3
soroban invoke --id $CID --fn draw \
  --arg 3862bb62e44058a1c2611780a884db257158a9688933914a5e9d122a082d3952
soroban invoke --id $CID --fn draw \
  --arg b10601951d41394712ed8727ab0bf5f54ab976b55fd169b9eb9b9d13c0295f5f
soroban invoke --id $CID --fn draw \
  --arg 06b5495fa146d37954324f75962c2631ada9f2d755d881a1b4fb198147196631
soroban invoke --id $CID --fn draw \
  --arg 77e8f7d7ef4a4c1d282304f226eb66ec42ce36f2df6a75a6a6b291785a0bb9c6
soroban invoke --id $CID --fn draw \
  --arg 8e2cd5e0583a983da73cc5a8d504bb6e2db9556af59188ed922026fbca6c17ea
soroban invoke --id $CID --fn draw \
  --arg bbb1da2a0aabd9e69706c23b1271dd297bf458a3cc05ee94519fa4f38c5dad4c
soroban invoke --id $CID --fn draw \
  --arg e63b11879d982f398bc4e3d00f0b4f91fe8a5f95e9d4e3618dab286dd0c98b9a
soroban invoke --id $CID --fn draw \
  --arg 0a475835f503b5f4f8131d87f0e45514e5b6d95d6346e12ecf9d475202fbe96f
soroban invoke --id $CID --fn draw \
  --arg 1a647b0e503c9e4579715c85c3e8797da92fef9693f35c4fb2fa203b409f60fe
```
</details>
You should get back a response like

```shell
{"age":["Numeric",98],"eyes":["Characters",[121,101,108,108,111,119]],"hair":["Characters",[98,114,111,119,110]]}
```
which (in this case) translates to

```json
{
    "age": 98,
    "eyes": "yellow",
    "hair": "brown"
}
```

Invoking the contract one more time with a new ID errors out (as expected):
```
error: HostError
Value: Status(ContractError(8))

Debug events (newest first):
   0: "VM trapped with host error"
   1: "escalating error 'Status(ContractError(8))' to VM trap"
   2: "failing with contract error status code 'Status(ContractError(8))'"
```

## The implementation
This implementation follows a naive builder-approch as to split up invoking the individual parts of the contract. Also as the actual distribution of the options is randomized and not defined by the input this seemed to be a good approach.

Discussions in dev-discord showed that this *may* not desired and it could/should be improved to a init-once invoke of the contract.

The contract will be initialized with a name and a collection-size. The name is not used right now but may be later to allow the same contract to be allowed to manage multiple collections (as of now for each collection a new deployment of the contract needs to be done), a set of traits (e.g. color, strength, accessoire), each trait with a set of options.

### Constraints
* There must be **at least one trait** in a *collection*.
* Each trait must have **at least one** *option*.
* The amount of options per trait can **not** exceed the *collection size*.

These constraints ensure that each option will be assigned to a trait-set when all combinations have been drawn.

### Future Improvements
* The contract should be intialized in a single step instead of with a builder-pattern
* The contract should be able to manage multiple sets of collections - currently a new deployment is needed for a new collection
* The contract *could* implement the [token interface](https://soroban.stellar.org/docs/common-interfaces/token)
