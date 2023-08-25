import { expose } from "comlink";

async function generate_params() {
  const multiThread = await import("nova_scotia_browser_ecdsa");
  await multiThread.default();
  await multiThread.initThreadPool(navigator.hardwareConcurrency);

  return await multiThread.generate_params();
}

async function generate_proof(pp: string, sigs: string) {
  const multiThread = await import("nova_scotia_browser_ecdsa");
  await multiThread.default();
  await multiThread.initThreadPool(navigator.hardwareConcurrency);
  
  return await multiThread.generate_proof(pp, sigs);
}

async function verify_proof(pp: string, proof: string, sigs: string) {
  const multiThread = await import("nova_scotia_browser_ecdsa");
  await multiThread.default();
  await multiThread.initThreadPool(navigator.hardwareConcurrency);

  return await multiThread.verify_compressed_proof(pp, proof, sigs);
}

const exports = {
  generate_params,
  generate_proof,
  verify_proof,
};
export type NovaScotiaWorkerECDSA = typeof exports;

expose(exports);
