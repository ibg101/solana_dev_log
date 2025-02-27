import * as anchor from "@coral-xyz/anchor";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, Transaction } from "@solana/web3.js";


describe("sealevel runtime test", () => {
    anchor.setProvider(anchor.AnchorProvider.env());
    const program = anchor.workspace.SmartContracts as anchor.Program;
    const provider = anchor.getProvider() as anchor.AnchorProvider;
    const mainPayer = provider.wallet;
    const batchSize = 5;
    const sleep = (ms: number) => new Promise(res => setTimeout(res, ms));

    const sendTransactionsInBatches = async (txs: Transaction[], signer: anchor.Wallet) => {
        const signedTxs = await signer.signAllTransactions(txs);
        for (const tx of signedTxs) {
            const sig = await provider.connection.sendRawTransaction(tx.serialize(), { skipPreflight: false });
            await provider.connection.confirmTransaction(sig, "processed");
        }
    };

    // !!! IMPORTANT !!! Although all transactions are signed simultaneously, they are executed sequentially. 
    // This is due to the fact that the mainPayer account's state is being mutated (its SOL balance is reduced in each transaction), 
    // which may cause issues if all transactions are sent concurrently.
    const airdropUsers = async (users: Keypair[]) => {
        const { blockhash } = await provider.connection.getLatestBlockhash();
        const txs = users.map(user => {
            const tx = new Transaction().add(
                anchor.web3.SystemProgram.transfer({
                    fromPubkey: mainPayer.publicKey,
                    toPubkey: user.publicKey,
                    lamports: 2 * LAMPORTS_PER_SOL
                })
            );
            tx.recentBlockhash = blockhash;
            tx.feePayer = mainPayer.publicKey;
            return tx;
        });

        await sendTransactionsInBatches(txs, mainPayer as anchor.Wallet);
    };

    const processUsersInBatches = async (users: Keypair[], method: string) => {
        console.time(`${method} processing`);
        for (let i = 0; i < users.length; i += batchSize) {
            const batch = users.slice(i, i + batchSize);
            const { blockhash } = await provider.connection.getLatestBlockhash();

            const txs = await Promise.all(batch.map(async (user) => {
                const [PDA, _bump] = PublicKey.findProgramAddressSync(
                    [
                        Buffer.from("meta"), 
                        user.publicKey.toBuffer()
                    ], 
                    program.programId
                );
                const tx = new Transaction().add(
                    await program.methods[method]()
                        .accounts({ meta: PDA, signer: user.publicKey, systemProgram: anchor.web3.SystemProgram.programId })
                        .signers([user])
                        .instruction()
                );
                tx.recentBlockhash = blockhash;
                tx.feePayer = user.publicKey;
                const wallet = new anchor.Wallet(user);
                return wallet.signTransaction(tx);
            }));

            let resultSigs = await Promise.allSettled(
                txs.map(tx => provider.connection.sendRawTransaction(tx.serialize(), { skipPreflight: false }))
            );

            resultSigs.forEach((sig) => {
                if (sig.status == "fulfilled") {
                    console.log(`✅ ${method} tx success: ${sig.value}`);
                } else {
                    console.error(`❌ ${method} tx failed:`, sig.reason);
                }
            });
        }
        console.timeEnd(`${method} processing`);
    };

    it("executes multiple transactions in parallel", async () => {
        const users = Array.from({ length: 20 }, () => Keypair.generate());
        
        await airdropUsers(users);
        await processUsersInBatches(users, "initPda");
        // for simplicity sake using timeout instead of confirming txs by commitment 
        await sleep(10000);
        await processUsersInBatches(users, "updatePda");
    });
});