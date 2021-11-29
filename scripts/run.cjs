// Copyright 2021 @skyekiwi authors & contributors
// SPDX-License-Identifier: GPL-3.0-or-later

const execSync = require('./execSync.cjs');
const fs = require('fs');
console.log('$ yarn vm', process.argv.slice(2).join(' '));

function runVM() {
  // compile the runner
  execSync('cd near-vm-runner-standalone && cargo build --release')

  // execute the vm
  execSync('./near-vm-runner-standalone/target/release/near-vm-runner-standalone --wasm-file ./near-vm-runner-standalone/target/release/main.wasm --method-name get_greeting --input \'{"account_id": "bob"}\' --state \'{"YQNib2I=":"c2pkZmtsamFza2RsZmpranNrZGY="}\' --timings > result.json')

  // parse the output 
  const contentRaw = fs.readFileSync('result.json');
  const content = JSON.parse(contentRaw);
  // const state = JSON.parse(content.state);
  console.log(content);
}

runVM()
