import { ProofData, BarretenbergVerifier as Verifier } from '@noir-lang/backend_barretenberg';
import * as fs from 'fs';

import { parseArgs } from "util";

const { values, positionals } = parseArgs({
  args: Bun.argv,
  options: {
    vKeyPath: {
      type: 'string',
    },
    proofPath: {
      type: 'string',
    },
  },
  strict: true,
  allowPositionals: true,
});

interface HyleOutput {
  version: number;
  initial_state: number[]; // TODO: en faire un vec<u8>
  next_state: number[]; // TODO: en faire un vec<u8>
  origin: string;
  caller: string;
  block_number: number;
  block_time: number;
  tx_hash: number[];
}


function parseString(vector: string[]): string {
  let length = parseInt(vector.shift() as string);
  let resp = "";
  for (var i = 0; i < length; i += 1)
    resp += String.fromCharCode(parseInt(vector.shift() as string, 16));
  return resp
}

function parseArray(length: number, vector: string[]): number[] {
  let resp: number[] = [];
  for (var i = 0; i < length; i += 1)
    resp.push(parseInt(vector.shift() as string, 16));
  return resp
}


function deserializePublicInputs<T>(publicInputs: string[]): HyleOutput {
  const version = parseInt(publicInputs.shift() as string);

  const statesLength = 4; // TODO: size fixed on 4 bytes. We might want to change it ?
  const tx_hash_length = 4; // TODO: size fixed on 4 bytes. We might want to change it ?

  const initial_state = parseArray(statesLength, publicInputs);
  const next_state = parseArray(statesLength, publicInputs);
  const origin = parseString(publicInputs);
  const caller = parseString(publicInputs);
  const block_number = parseInt(publicInputs.shift() as string);
  const block_time = parseInt(publicInputs.shift() as string);
  const tx_hash = parseArray(tx_hash_length, publicInputs);
  // We don't parse the rest, which correspond to programOutputs

  return {
      version,
      initial_state,
      next_state,
      origin,
      caller,
      block_number,
      block_time,
      tx_hash
  };
}

const proof = JSON.parse(fs.readFileSync(values.proofPath, { encoding: 'utf8' }));
const b64vKey = fs.readFileSync(values.vKeyPath, { encoding: 'utf8' });
const vKey = Uint8Array.from(Buffer.from(b64vKey, 'base64'));

const deserializedProofData: ProofData = {
  proof: Uint8Array.from(proof.proof),
  publicInputs: proof.publicInputs
};

// Verifying
const verifier = new Verifier();
const isValid = await verifier.verifyProof(deserializedProofData, vKey);

if (isValid){
  const hyleOutput = deserializePublicInputs(deserializedProofData.publicInputs);
  process.stdout.write(JSON.stringify(hyleOutput));
  process.exit(0);
}
else {
  process.exit(1);
}