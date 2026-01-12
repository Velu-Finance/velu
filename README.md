<p align="center">
  <img src="https://readme-typing-svg.herokuapp.com?color=%23FF6B6B&lines=Velu:+Modern+FinTech+Stack" alt="Typing SVG" />
</p>

<p align="center">
  <img src="https://img.shields.io/github/actions/workflow/status/Velu-Finance/velu/ci.yml" alt="CI">
</p>

<div align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=orange" alt="Rust">
  <img src="https://img.shields.io/badge/Svelte-5A67D8?style=flat-square&logo=svelte&logoColor=white" alt="Svelte">
  <img src="https://img.shields.io/badge/Docker-2496ED?style=flat-square&logo=docker&logoColor=white" alt="Docker">
  <img src="https://img.shields.io/badge/TypeScript-007ACC?style=flat-square&logo=typescript&logoColor=white" alt="TypeScript">
</div>

# Velu — Payment Control & Fraud Prevention

## Overview

Velu is a SaaS tool for companies that pay many suppliers every month using payment files (CSV or ISO 20022 pain.001). Instead of sending these files directly to the bank, the company uploads them to Velu first. Velu analyzes each payment, compares it with a history of trusted suppliers and bank accounts, and splits the file into “safe” payments and “needs review” payments.

The goal is simple: reduce the risk of sending money to fraudsters when bank account details are changed by mistake or due to email/payment fraud.

## Key Features

- Upload payment files (CSV and pain.001) with hundreds or thousands of payments.
- Automatically detect:
  - New suppliers.
  - Changes in bank accounts for existing suppliers.
  - Unusually large payments.
- Maintain a **beneficiary ledger**:
  - History of suppliers and their trusted bank accounts.
  - Track when each account was first and last used.
- Exception workflow:
  - Create cases for risky payments.
  - Assign cases to approvers.
  - Approve once, approve and trust a new account, or reject the payment.
- Full audit log:
  - Track who approved or rejected what, and when.
- Multi-user and roles:
  - Accountants upload and prepare payments.
  - Approvers/finance managers review and approve.
  - Admins manage users and settings.
- Basic dashboard:
  - Number of payments processed.
  - How many were OK vs needed review.
  - Top suppliers by volume.

## Typical Use Case

1. An accountant exports a payment file from the company’s ERP/accounting system.
2. Instead of sending the file directly to the bank, they upload it to Velu.
3. Velu:
   - Checks each row against its supplier & bank account history.
   - Flags new or changed bank accounts and high-risk payments.
4. Velu generates:
   - A clean file with safe payments (“OK”) for the bank.
   - A separate list of payments that must be manually reviewed.
5. A finance manager logs in, reviews the flagged payments, and decides:
   - Approve once (only this time).
   - Approve and mark the new account as trusted.
   - Reject and investigate further.

## Tech Stack (High Level)

- **Backend:** Rust, Axum, SeaORM, PostgreSQL.
- **Frontend:** SvelteKit, Tailwind CSS.
- **Infra:** Docker, Caddy, VPS hosting (e.g. Hetzner) with HTTPS.

The project is structured as a mono-repo with `backend/`, `frontend/`, `docs/`, and deployment scripts.

## Why It’s Useful

- Reduces the risk of invoice fraud and bank account substitution.
- Provides clear responsibility and audit trail for who approved which payments.
- Fits naturally into existing workflows based on payment files, without replacing the bank or ERP.

