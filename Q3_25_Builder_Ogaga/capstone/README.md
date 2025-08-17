# DAOjo - Nigerian Savings Groups on Solana

**Turbin3 Builders Cohort Capstone Project**

Overview

DAOjo is my take on bringing traditional Nigerian savings groups (Ajo/Esusu) to Solana. Instead of trusting one person to hold everyone's money and distribute it fairly, the whole thing runs on smart contracts with democratic voting.

You create a circle with some friends, family members, or coworkers, everyone contributes USDC monthly, and when someone needs money they make a request. The group votes yes or no, and if it passes, the funds transfer automatically. No coordinator needed, no risk of someone running off with the money.

This is an MVP implementation with core functionality working end-to-end. More features and improvements are planned for future versions.

**Devnet Program ID:** `3ZN15DR98zTR6nb6A9ekAziw2vRZ3JiWMZ3nrSyNkjMV`

## Why This Exists 

Traditional Ajo works great until it doesn't. You're basically trusting one person with everyone's money, hoping they don't disappear or make bad decisions about who gets paid when. Plus you're stuck with whatever rotation schedule was decided at the start.

DAOjo fixes the trust problem with on-chain treasury management and lets the group decide democratically when someone actually needs help, rather than following some rigid schedule that might not match real life. Unlike traditional Ajo where the coordinator can disappear with everyone's money, the funds in DAOjo are locked in a program-controlled account that no individual can access.

## How It Works

The whole thing is pretty straightforward:

1. **Create a circle** - Set up your group with a name, monthly contribution amount, and invite code
2. **Friends join** - Share the invite code, up to 3 people total per circle
3. **Everyone contributes** - Monthly USDC deposits go into a shared treasury (PDA-controlled)
4. **Someone needs money** - They create a funding request with amount and reason
5. **Group votes** - Democratic yes/no voting, majority wins
6. **Auto-payout** - If approved, USDC transfers directly from treasury to requester

## What's Actually Built

**Core Features:**
- `create_circle` - Start a new savings group with treasury
- `join_circle` - Join existing groups with invite codes  
- `contribute` - Deposit monthly USDC to shared treasury
- `create_request` - Submit funding requests to your circle
- `vote_on_request` - Democratic voting on funding requests
- `disburse_funds` - Auto-transfer approved funds from treasury

**Security stuff that actually matters:**
- Only circle members can contribute or vote
- Can't vote twice on the same request  
- Treasury is controlled by program, not individuals
- All vote counts and balances are transparent on-chain

## Running This Thing

You'll need the usual Solana dev setup - Rust, Anchor CLI, Node.js, etc. Nothing fancy.

```bash
# Build it
anchor build

# Test locally  
anchor test

# Test against the deployed devnet program
anchor test --provider.cluster devnet
```

The tests cover everything - creating circles, joining, contributing, voting, fund disbursement, and edge cases like double voting and non-member access attempts. All 10 tests should pass if everything's working right.

## Accomplishments

- Full end-to-end functionality on devnet
- All 6 core features working: circle creation through fund disbursement  
- Comprehensive test suite covering happy paths and edge cases
- Democratic voting with majority consensus logic
- Treasury security via PDA control (no one person can drain funds)
- Fixed-size account structures for predictable memory allocation

## What I Had to Simplify for the Capstone

**Limited to 3 members per circle.** Traditional Ajo groups can have 10-20+ members, but I went with fixed-size arrays (`member1`, `member2`, `member3`) because dynamic vectors were causing "memory allocation failed" errors that I couldn't debug in time. In production you'd definitely want flexible member counts - this was just a technical limitation I had to work around.

**No recurring contribution enforcement.** Right now anyone can contribute any amount any time. Traditional Ajo has strict monthly schedules, and production would need time-based contribution tracking and maybe penalties for missed payments.

**Simple majority voting.** Production might want more sophisticated voting mechanisms like quorum requirements or weighted voting based on contribution history, similar to how traditional Ajo groups develop trust over time.

## Technical Details 

The accounts are pretty simple:

**CircleAccount** - Stores circle info, members, and treasury details
**FundingRequest** - Tracks funding proposals, votes, and status

I went with fixed-size arrays instead of vectors because dynamic allocation was causing "memory allocation failed" errors that took forever to debug. So each circle maxes out at 3 members with individual `member1`, `member2`, `member3` fields. Same deal with voters on requests.

The treasury uses a PDA (program-derived address) so only the smart contract can authorize transfers. No one person can drain the funds even if they wanted to, a major contrast to the traditional Ajo system. 


