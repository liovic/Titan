import { AddressData, BlockTip, TitanHttpClient } from '@titanbtcio/sdk';

async function sendTransaction(txHex: string): Promise<string> {
  const client = new TitanHttpClient('http://localhost:3030');
  return await client.sendTransaction(txHex);
}

async function main() {
  const txHex =
    '020000000001037c748146487b08803574e0f4fc410a6e83bc817dac16d076bf5d8fbbd53819b40000000000ffffffff5286e1e460feafc321548124cf61bb9b0deaeacc33563c922922dd9a6e086dc20300000000ffffffff7c748146487b08803574e0f4fc410a6e83bc817dac16d076bf5d8fbbd53819b40100000000ffffffff062202000000000000225120b6c701270931c1ee7e43821a22203d8de435c8d3c1eb7b97e049a45ccdd7b75822020000000000002251202dc5e99ddad5c4824e84ee53c9ef5dbab00330ed491887ed6ad3a0c8d688ae292202000000000000225120bf313b85d91eb7eb673968dd76f11b7d4bad43f4b7965724f4f4587ddc191d72f33b0000000000002251202dc5e99ddad5c4824e84ee53c9ef5dbab00330ed491887ed6ad3a0c8d688ae290000000000000000106a5d0d160100ffad03238a94ffdf0102d07f880200000000225120bf313b85d91eb7eb673968dd76f11b7d4bad43f4b7965724f4f4587ddc191d720340952921eb511d6ef885c8d44f08808789b8baf69cb6934dfa3d7efbac7ed591138a28d931e5646aec4fda9b601095f059744d8e360c63f1ef517b42c123efa8a54520ab408c7989d903c5c09552e7e0d39f49b2e5339e9c4c8ff696b1797aecea151bac63202cba9a023a513873fd02efd93f922d11eb709efde37f8ec0c40abf46c20623ed6821c0ab408c7989d903c5c09552e7e0d39f49b2e5339e9c4c8ff696b1797aecea151b01410ef5cce7c4a07eb798468411e31c9a316d440b6493efe04fcd422cf9b163710b9d88023fdeee7a0ebed9b5614dfc65ba0d5f38c64713a7883f83f03e770309b981034003b3cbd99b2088332dfe0e7f24847ab9f18bde126c0d5edc8d6303976c2ddcefda3a92c021e895b702b91e2f9296a822750a02fe4f57eeee538fbb78e535ca524520ab408c7989d903c5c09552e7e0d39f49b2e5339e9c4c8ff696b1797aecea151bac6320f459378df0b7fee33978573c17811f581e78b2048cef120752d80673ffbfe2cd6821c1ab408c7989d903c5c09552e7e0d39f49b2e5339e9c4c8ff696b1797aecea151b00000000';
  try {
    const txid = await sendTransaction(txHex);
    console.log(`Transaction sent: ${txid}`);
  } catch (err) {
    console.error(typeof err);
    console.error(err);
  }
}

main();
