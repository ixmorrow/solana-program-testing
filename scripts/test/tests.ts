import * as chai from 'chai'
import 'mocha'
import * as web3 from '@solana/web3.js'
import * as borsh from "@project-serum/borsh"

// Using local cluster
const RPC_ENDPOINT_URL = "http://localhost:8899"
const commitment = 'confirmed'
const connection = new web3.Connection(RPC_ENDPOINT_URL, commitment)

// MY WALLET SETTING
const id_json_path = require('os').homedir() + "/.config/solana/test-wallet.json"
const secret = Uint8Array.from(JSON.parse(require("fs").readFileSync(id_json_path)))
const wallet = web3.Keypair.fromSecretKey(secret as Uint8Array)

const program_id = new web3.PublicKey("Gh6NvggsWpEvKqwqJ7gM4KaUmQTeVZVvyETzVV4E7ABt")

// declare testing section with Mocha
describe("begin tests", async () => {
    const keypair = new web3.Keypair()

    let pda = (await web3.PublicKey.findProgramAddress(
        [keypair.publicKey.toBuffer()],
        program_id
      ))[0];
    const tx = new web3.Transaction()

    // first Mocha test
    it('first test', async () => {

        const ix = ix_obj(
            Buffer.from([1,2,3]),
            keypair.publicKey,
            pda,
            program_id
        )

        tx.add(ix)

        //console.log("Requesting Airdrop of 10 SOL...");
        await connection.requestAirdrop(keypair.publicKey, 10e9);
        //console.log("Airdrop received");

        let txid = await web3.sendAndConfirmTransaction(connection, tx, [keypair], {
            skipPreflight: true,
            preflightCommitment: "confirmed"
            });
        
        // sleep to allow time to update
        await new Promise((resolve) => setTimeout(resolve, 1000))

        const acct_info = await connection.getAccountInfo(pda)

        const acct = acct_struct.decode(acct_info.data)
        //console.log(acct)
        chai.expect(acct.num).to.equal(1)
    })

    it('second test', async () => {
        const acct_info = await connection.getAccountInfo(pda)
        const acct = acct_struct.decode(acct_info.data)
        chai.expect(acct.num).to.equal(1)
    })
})

const ix_obj = (
    i: Buffer,
    payer: web3.PublicKey,
    test_acct: web3.PublicKey,
    program_id: web3.PublicKey
    ) => {
        return new web3.TransactionInstruction({
            keys: [
                {
                    pubkey: payer,
                    isSigner: true,
                    isWritable: true

                },
                {
                    pubkey: test_acct,
                    isSigner: false,
                    isWritable: true
                },
                {
                    pubkey: web3.SystemProgram.programId,
                    isSigner: false,
                    isWritable: false
                }
            ],
            data: i,
            programId: program_id
    })
}

export const acct_struct = borsh.struct([
    borsh.u8("num")
])