// Copyright 2021 @skyekiwi authors & contributors
// SPDX-License-Identifier: GPL-3.0-or-later

import fs from 'fs';
import { fromByteArray, toByteArray } from 'base64-js';
import { u8aToString } from '@skyekiwi/util';

const { execute } = require('./execSync');

console.log('$ yarn vm', process.argv.slice(2).join(' '));

function runVM({
  methodName = "",
  stateInput = "{}",
  input = "",
  wasmFile = "./wasm/greeting.wasm",
  profiling = false
}) {
  // compile the runner
  execute('cd near-vm-runner-standalone && cargo build --release')

  const runnerPath = "./near-vm-runner-standalone/target/release/near-vm-runner-standalone";
  // execute the vm
  // execute('./near-vm-runner-standalone/target/release/near-vm-runner-standalone --wasm-file ./near-vm-runner-standalone/target/release/main.wasm --method-name get_greeting --input \'{"account_id": "bob"}\' --state \'{"YQNib2I=":"c2pkZmtsamFza2RsZmpranNrZGY="}\' --timings > result.json')
  execute(`${runnerPath} --wasm-file ${wasmFile} --method-name ${methodName} --input \'${input}\' --state \'${stateInput}\' ${profiling ? "--timings" : ""} > result.json`)
  // parse the output 
  const contentRaw = fs.readFileSync('result.json');
  const content = JSON.parse(contentRaw.toString());
  const stateB64 = JSON.parse(content.state);
  let state: {[key: string]: string} = {}
  
  for (const key in stateB64) {
    const k = u8aToString(toByteArray(key))
    const v = u8aToString(toByteArray(stateB64[key]))
    state[k] = v;
  }

  console.log("-------EXEC RESULT BEGINS-------");
  try {
    console.log("Outcome", u8aToString(Uint8Array.from(JSON.parse(content.outcome))));
  } catch(err) {
    // pass - in case of the outcome is 'None'
    // console.error(err)
  }

  console.log(state);
  
  console.log("------- EXEC RESULT ENDS -------");
  return stateB64;
}

const state = runVM({
  methodName: 'set_greeting',
  input: '{"message": "somethingelse"}',
  stateInput: '{}',
})

runVM({
  methodName: 'get_greeting',
  input: '{"account_id": "bob"}',
  stateInput: JSON.stringify(state),
})
