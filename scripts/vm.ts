// Copyright 2021 @skyekiwi authors & contributors
// SPDX-License-Identifier: GPL-3.0-or-later

import fs from 'fs';
import { fromByteArray, toByteArray } from 'base64-js';
import { u8aToString } from '@skyekiwi/util';

const { execute } = require('./execSync');

console.log('$ yarn vm', process.argv.slice(2).join(' '));

function compile() {
  // compile the runner
  execute('cd src/near-vm-runner-standalone && cargo build --release')
}

function runVM({
  methodName = "",
  stateInput = "{}",
  input = "",
  wasmFile = "./wasm/greeting.wasm",
  contextFile = "./context/system.json",
  profiling = false
}) {
  const runnerPath = "./src/near-vm-runner-standalone/target/release/near-vm-runner-standalone";
  execute(`${runnerPath} --context-file ${contextFile} --wasm-file ${wasmFile} --method-name ${methodName} --input \'${input}\' --state \'${stateInput}\' ${profiling ? "--timings" : ""} > result.json`)
  
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

  console.log()
  console.log("-------EXEC RESULT BEGINS-------");
  try {
    console.log("Return Value", u8aToString(Uint8Array.from(JSON.parse(content.outcome))));
  } catch(err) {
    // pass - in case of the outcome is 'None'
    // console.error(err)
  }

  console.log(state);
  console.log("------- EXEC RESULT ENDS -------");
  console.log()

  return stateB64;
}

compile()

let state = {}

state = runVM({
  contextFile: './context/system.json',
  methodName: 'set_greeting',
  input: '{"message": "system_hello"}',
  stateInput: '{}',
})

state = runVM({
  contextFile: './context/bob.json',
  methodName: 'set_greeting',
  input: '{"message": "bob_hello"}',
  stateInput: JSON.stringify(state),
})

state = runVM({
  contextFile: './context/zs.json',
  methodName: 'set_greeting',
  input: '{"message": "zs_hello"}',
  stateInput: JSON.stringify(state),
})

state = runVM({
  methodName: 'get_greeting',
  input: '{"account_id": "bob.sk"}',
  stateInput: JSON.stringify(state),
})
