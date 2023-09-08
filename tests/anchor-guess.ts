import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorGuess } from "../target/types/anchor_guess";
import { expect } from 'chai'
describe("anchor-guess", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorGuess as Program<AnchorGuess>;
  it('initialize!', async () => {

    const gameKeypair = anchor.web3.Keypair.generate();
    const player = (program.provider as anchor.AnchorProvider).wallet;
    
    await program.methods
      .setupGame()
      .accounts({
        game: gameKeypair.publicKey,
        player: player.publicKey,
      })
      .signers([gameKeypair])
      .rpc();
    let gameState = await program.account.game.fetch(gameKeypair.publicKey);
    expect(gameState.moveCount).to.equal(0)
    expect(gameState.player).to.eql(player.publicKey)
    expect(gameState.state).to.eql({active: {}})
    expect(gameState.secret.length).to.equal(5);
  })
  it('guess that miss all', async () => {

    const gameKeypair = anchor.web3.Keypair.generate();
    const player = (program.provider as anchor.AnchorProvider).wallet;
    
    await program.methods
      .setupGame()
      .accounts({
        game: gameKeypair.publicKey,
        player: player.publicKey,
      })
      .signers([gameKeypair])
      .rpc();
    let gameState = await program.account.game.fetch(gameKeypair.publicKey);
    gameState.secret = "penta" // Mock secret for test
    await program.methods
      .guess("holla")
      .accounts({
        player: player.publicKey,
        game: gameKeypair.publicKey,
      })
      .signers([])
      .rpc()
    gameState = await program.account.game.fetch(gameKeypair.publicKey);
    expect(gameState.moveCount).to.equal(1)
    expect(gameState.guessState).to.eql([{ inCorrectPosition: {} }, { inCorrectPosition: {} }, { inCorrectPosition: {} }, { inCorrectPosition: {} }, { inCorrectPosition: {} }]);
    })


  })
