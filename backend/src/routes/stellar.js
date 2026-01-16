const { Router } = require("express");
const { query, param } = require("express-validator");
const validate = require("../middleware/validate");
const { horizonServer, rpcServer, NETWORK, CONTRACT_IDS } = require("../config/stellar");

const router = Router();

/**
 * GET /api/v1/stellar/account/:publicKey
 * Fetch account balances from Horizon.
 */
router.get(
  "/account/:publicKey",
  [param("publicKey").isString().isLength({ min: 56, max: 56 })],
  validate,
  async (req, res, next) => {
    try {
      const account = await horizonServer.loadAccount(req.params.publicKey);
      res.json({
        publicKey: account.account_id,
        sequence: account.sequence,
        balances: account.balances,
        subentryCount: account.subentry_count,
      });
    } catch (err) {
      if (err.response?.status === 404) {
        return res.status(404).json({ error: "Account not found on network" });
      }
      next(err);
    }
  }
);

/**
 * GET /api/v1/stellar/network
 * Return current network config and contract addresses.
 */
router.get("/network", (_req, res) => {
  res.json({
    network: NETWORK,
    contracts: CONTRACT_IDS,
    rpcUrl: process.env.STELLAR_RPC_URL,
    horizonUrl: process.env.STELLAR_HORIZON_URL,
  });
});

/**
 * GET /api/v1/stellar/fee
 * Fetch current recommended fee from Horizon.
 */
router.get("/fee", async (_req, res, next) => {
  try {
    const feeStats = await horizonServer.feeStats();
    res.json({
      baseFee: feeStats.last_ledger_base_fee,
      p50: feeStats.fee_charged.p50,
      p90: feeStats.fee_charged.p90,
      p99: feeStats.fee_charged.p99,
    });
  } catch (err) {
    next(err);
  }
});

module.exports = router;
