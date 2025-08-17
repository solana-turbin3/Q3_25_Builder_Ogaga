import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Capstone } from "../target/types/capstone";
import {
    Keypair,
    SystemProgram,
    PublicKey,
    LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import {
    createMint,
    getOrCreateAssociatedTokenAccount,
    mintTo,
    getAccount,
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
    getAssociatedTokenAddress,
} from "@solana/spl-token";
import { expect } from "chai";

// Error constants matching Rust program's CustomError enum
const PROGRAM_ERRORS = {
    NOT_A_MEMBER: "You are not a member of this circle",
    INVALID_INVITE_CODE: "Invalid invite code",
    REQUEST_NOT_ACTIVE: "Request is not active",
    ALREADY_VOTED: "You have already voted on this request",
    REQUEST_NOT_APPROVED: "Request not approved",
    INSUFFICIENT_FUNDS: "Insufficient funds in treasury",
} as const;

// Test configuration constants
const TEST_CONFIG = {
    USDC_DECIMALS: 6,
    SOL_AIRDROP_AMOUNT: 2,
    INITIAL_USDC_BALANCE: 500,
    STANDARD_CONTRIBUTION: 100,
    STANDARD_REQUEST: 50,
    SMALL_REQUEST: 25,
    CIRCLE_NAME: "Lagos Circle",
    INVITE_CODE: "LAGOS123",
    INVALID_INVITE: "WRONGCODE",
    SETUP_DELAY_MS: 2000,
} as const;

describe("DAOjo Savings Circle Program", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.Capstone as Program<Capstone>;

    // Test participants
    let creator: Keypair;
    let member1: Keypair;
    let member2: Keypair;
    let nonMember: Keypair;

    // USDC token setup
    let usdcMint: PublicKey;
    let creatorUsdcAccount: PublicKey;
    let member1UsdcAccount: PublicKey;
    let member2UsdcAccount: PublicKey;
    let nonMemberUsdcAccount: PublicKey;
    let treasuryTokenAccount: PublicKey;

    // Program account addresses
    let circlePda: PublicKey;
    let treasuryAuthorityPda: PublicKey;

    // Helper functions for better test readability
    const toUsdcAmount = (amount: number) => new anchor.BN(amount * 10 ** TEST_CONFIG.USDC_DECIMALS);
    const fromUsdcAmount = (amount: bigint) => Number(amount) / (10 ** TEST_CONFIG.USDC_DECIMALS);

    /**
     * Comprehensive test environment setup
     * Creates accounts, mints tokens, and derives PDAs needed for testing
     */
    before(async () => {
        // Arrange: Create test keypairs
        creator = Keypair.generate();
        member1 = Keypair.generate();
        member2 = Keypair.generate();
        nonMember = Keypair.generate();

        // Arrange: Fund all accounts with SOL for transaction fees
        const airdropAmount = TEST_CONFIG.SOL_AIRDROP_AMOUNT * LAMPORTS_PER_SOL;
        await Promise.all([
            provider.connection.requestAirdrop(creator.publicKey, airdropAmount),
            provider.connection.requestAirdrop(member1.publicKey, airdropAmount),
            provider.connection.requestAirdrop(member2.publicKey, airdropAmount),
            provider.connection.requestAirdrop(nonMember.publicKey, airdropAmount),
        ]);

        // Wait for airdrops to complete
        await new Promise(resolve => setTimeout(resolve, TEST_CONFIG.SETUP_DELAY_MS));

        // Arrange: Create USDC mint for testing token operations
        usdcMint = await createMint(
            provider.connection,
            creator,
            creator.publicKey,
            creator.publicKey,
            TEST_CONFIG.USDC_DECIMALS
        );

        // Arrange: Create token accounts for all participants
        const tokenAccounts = await Promise.all([
            getOrCreateAssociatedTokenAccount(provider.connection, creator, usdcMint, creator.publicKey),
            getOrCreateAssociatedTokenAccount(provider.connection, creator, usdcMint, member1.publicKey),
            getOrCreateAssociatedTokenAccount(provider.connection, creator, usdcMint, member2.publicKey),
            getOrCreateAssociatedTokenAccount(provider.connection, creator, usdcMint, nonMember.publicKey),
        ]);

        [creatorUsdcAccount, member1UsdcAccount, member2UsdcAccount, nonMemberUsdcAccount] = 
            tokenAccounts.map(account => account.address);

        // Arrange: Mint initial USDC balances to all participants
        const initialBalance = TEST_CONFIG.INITIAL_USDC_BALANCE * 10 ** TEST_CONFIG.USDC_DECIMALS;
        await Promise.all([
            mintTo(provider.connection, creator, usdcMint, creatorUsdcAccount, creator, initialBalance),
            mintTo(provider.connection, creator, usdcMint, member1UsdcAccount, creator, initialBalance),
            mintTo(provider.connection, creator, usdcMint, member2UsdcAccount, creator, initialBalance),
            mintTo(provider.connection, creator, usdcMint, nonMemberUsdcAccount, creator, initialBalance),
        ]);

        // Arrange: Derive program-derived addresses (PDAs)
        [circlePda] = PublicKey.findProgramAddressSync(
            [Buffer.from("circle"), Buffer.from(TEST_CONFIG.INVITE_CODE)],
            program.programId
        );

        [treasuryAuthorityPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("treasury_auth"), Buffer.from(TEST_CONFIG.INVITE_CODE)],
            program.programId
        );

        treasuryTokenAccount = await getAssociatedTokenAddress(
            usdcMint,
            treasuryAuthorityPda,
            true
        );


    });

    describe("Circle Creation", () => {
        it("should create a savings circle with USDC treasury", async () => {
            // Arrange: Set up circle parameters
            const expectedName = TEST_CONFIG.CIRCLE_NAME;
            const expectedContribution = toUsdcAmount(TEST_CONFIG.STANDARD_CONTRIBUTION);
            const expectedInviteCode = TEST_CONFIG.INVITE_CODE;

            // Act: Create the savings circle
            await program.methods
                .createCircle(expectedName, expectedContribution, expectedInviteCode)
                .accounts({
                    creator: creator.publicKey,
                    circleAccount: circlePda,
                    treasuryTokenAccount: treasuryTokenAccount,
                    treasuryAuthority: treasuryAuthorityPda,
                    usdcMint: usdcMint,
                    systemProgram: SystemProgram.programId,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                } as any)
                .signers([creator])
                .rpc();

            // Assert: Verify circle was created with correct parameters
            const circleAccount = await program.account.circleAccount.fetch(circlePda);
            
            expect(circleAccount.name).to.equal(expectedName);
            expect(circleAccount.contributionAmount.toString()).to.equal(expectedContribution.toString());
            expect(circleAccount.creator.toBase58()).to.equal(creator.publicKey.toBase58());
            expect(circleAccount.memberCount).to.equal(1);
            expect(circleAccount.member1.toBase58()).to.equal(creator.publicKey.toBase58());
            expect(circleAccount.inviteCode).to.equal(expectedInviteCode);
            expect(circleAccount.bump).to.be.greaterThan(0);

            // Assert: Verify treasury token account exists
            const treasuryAccount = await getAccount(provider.connection, treasuryTokenAccount);
            expect(treasuryAccount).to.exist;


        });
    });

    describe("Circle Membership", () => {
        it("should allow valid users to join circle with correct invite code", async () => {
            // Act: Member 1 joins the circle
            await program.methods
                .joinCircle(TEST_CONFIG.INVITE_CODE)
                .accounts({
                    joiner: member1.publicKey,
                    circleAccount: circlePda,
                } as any)
                .signers([member1])
                .rpc();

            // Act: Member 2 joins the circle
            await program.methods
                .joinCircle(TEST_CONFIG.INVITE_CODE)
                .accounts({
                    joiner: member2.publicKey,
                    circleAccount: circlePda,
                } as any)
                .signers([member2])
                .rpc();

            // Assert: Verify membership updates
            const circleAccount = await program.account.circleAccount.fetch(circlePda);
            
            expect(circleAccount.memberCount).to.equal(3);
            expect(circleAccount.member1.toBase58()).to.equal(creator.publicKey.toBase58());
            expect(circleAccount.member2.toBase58()).to.equal(member1.publicKey.toBase58());
            expect(circleAccount.member3.toBase58()).to.equal(member2.publicKey.toBase58());


        });

        it("should reject joining attempts with invalid invite codes", async () => {
            // Arrange: Generate PDA with invalid invite code
            const [wrongCirclePda] = PublicKey.findProgramAddressSync(
                [Buffer.from("circle"), Buffer.from(TEST_CONFIG.INVALID_INVITE)],
                program.programId
            );

            // Act & Assert: Attempt to join with wrong invite code should fail
            try {
                await program.methods
                    .joinCircle(TEST_CONFIG.INVALID_INVITE)
                    .accounts({
                        joiner: nonMember.publicKey,
                        circleAccount: wrongCirclePda,
                    } as any)
                    .signers([nonMember])
                    .rpc();
                
                expect.fail("Expected join attempt with invalid invite code to fail");
            } catch (error: any) {
                // Accept various error types that indicate account doesn't exist or constraint violation
                const validErrorIndicators = [
                    "Account does not exist",
                    "AccountNotInitialized",
                    "The program expected this account to be already initialized",
                    "Invalid account data",
                    "3012", // Account not initialized error code
                ];
                
                const hasValidError = validErrorIndicators.some(indicator => 
                    error.message.includes(indicator)
                );
                
                expect(hasValidError).to.be.true;

            }
        });
    });

    describe("Contributions", () => {
        it("should allow circle members to contribute USDC to treasury", async () => {
            // Arrange: Record initial treasury balance
            const treasuryBalanceBefore = await getAccount(provider.connection, treasuryTokenAccount);
            const expectedContribution = toUsdcAmount(TEST_CONFIG.STANDARD_CONTRIBUTION);

            // Act: Member contributes to treasury
            await program.methods
                .contribute(TEST_CONFIG.INVITE_CODE)
                .accounts({
                    member: member1.publicKey,
                    circleAccount: circlePda,
                    memberTokenAccount: member1UsdcAccount,
                    treasuryTokenAccount: treasuryTokenAccount,
                    treasuryAuthority: treasuryAuthorityPda,
                    usdcMint: usdcMint,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                } as any)
                .signers([member1])
                .rpc();

            // Assert: Verify treasury balance increased by contribution amount
            const treasuryBalanceAfter = await getAccount(provider.connection, treasuryTokenAccount);
            const actualIncrease = Number(treasuryBalanceAfter.amount - treasuryBalanceBefore.amount);
            
            expect(actualIncrease).to.equal(Number(expectedContribution));

        });

        it("should prevent non-members from contributing to treasury", async () => {
            // Act & Assert: Non-member attempt to contribute should fail
            try {
                await program.methods
                    .contribute(TEST_CONFIG.INVITE_CODE)
                    .accounts({
                        member: nonMember.publicKey,
                        circleAccount: circlePda,
                        memberTokenAccount: nonMemberUsdcAccount,
                        treasuryTokenAccount: treasuryTokenAccount,
                        treasuryAuthority: treasuryAuthorityPda,
                        usdcMint: usdcMint,
                        tokenProgram: TOKEN_PROGRAM_ID,
                        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                    } as any)
                    .signers([nonMember])
                    .rpc();
                
                expect.fail("Expected non-member contribution to be rejected");
            } catch (error: any) {
                expect(error.message).to.include(PROGRAM_ERRORS.NOT_A_MEMBER);

            }
        });
    });

    describe("Funding Requests", () => {
        const requestAmount = toUsdcAmount(TEST_CONFIG.STANDARD_REQUEST);
        const requestDescription = "Emergency medical expense";
        let fundingRequestPda: PublicKey;

        it("should create funding request with proper initialization", async () => {
            // Arrange: Derive funding request PDA
            [fundingRequestPda] = PublicKey.findProgramAddressSync(
                [
                    Buffer.from("request"),
                    circlePda.toBuffer(),
                    member1.publicKey.toBuffer(),
                ],
                program.programId
            );

            // Act: Create funding request
            await program.methods
                .createRequest(TEST_CONFIG.INVITE_CODE, requestAmount, requestDescription)
                .accounts({
                    requester: member1.publicKey,
                    circleAccount: circlePda,
                    fundingRequest: fundingRequestPda,
                    systemProgram: SystemProgram.programId,
                } as any)
                .signers([member1])
                .rpc();

            // Assert: Verify request created with correct parameters
            const requestAccount = await program.account.fundingRequest.fetch(fundingRequestPda);
            
            expect(requestAccount.requester.toBase58()).to.equal(member1.publicKey.toBase58());
            expect(requestAccount.circle.toBase58()).to.equal(circlePda.toBase58());
            expect(requestAccount.amount.toString()).to.equal(requestAmount.toString());
            expect(requestAccount.description).to.equal(requestDescription);
            expect(requestAccount.votesFor).to.equal(0);
            expect(requestAccount.votesAgainst).to.equal(0);
            expect(requestAccount.voterCount).to.equal(0);
            expect(requestAccount.status).to.deep.equal({ active: {} });
            expect(requestAccount.createdAt.toNumber()).to.be.greaterThan(0);
            expect(requestAccount.bump).to.be.greaterThan(0);


        });

        it("should process votes and determine approval through majority consensus", async () => {
            // Act: Creator votes YES (1st vote)
            await program.methods
                .voteOnRequest(TEST_CONFIG.INVITE_CODE, true)
                .accounts({
                    voter: creator.publicKey,
                    circleAccount: circlePda,
                    fundingRequest: fundingRequestPda,
                } as any)
                .signers([creator])
                .rpc();

            // Assert: Verify first vote recorded correctly
            let requestAccount = await program.account.fundingRequest.fetch(fundingRequestPda);
            expect(requestAccount.votesFor).to.equal(1);
            expect(requestAccount.votesAgainst).to.equal(0);
            expect(requestAccount.voterCount).to.equal(1);
            expect(requestAccount.voter1.toBase58()).to.equal(creator.publicKey.toBase58());
            expect(requestAccount.status).to.deep.equal({ active: {} });

            // Act: Member 2 votes YES (2nd vote - creates majority 2/3)
            await program.methods
                .voteOnRequest(TEST_CONFIG.INVITE_CODE, true)
                .accounts({
                    voter: member2.publicKey,
                    circleAccount: circlePda,
                    fundingRequest: fundingRequestPda,
                } as any)
                .signers([member2])
                .rpc();

            // Assert: Verify majority approval achieved
            requestAccount = await program.account.fundingRequest.fetch(fundingRequestPda);
            expect(requestAccount.votesFor).to.equal(2);
            expect(requestAccount.votesAgainst).to.equal(0);
            expect(requestAccount.voterCount).to.equal(2);
            expect(requestAccount.voter1.toBase58()).to.equal(creator.publicKey.toBase58());
            expect(requestAccount.voter2.toBase58()).to.equal(member2.publicKey.toBase58());
            expect(requestAccount.status).to.deep.equal({ approved: {} });


        });

        it("should transfer approved funds from treasury to requester", async () => {
            // Arrange: Record balances before disbursement
            const requesterBalanceBefore = await getAccount(provider.connection, member1UsdcAccount);
            const treasuryBalanceBefore = await getAccount(provider.connection, treasuryTokenAccount);

            // Act: Disburse approved funds
            await program.methods
                .disburseFunds(TEST_CONFIG.INVITE_CODE)
                .accounts({
                    authority: creator.publicKey,
                    circleAccount: circlePda,
                    fundingRequest: fundingRequestPda,
                    requesterTokenAccount: member1UsdcAccount,
                    treasuryTokenAccount: treasuryTokenAccount,
                    treasuryAuthority: treasuryAuthorityPda,
                    usdcMint: usdcMint,
                    tokenProgram: TOKEN_PROGRAM_ID,
                } as any)
                .signers([creator])
                .rpc();

            // Assert: Verify token transfers occurred correctly
            const requesterBalanceAfter = await getAccount(provider.connection, member1UsdcAccount);
            const treasuryBalanceAfter = await getAccount(provider.connection, treasuryTokenAccount);

            const requesterIncrease = Number(requesterBalanceAfter.amount - requesterBalanceBefore.amount);
            const treasuryDecrease = Number(treasuryBalanceBefore.amount - treasuryBalanceAfter.amount);

            expect(requesterIncrease).to.equal(Number(requestAmount));
            expect(treasuryDecrease).to.equal(Number(requestAmount));

            // Assert: Verify request status updated to disbursed
            const requestAccount = await program.account.fundingRequest.fetch(fundingRequestPda);
            expect(requestAccount.status).to.deep.equal({ disbursed: {} });


        });
    });

    describe("Edge Cases", () => {
        it("should prevent duplicate voting by same member", async () => {
            // Arrange: Create new request for double voting test
            const [doubleVoteRequestPda] = PublicKey.findProgramAddressSync(
                [
                    Buffer.from("request"),
                    circlePda.toBuffer(),
                    member2.publicKey.toBuffer(),
                ],
                program.programId
            );

            const testRequestAmount = toUsdcAmount(TEST_CONFIG.SMALL_REQUEST);
            const testDescription = "Test double vote prevention";

            // Arrange: Create test request
            await program.methods
                .createRequest(TEST_CONFIG.INVITE_CODE, testRequestAmount, testDescription)
                .accounts({
                    requester: member2.publicKey,
                    circleAccount: circlePda,
                    fundingRequest: doubleVoteRequestPda,
                    systemProgram: SystemProgram.programId,
                } as any)
                .signers([member2])
                .rpc();

            // Act: Creator votes once (should succeed)
            await program.methods
                .voteOnRequest(TEST_CONFIG.INVITE_CODE, true)
                .accounts({
                    voter: creator.publicKey,
                    circleAccount: circlePda,
                    fundingRequest: doubleVoteRequestPda,
                } as any)
                .signers([creator])
                .rpc();

            // Act & Assert: Creator attempts to vote again (should fail)
            try {
                await program.methods
                    .voteOnRequest(TEST_CONFIG.INVITE_CODE, false)
                    .accounts({
                        voter: creator.publicKey,
                        circleAccount: circlePda,
                        fundingRequest: doubleVoteRequestPda,
                    } as any)
                    .signers([creator])
                    .rpc();
                
                expect.fail("Expected double voting attempt to be rejected");
            } catch (error: any) {
                expect(error.message).to.include(PROGRAM_ERRORS.ALREADY_VOTED);

            }

            // Assert: Verify vote count remains unchanged after failed duplicate attempt
            const requestAccount = await program.account.fundingRequest.fetch(doubleVoteRequestPda);
            expect(requestAccount.votesFor).to.equal(1);
            expect(requestAccount.voterCount).to.equal(1);
        });
    });
});
