const { Router } = require("express");
const { body, query, param } = require("express-validator");
const validate = require("../middleware/validate");
const { horizonServer, rpcServer, NETWORK, CONTRACT_IDS } = require("../config/stellar");
const {
  ASSET_TYPES,
  LICENSE_TYPES,
  indexAsset,
} = require("../services/assetService");
const {
  buildListAssetTx,
  submitSignedTx,
} = require("../services/listingService");

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

// ── Asset listing (Freighter-signed flow) ─────────────────────────────────────

// Shared validators mirroring the on-chain marketplace rules. `price` is in
// stroops and is kept as a string end-to-end to avoid precision loss.
const listingBodyRules = [
  body("owner").isString().trim().isLength({ min: 56, max: 56 }),
  body("name").isString().trim().isLength({ min: 1, max: 200 }),
  body("description").isString().trim().isLength({ min: 1, max: 2000 }),
  body("assetType").isIn(ASSET_TYPES),
  body("licenseType").isIn(LICENSE_TYPES),
  body("price").isInt({ min: 1 }), // stroops; must be > 0 (InvalidPrice)
];

/**
 * POST /api/v1/stellar/list-asset/build
 * Build & simulate an unsigned `list_asset` transaction for the frontend to
 * sign with Freighter. Contract rejections (e.g. InvalidPrice) surface here.
 */
router.post("/list-asset/build", listingBodyRules, validate, async (req, res, next) => {
  try {
    const { owner, name, description, assetType, licenseType, price } = req.body;
    const result = await buildListAssetTx({
      owner,
      name: name.trim(),
      description: description.trim(),
      assetType,
      licenseType,
      price: String(price),
    });
    res.json(result);
  } catch (err) {
    // Surface deliberately-thrown, client-safe errors (contract rejections,
    // unconfigured contract, RPC failures) with their message + optional code.
    if (err.status) {
      return res
        .status(err.status)
        .json({ error: err.message, ...(err.code && { code: err.code }) });
    }
    next(err);
  }
});

/**
 * POST /api/v1/stellar/list-asset/submit
 * Submit the Freighter-signed transaction, wait for confirmation, index the
 * new asset, and return its on-chain id so the client can redirect.
 */
router.post(
  "/list-asset/submit",
  [body("signedXdr").isString().isLength({ min: 1 }), ...listingBodyRules],
  validate,
  async (req, res, next) => {
    try {
      const { signedXdr, owner, name, description, assetType, licenseType, price } = req.body;
      const { hash, assetId } = await submitSignedTx(signedXdr);

      let asset = null;
      if (assetId != null) {
        asset = indexAsset({
          id: assetId,
          owner,
          name: name.trim(),
          description: description.trim(),
          assetType,
          licenseType,
          price: Number(price),
        });
      }

      res.status(201).json({ hash, id: assetId, asset });
    } catch (err) {
      if (err.status) {
        return res
          .status(err.status)
          .json({ error: err.message, ...(err.code && { code: err.code }) });
      }
      next(err);
    }
  }
);

module.exports = router;
