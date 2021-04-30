const {
  LCDClient,
  MnemonicKey,
  MsgExecuteContract,
  StdFee,
} = require("@terra-money/terra.js");
const axios = require("axios");

// terra constants
const stdContractAddress = "terra1hurg8ze4tkdy00ppuy7feuse0y2uh0mc9vuwl9";
const mnemonic = process.env.COSMWASM_DOC_SEED;
const GAS = 200_000;

// band constants
const bandUrl = "https://asia-rpc.bandchain.org/oracle/request_prices";
const symbols = ["BTC", "ETH", "BAND"];

// connect to tequila testnet
const terra = new LCDClient({
  URL: "https://tequila-lcd.terra.dev",
  chainID: "tequila-0004",
});

// create wallet obj
const wallet = terra.wallet(new MnemonicKey({ mnemonic }));

const sleep = async (ms) => new Promise((r) => setTimeout(r, ms));

const getPricesFromBand = async () => {
  try {
    const {
      data: { result },
    } = await axios.post(bandUrl, {
      symbols,
      min_count: 10,
      ask_count: 16,
    });
    return {
      symbols,
      rates: result.map((e) => e["px"] + ""),
      resolve_times: result.map((e) => Number(e["resolve_time"])),
      request_ids: result.map((e) => Number(e["request_id"])),
    };
  } catch (e) {
    console.log(e);
    return null;
  }
};

const validateTx = async (txhash) => {
  let max_retry = 30;
  while (max_retry > 0) {
    await sleep(1000);
    max_retry--;
    try {
      process.stdout.clearLine();
      process.stdout.cursorTo(0);
      process.stdout.write("polling: " + (30 - max_retry));
      const txInfo = await terra.tx.txInfo(txhash);
      return txInfo;
    } catch (err) {
      if (err.isAxiosError && err.response && err.response.status !== 404) {
        console.error(err.response.data);
      } else if (!err.isAxiosError) {
        console.error(err.message);
      }
    }
  }
  return null;
};

const getCurrentRateFromStdContract = async () => {
  try {
    const result = await terra.wasm.contractQuery(stdContractAddress, {
      get_reference_data_bulk: {
        base_symbols: symbols,
        quote_symbols: Array(symbols.length).fill("USD"),
      },
    });
    return result;
  } catch (e) {
    console.log("Fail to get rate from std contract");
    console.log(e);
  }
  return null;
};

(async () => {
  while (true) {
    try {
      // get prices from band
      const relay = await getPricesFromBand();
      if (relay) {
        console.log("\nrelay message: ", JSON.stringify({ relay }));
      } else {
        throw "Fail to get stock price from band";
      }

      // create msg
      const execute = new MsgExecuteContract(
        wallet.key.accAddress,
        stdContractAddress,
        { relay }
      );
      // sign tx
      const signedTx = await wallet.createAndSignTx({
        msgs: [execute],
        fee: new StdFee(GAS, { uluna: Math.ceil(GAS * 0.15) }),
      });

      // broadcast tx
      const { txhash } = await terra.tx.broadcastSync(signedTx);
      console.log("broadcast tx: ", txhash);

      // wait for tx result
      const txResult = await validateTx(txhash);
      console.log("\n");
      if (!txResult) {
        throw "Fail to get result from chain";
      }

      if (!txResult.code) {
        console.log("tx successfully send!");
      } else {
        throw "Fail to send tx with result: " + JSON.stringify(txResult);
      }

      // get rates from std_reference_basic
      const currentRates = await getCurrentRateFromStdContract();
      if (currentRates) {
        console.log("current rates: ", JSON.stringify(currentRates));
      } else {
        throw "Fail to get current rates from std contract";
      }
    } catch (e) {
      console.log(e);
    }
    console.log(
      "=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-="
    );
    await sleep(10_000);
  }
})();
