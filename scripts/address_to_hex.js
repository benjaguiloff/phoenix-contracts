const SorobanClient = require("soroban-client");
let contractId = process.argv[2] || undefined;
if (contractId) {
  console.log(new SorobanClient.Contract(contractId).contractId("hex"));
}
