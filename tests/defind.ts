import * as anchor from "@coral-xyz/anchor";
import {Program, setProvider} from "@coral-xyz/anchor";
import { Defind } from "../target/types/defind";
import {expect} from "chai";

describe("defind", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace.Defind as Program<Defind>;

  const counter = anchor.web3.Keypair.generate()

  // const deposit_data = {
  //     owner: provider.wallet.publicKey, //32
  //     pub deposits: u64, //1
  //     pub share: f32, //4
  //     pub fund: Pubkey, //32
  // }

  const fund = {
    name: "test",
  }

  const [fundPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("fundaccount"), provider.wallet.publicKey.toBuffer()],
      program.programId
  )

  it("Bank created!", async  () => {
    const tx = await program.methods
        .create(fund.name)
        .accounts({
          fund: fundPda,
          user: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc()

    const account = await program.account.fund.fetch(fundPda)
    expect(fund.name == account.name)
  })

  it("Is initialized!", async () => {})
  //
  // it("Incremented the count", async () => {})
  //
  // it("Deposit successfully", async () => {
  // })
});
