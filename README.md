## Port of NEAR VM

This VM is intended to be the secret runtime VM for the SkyeKiwi Network. Ported from the [NEAR Protocol](https://github.com/near/nearcore)

## Usage

Run `yarn vm`

It should outputs something like: 
```
$ yarn vm 
$ cd near-vm-runner-standalone && cargo build --release
    Finished release [optimized] target(s) in 0.19s
$ ./near-vm-runner-standalone/target/release/near-vm-runner-standalone --wasm-file ./wasm/greeting.wasm --method-name set_greeting --input '{"message": "somethingelse"}' --state '{}'  > result.json
-------EXEC RESULT BEGINS-------
{
  STATE: '\x01\x00\x00\x00a',
  'a\x03\x00\x00\x00bob': '\r\x00\x00\x00somethingelse'
}
------- EXEC RESULT ENDS -------
$ cd near-vm-runner-standalone && cargo build --release
    Finished release [optimized] target(s) in 0.18s
$ ./near-vm-runner-standalone/target/release/near-vm-runner-standalone --wasm-file ./wasm/greeting.wasm --method-name get_greeting --input '{"account_id": "bob"}' --state '{"U1RBVEU=":"AQAAAGE=","YQMAAABib2I=":"DQAAAHNvbWV0aGluZ2Vsc2U="}'  > result.json
-------EXEC RESULT BEGINS-------
Outcome "somethingelse"
{
  STATE: '\x01\x00\x00\x00a',
  'a\x03\x00\x00\x00bob': '\r\x00\x00\x00somethingelse'
}
------- EXEC RESULT ENDS -------
```


## License

The entire code within this repository is licensed under the [GPLv3](LICENSE).

Please [contact us](https://skye.kiwi) if you have questions about
the licensing of our products.
