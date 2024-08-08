import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Sushiswap } from "../target/types/sushiswap";
import { Lptoken } from "../target/types/lptoken";
import { TOKEN_PROGRAM_ID, } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { getAssociatedTokenAddressSync, createAssociatedTokenAccountInstruction, MINT_SIZE, ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddress } from "@solana/spl-token";

import {DataV2, createCreateMetadataAccountV3Instruction} from '@metaplex-foundation/mpl-token-metadata';
import { Metaplex, UploadMetadataInput, bundlrStorage, keypairIdentity, findMetadataPda} from '@metaplex-foundation/js';
import { Transaction } from "@solana/web3.js";

describe("sushiswap", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // const sushiswap = anchor.workspace.Sushiswap as Program<Sushiswap>;
  const lptoken = anchor.workspace.Lptoken as Program<Lptoken>;

  let fromaccount = anchor.web3.Keypair.fromSecretKey(Uint8Array.from([
    16, 156, 69, 110, 153, 128, 125, 146, 147, 144,
    144, 187, 9, 196, 134, 132, 140, 209, 107, 135,
    237, 252, 245, 192, 157, 36, 197, 187, 218, 204,
    242, 226, 161, 67, 201, 161, 86, 70, 1, 23,
    132, 45, 239, 197, 11, 119, 77, 160, 20, 224,
    155, 147, 52, 6, 15, 134, 101, 254, 135, 226,
    230, 117, 177, 253
  ]));//  BrWbHJTXAuoPLz1fEY6HBeqNqPnj8mHrYEKQBcJ7sXFA


  let toaccount = anchor.web3.Keypair.fromSecretKey(Uint8Array.from([
    25, 25, 139, 206, 179, 172, 28, 39, 161, 113, 95,
    113, 45, 87, 252, 211, 177, 251, 25, 101, 117, 211,
    34, 255, 239, 122, 251, 36, 88, 140, 75, 207, 193,
    8, 235, 138, 180, 54, 209, 42, 44, 28, 35, 194,
    99, 131, 197, 154, 60, 184, 144, 238, 221, 58, 218,
    128, 64, 111, 118, 21, 113, 177, 176, 154
  ]))//DzXaL5Ubg1dsurvD9tvoRCGVWWW6uu9PZEHFTvRcLhGm

  let mainaccount = anchor.web3.Keypair.fromSecretKey(Uint8Array.from(
    [172,134,162,97,68,166,114,105,238,18,175,255,99,109,156,81,177,73,193,178,249,249,45,46,43,178,101,13,72,47,60,77,217,202,137,71,134,176,190,87,236,99,21,7,182,168,143,25,215,65,192,65,141,97,79,206,124,30,135,50,112,3,185,16]
    )) //EYpaBQmMpSrGFKwQL8W3eCpDgRneMo1XjgS5Vz4aUHKV

  


//   it("Create meta data ",async()=>{

// //     const metaplex = Metaplex.make(provider.connection)
// //   .use(keypairIdentity(toaccount))
// //   .use(bundlrStorage({
// //     address: 'http://localhost:8899',
// //     providerUrl: provider.connection.rpcEndpoint,
// //     timeout: 60000,
// //   }));


//     const [tokenXMint,Xbump] =  anchor.web3.PublicKey.findProgramAddressSync([
//       anchor.utils.bytes.utf8.encode("token"),
//       anchor.utils.bytes.utf8.encode("SBtc"),
//     ],lptoken.programId);

// //     const MY_TOKEN_METADATA: UploadMetadataInput = {
// //       name: "Solana Bitcoin",
// //       symbol: "SBTC",
// //       description: "This is a test token!",
// //       image: "https://cdn.pixabay.com/photo/2019/06/23/19/15/bitcoin-4294492_1280.png" //add public URL to image you'd like to use
// //   }


// //   const ON_CHAIN_METADATA = {
// //     name: MY_TOKEN_METADATA.name, 
// //     symbol: MY_TOKEN_METADATA.symbol,
// //     uri: 'TO_UPDATE_LATER',
// //     sellerFeeBasisPoints: 0,
// //     creators: null,
// //     collection: null,
// //     uses: null
// //   } as DataV2;

// //   const uploadMetadata = async (tokenMetadata: UploadMetadataInput): Promise<string> => {
// //     //Upload to Arweave
// //     const { uri } = await metaplex.nfts().uploadMetadata(tokenMetadata);
// //     console.log(`Arweave URL: `, uri);
// //     return uri;
// //   }


// //   const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
// //     'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s',
// // );

// //   // const requiredBalance = await getMinimumBalanceForRentExemptMint(provider.connection);
// //   //metadata account associated with mint
// //   const metadataPDA = await anchor.web3.PublicKey.findProgramAddressSync([
// //     Buffer.from('metadata'),
// //     TOKEN_METADATA_PROGRAM_ID.toBuffer(),
// //     tokenXMint.toBuffer(),
// //   ],TOKEN_METADATA_PROGRAM_ID);
  
// //   //get associated token account of your wallet
// //   const tokenATA = await getAssociatedTokenAddress(tokenXMint, toaccount.publicKey); 
  
  
//   // const tran = new Transaction().add(
//   //     createCreateMetadataAccountV3Instruction({
//   //       metadata: metadataPDA,
//   //       mint: tokenXMint,
//   //       mintAuthority: toaccount.publicKey,
//   //       payer: toaccount.publicKey,
//   //       updateAuthority: toaccount.publicKey,
//   //   }, {
//   //       createMetadataAccountArgsV3: {
//   //           data: ON_CHAIN_METADATA,
//   //           isMutable: true,
//   //           collectionDetails: null
//   //       }
//   //   })
//   // );
    

//   console.log(tokenXMint);


//   // console.log(await uploadMetadata(MY_TOKEN_METADATA));


//   })


  //  it("Find PDA",async()=>{


  //     const [tokenXMint,Xbump] =  anchor.web3.PublicKey.findProgramAddressSync([
  //       anchor.utils.bytes.utf8.encode("token"),
  //       anchor.utils.bytes.utf8.encode("SEth"),
  //     ],lptoken.programId);

  //     const [tokenYMint,Ybump] =  anchor.web3.PublicKey.findProgramAddressSync([
  //       anchor.utils.bytes.utf8.encode("token"),
  //       anchor.utils.bytes.utf8.encode("SBtc"),
  //     ],lptoken.programId);


  //     const [token_meta_dataPDA,metadatabump] =  anchor.web3.PublicKey.findProgramAddressSync([
  //       anchor.utils.bytes.utf8.encode("TokenPairMetadataTSS"),
  //       tokenXMint.toBuffer(),
  //       tokenYMint.toBuffer()
  //     ],sushiswap.programId);

  //     let txh = await sushiswap.methods.testFn().accounts({
  //       signer:toaccount.publicKey,
  //       tokenPairMetadataAccount:token_meta_dataPDA,
  //       tokenX:tokenXMint,
  //       tokenY:tokenYMint
  //     }).signers([toaccount]).rpc().catch((e)=>{
  //       console.log(e);
  //     });


  //  })



  //  it("Create Pair",async()=>{

  //     const [tokenXMint,Xbump] =  anchor.web3.PublicKey.findProgramAddressSync([
  //       anchor.utils.bytes.utf8.encode("token"),
  //       anchor.utils.bytes.utf8.encode("SEth"),
  //     ],lptoken.programId);

  //     const [tokenYMint,Ybump] =  anchor.web3.PublicKey.findProgramAddressSync([
  //       anchor.utils.bytes.utf8.encode("token"),
  //       anchor.utils.bytes.utf8.encode("SBtc"),
  //     ],lptoken.programId);


  //   const [sushilp_PDA,sushilpbump] =  anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("sushilptokenTS"),
  //     tokenXMint.toBuffer(),
  //     tokenYMint.toBuffer()
  //   ],lptoken.programId);



  //     const [token_meta_dataPDA,metadatabump] =  anchor.web3.PublicKey.findProgramAddressSync([
  //       anchor.utils.bytes.utf8.encode("TokenPairMetadataTS"),
  //       tokenXMint.toBuffer(),
  //       tokenYMint.toBuffer()
  //     ],sushiswap.programId);

  //     const [vaultPDA,vaultbump] = anchor.web3.PublicKey.findProgramAddressSync([
  //       anchor.utils.bytes.utf8.encode("vaultTS"),
  //       tokenXMint.toBuffer(),
  //       tokenYMint.toBuffer()
  //     ],lptoken.programId);

  //     let assosiatedaccount_for_x = await getAssociatedTokenAddressSync(tokenXMint,token_meta_dataPDA,true);

  //     let assosiatedaccount_for_y = await getAssociatedTokenAddressSync(tokenYMint,token_meta_dataPDA,true);

  //     let assosiatedaccount_for_sushilp = await getAssociatedTokenAddressSync(sushilp_PDA,toaccount.publicKey);
  //     let assosiatedaccount_for_fees = await getAssociatedTokenAddressSync(sushilp_PDA,sushilp_PDA,true);


  //     let txh = await sushiswap.methods.createPair(metadatabump).accounts({
  //       associatedAccountForFees:assosiatedaccount_for_fees,
  //       user:toaccount.publicKey,
  //       tokenX:tokenXMint,
  //       tokenY:tokenYMint,
  //       associatedAccountForSushilp:assosiatedaccount_for_sushilp,
  //       associatedAccountForX:assosiatedaccount_for_x,
  //       associatedAccountForY:assosiatedaccount_for_y,
  //       tokenPairMetadataAccount:token_meta_dataPDA,
  //       sushilptoken:sushilp_PDA,
  //       sushiTokenProgram:lptoken.programId,
  //       vault:vaultPDA,
  //       associatedTokenProgram:ASSOCIATED_TOKEN_PROGRAM_ID,
  //       systemProgram:anchor.web3.SystemProgram.programId,
  //       tokenProgram:TOKEN_PROGRAM_ID,
  //       rent:anchor.web3.SYSVAR_RENT_PUBKEY,
  //     }).signers([toaccount]).rpc().catch((e)=>{console.log(e)});

  //     console.log("Tx hash",txh);


  //     // let d = await provider.connection.getTokenAccountBalance(assosiatedaccount_for_x);

  //     // console.log(d);
  //     // console.log(d2);
  //     // let assosiatedaccount = await getAssociatedTokenAddressSync(tokenXMint,fromaccount.publicKey);

  //     // let d2 = await provider.connection.getTokenAccountBalance(assosiatedaccount_for_sushilp);

  //     // let d = await sushiswap.account.tokenPairMetadata.fetch(token_meta_dataPDA);

  //     // console.log(d);



  //  });



  // it("Add liquidity",async() =>{

  //       const [tokenXMint,Xbump] =  anchor.web3.PublicKey.findProgramAddressSync([
  //       anchor.utils.bytes.utf8.encode("token"),
  //       anchor.utils.bytes.utf8.encode("SEth"),
  //     ],lptoken.programId);

  //     const [tokenYMint,Ybump] =  anchor.web3.PublicKey.findProgramAddressSync([
  //       anchor.utils.bytes.utf8.encode("token"),
  //       anchor.utils.bytes.utf8.encode("SBtc"),
  //     ],lptoken.programId);


  //   const [sushilp_PDA,sushilpbump] =  anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("sushilptokenTS"),
  //     tokenXMint.toBuffer(),
  //     tokenYMint.toBuffer()
  //   ],lptoken.programId);

  //     const [token_meta_dataPDA,metadatabump] =  anchor.web3.PublicKey.findProgramAddressSync([
  //       anchor.utils.bytes.utf8.encode("TokenPairMetadataTS"),
  //       tokenXMint.toBuffer(),
  //       tokenYMint.toBuffer()
  //     ],sushiswap.programId);

  //     const [vaultPDA,vaultbump] = anchor.web3.PublicKey.findProgramAddressSync([
  //       anchor.utils.bytes.utf8.encode("vaultTS"),
  //       tokenXMint.toBuffer(),
  //       tokenYMint.toBuffer()
  //     ],lptoken.programId);

  //     let assosiatedaccount_for_x = await getAssociatedTokenAddressSync(tokenXMint,token_meta_dataPDA,true);

  //     let assosiatedaccount_for_y = await getAssociatedTokenAddressSync(tokenYMint,token_meta_dataPDA,true);

  //     let assosiatedaccount_for_sushilp = await getAssociatedTokenAddressSync(sushilp_PDA,toaccount.publicKey);

  //     let assosiatedaccount_for_fees = await getAssociatedTokenAddressSync(sushilp_PDA,sushilp_PDA,true);

  //     let from_assosiatedaccount_for_x = await getAssociatedTokenAddressSync(tokenXMint,toaccount.publicKey);

  //     let from_assosiatedaccount_for_y = await getAssociatedTokenAddressSync(tokenYMint,toaccount.publicKey);



  //   let amount = new anchor.BN(2 * 100000000);

  //   let tx = await sushiswap.methods.addLiquidity(amount,amount,amount,amount).accounts({
  //     associatedAccountForFees:assosiatedaccount_for_fees,
  //     associatedAccountForSushilp:assosiatedaccount_for_sushilp,
  //     fromAssociatedAccountForX:from_assosiatedaccount_for_x,
  //     fromAssociatedAccountForY:from_assosiatedaccount_for_y,
  //     sushilptoken:sushilp_PDA,
  //     toAssociatedAccountForX:assosiatedaccount_for_x,
  //     toAssociatedAccountForY:assosiatedaccount_for_y,
  //     tokenPairMetadataAccount:token_meta_dataPDA,
  //     user:toaccount.publicKey,
  //     vault:vaultPDA,
  //     sushiTokenProgram:lptoken.programId,
  //     associatedTokenProgram:ASSOCIATED_TOKEN_PROGRAM_ID,
  //     systemProgram:anchor.web3.SystemProgram.programId,
  //     tokenProgram:TOKEN_PROGRAM_ID,
  //     rent:anchor.web3.SYSVAR_RENT_PUBKEY,
  //     tokenX:tokenXMint,
  //     tokenY:tokenYMint
  //   }).signers([toaccount]).rpc().catch((e)=>{
  //     console.log(e);
  //   });

  //   console.log(tx);

  // })


  // it("Remove liquidity",async() =>{

  //         const [tokenXMint,Xbump] =  anchor.web3.PublicKey.findProgramAddressSync([
  //         anchor.utils.bytes.utf8.encode("token"),
  //         anchor.utils.bytes.utf8.encode("SEth"),
  //       ],lptoken.programId);

  //       const [tokenYMint,Ybump] =  anchor.web3.PublicKey.findProgramAddressSync([
  //         anchor.utils.bytes.utf8.encode("token"),
  //         anchor.utils.bytes.utf8.encode("SBtc"),
  //       ],lptoken.programId);


  //     const [sushilp_PDA,sushilpbump] =  anchor.web3.PublicKey.findProgramAddressSync([
  //       anchor.utils.bytes.utf8.encode("sushilptokenTS"),
  //       tokenXMint.toBuffer(),
  //       tokenYMint.toBuffer()
  //     ],lptoken.programId);

  //       const [token_meta_dataPDA,metadatabump] =  anchor.web3.PublicKey.findProgramAddressSync([
  //         anchor.utils.bytes.utf8.encode("TokenPairMetadataTS"),
  //         tokenXMint.toBuffer(),
  //         tokenYMint.toBuffer()
  //       ],sushiswap.programId);

  //       const [vaultPDA,vaultbump] = anchor.web3.PublicKey.findProgramAddressSync([
  //         anchor.utils.bytes.utf8.encode("vaultTS"),
  //         tokenXMint.toBuffer(),
  //         tokenYMint.toBuffer()
  //       ],lptoken.programId);

  //       let assosiatedaccount_for_x = await getAssociatedTokenAddressSync(tokenXMint,toaccount.publicKey);

  //       let assosiatedaccount_for_y = await getAssociatedTokenAddressSync(tokenYMint,toaccount.publicKey);

  //       let assosiatedaccount_for_sushilp = await getAssociatedTokenAddressSync(sushilp_PDA,toaccount.publicKey);

  //       let assosiatedaccount_for_fees = await getAssociatedTokenAddressSync(sushilp_PDA,sushilp_PDA,true);

  //       let from_assosiatedaccount_for_x = await getAssociatedTokenAddressSync(tokenXMint,token_meta_dataPDA,true);

  //       let from_assosiatedaccount_for_y = await getAssociatedTokenAddressSync(tokenYMint,token_meta_dataPDA,true);



  //     let amount = new anchor.BN(399999000);
  //     let amount0 = new anchor.BN(0);

  //     let tx = await sushiswap.methods.removeLiquidity(amount,amount0,amount0).accounts({
  //       associatedAccountForFees:assosiatedaccount_for_fees,
  //       associatedAccountForSushilp:assosiatedaccount_for_sushilp,
  //       fromAssociatedAccountForX:from_assosiatedaccount_for_x,
  //       fromAssociatedAccountForY:from_assosiatedaccount_for_y,
  //       sushilptoken:sushilp_PDA,
  //       toAssociatedAccountForX:assosiatedaccount_for_x,
  //       toAssociatedAccountForY:assosiatedaccount_for_y,
  //       tokenPairMetadataAccount:token_meta_dataPDA,
  //       user:toaccount.publicKey,
  //       vault:vaultPDA,
  //       sushiTokenProgram:lptoken.programId,
  //       associatedTokenProgram:ASSOCIATED_TOKEN_PROGRAM_ID,
  //       systemProgram:anchor.web3.SystemProgram.programId,
  //       tokenProgram:TOKEN_PROGRAM_ID,
  //       rent:anchor.web3.SYSVAR_RENT_PUBKEY,
  //       tokenX:tokenXMint,
  //       tokenY:tokenYMint
  //     }).signers([toaccount]).rpc().catch((e)=>{
  //       console.log(e);
  //     });

  //     console.log(tx);

  //   })


  // it("Swap", async () => {

  //   const [tokenXMint, Xbump] = anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("token"),
  //     anchor.utils.bytes.utf8.encode("SEth"),
  //   ], lptoken.programId);

  //   const [tokenYMint, Ybump] = anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("token"),
  //     anchor.utils.bytes.utf8.encode("SBtc"),
  //   ], lptoken.programId);


  //   const [sushilp_PDA, sushilpbump] = anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("sushilptokenTS"),
  //     tokenXMint.toBuffer(),
  //     tokenYMint.toBuffer()
  //   ], lptoken.programId);

  //   const [token_meta_dataPDA, metadatabump] = anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("TokenPairMetadataTS"),
  //     tokenXMint.toBuffer(),
  //     tokenYMint.toBuffer()
  //   ], sushiswap.programId);

  //   const [vaultPDA, vaultbump] = anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("vaultTS"),
  //     tokenXMint.toBuffer(),
  //     tokenYMint.toBuffer()
  //   ], lptoken.programId);

  //   let assosiatedaccount_for_x = await getAssociatedTokenAddressSync(tokenXMint, toaccount.publicKey);

  //   let assosiatedaccount_for_y = await getAssociatedTokenAddressSync(tokenYMint, toaccount.publicKey);

  //   let assosiatedaccount_for_sushilp = await getAssociatedTokenAddressSync(sushilp_PDA, toaccount.publicKey);

  //   let assosiatedaccount_for_fees = await getAssociatedTokenAddressSync(sushilp_PDA, sushilp_PDA, true);

  //   let from_assosiatedaccount_for_x = await getAssociatedTokenAddressSync(tokenXMint, token_meta_dataPDA, true);

  //   let from_assosiatedaccount_for_y = await getAssociatedTokenAddressSync(tokenYMint, token_meta_dataPDA, true);



  //   let amount = new anchor.BN(1 * 100000000);
  //   let amountOut = new anchor.BN(0);

  //   let tx = await sushiswap.methods.swap(amount, amountOut,true).accounts({
  //     associatedAccountForFees: assosiatedaccount_for_fees,
  //     associatedAccountForSushilp: assosiatedaccount_for_sushilp,
  //     poolAssociatedAccountForX: from_assosiatedaccount_for_x,
  //     poolAssociatedAccountForY: from_assosiatedaccount_for_y,
  //     sushilptoken: sushilp_PDA,
  //     toAssociatedAccountForX: assosiatedaccount_for_x,
  //     toAssociatedAccountForY: assosiatedaccount_for_y,
  //     tokenPairMetadataAccount: token_meta_dataPDA,
  //     user: toaccount.publicKey,
  //     vault: vaultPDA,
  //     sushiTokenProgram: lptoken.programId,
  //     associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //     tokenProgram: TOKEN_PROGRAM_ID,
  //     rent: anchor.web3.SYSVAR_RENT_PUBKEY,
  //     tokenX: tokenXMint,
  //     tokenY: tokenYMint
  //   }).signers([toaccount]).rpc().catch((e) => {
  //     console.log(e);
  //   });

  //   console.log(tx);

  // })





  // it("Is initialized!", async () => {

  //   const [sushilpPDA,lpbump] =  anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("sushilptoken"),
  //     anchor.utils.bytes.utf8.encode("tokena"),
  //     anchor.utils.bytes.utf8.encode("tokenb"),
  //   ],lptoken.programId);

  //   // console.log(sushilpPDA);

  //   const [vaultPDA,vaultbump] = anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("vault"),
  //     anchor.utils.bytes.utf8.encode("tokena"),
  //     anchor.utils.bytes.utf8.encode("tokenb"),
  //   ],lptoken.programId);


  //   let assosiatedaccount = await getAssociatedTokenAddressSync(sushilpPDA,fromaccount.publicKey);

  //   let txh = await lptoken.methods.initialize("tokena","tokenb").accounts({
  //     owner:fromaccount.publicKey,
  //     sushilptoken:sushilpPDA,
  //     vault:vaultPDA,
  //     tokenProgram:TOKEN_PROGRAM_ID,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //     rent:anchor.web3.SYSVAR_RENT_PUBKEY,
  //     associatedAccount:assosiatedaccount,
  //     associatedTokenProgram:ASSOCIATED_TOKEN_PROGRAM_ID
  //   }).signers([fromaccount]).rpc();

  //   console.log(txh);

  // });


  it("Create token!", async () => {

    const [sushilpPDA,lpbump] =  anchor.web3.PublicKey.findProgramAddressSync([
      anchor.utils.bytes.utf8.encode("token"),
      anchor.utils.bytes.utf8.encode("SEth"),
    ],lptoken.programId);


    // console.log(sushilpPDA);

    const [vaultPDA,vaultbump] = anchor.web3.PublicKey.findProgramAddressSync([
      anchor.utils.bytes.utf8.encode("vault"),
      anchor.utils.bytes.utf8.encode("SEth"),
    ],lptoken.programId);

      const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
     'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s',
      ); 

    const metadataPDA = await anchor.web3.PublicKey.findProgramAddressSync([
    Buffer.from('metadata'),
    TOKEN_METADATA_PROGRAM_ID.toBuffer(),
    sushilpPDA.toBuffer(),
    ],TOKEN_METADATA_PROGRAM_ID);

    console.log(metadataPDA[0]);
    console.log(metadataPDA[1]);

    let assosiatedaccount = await getAssociatedTokenAddressSync(sushilpPDA,mainaccount.publicKey);

    

    let txh = await lptoken.methods.createNewToken("SEth",lpbump,vaultbump).accounts({

      owner:mainaccount.publicKey,
      sushilptoken:sushilpPDA,
      vault:vaultPDA,
      tokenProgram:TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
      rent:anchor.web3.SYSVAR_RENT_PUBKEY,
      associatedAccount:assosiatedaccount,
      associatedTokenProgram:ASSOCIATED_TOKEN_PROGRAM_ID,
      metadataAccount:metadataPDA[0],
      tokenMetadataProgram:TOKEN_METADATA_PROGRAM_ID,
      
    }).signers([mainaccount]).rpc().catch((e)=>{
      console.log(e);
    });

    console.log(txh);

  });

  // it("Mint Coin",async ()=>{

  //   const [tokenXMint,Xbump] =  anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("token"),
  //     anchor.utils.bytes.utf8.encode("SEth"),
  //   ],lptoken.programId);

  //   const [tokenYMint,Ybump] =  anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("token"),
  //     anchor.utils.bytes.utf8.encode("SBtc"),
  //   ],lptoken.programId);

  //   console.log(tokenYMint);

  //      const [sushilpPDA,lpbump] =await  anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("sushilptoken"),
  //   ],lptoken.programId);



  //   const [vaultPDA,vaultbump] =await anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("vault"),
  //     anchor.utils.bytes.utf8.encode("SBtc"),
  //   ],lptoken.programId);

  //   let assaccount = await getAssociatedTokenAddressSync(tokenYMint,toaccount.publicKey);

  //   let tx = await lptoken.methods.mintCoinTo("SBtc",new anchor.BN(10 * anchor.web3.LAMPORTS_PER_SOL)).accounts({
  //     authority:toaccount.publicKey,
  //     sushilptoken:tokenYMint,
  //     to:assaccount,
  //     tokenProgram:TOKEN_PROGRAM_ID,
  //     vault:vaultPDA,
  //     systemProgram:anchor.web3.SystemProgram.programId,
  //     rent:anchor.web3.SYSVAR_RENT_PUBKEY

  //   }).signers([toaccount]).rpc().catch((e)=>{
  //     console.log(e);
  //   });


  // })




  //  it("Get account", async () => {

  //   console.log("Mint size:",MINT_SIZE);

  //   let lamports = await provider.connection.getMinimumBalanceForRentExemption(MINT_SIZE);
  //   let pid = lptoken.programId;

  //     const [tokenXMint,Xbump] =  anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("token"),
  //     anchor.utils.bytes.utf8.encode("SEth"),
  //   ],lptoken.programId);

  //   const [tokenYMint,Ybump] =  anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("token"),
  //     anchor.utils.bytes.utf8.encode("SBtc"),
  //   ],lptoken.programId);

  //     const [sushilp_PDA,sushilpbump] =  anchor.web3.PublicKey.findProgramAddressSync([
  //   anchor.utils.bytes.utf8.encode("sushilptokenTS"),
  //   tokenXMint.toBuffer(),
  //   tokenYMint.toBuffer()
  // ],lptoken.programId);

  //   let assosiatedaccount = await getAssociatedTokenAddressSync(tokenYMint, toaccount.publicKey);

  //   const mint_tx = new anchor.web3.Transaction().add(
  //     // anchor.web3.SystemProgram.createAccount({
  //     //   fromPubkey: fromaccount.publicKey,
  //     //   newAccountPubkey: mint.publicKey,
  //     //   space: MINT_SIZE,
  //     //   lamports: lamports,
  //     //   programId: TOKEN_PROGRAM_ID
  //     // }),

  //     // createInitializeMintInstruction(
  //     //   mint.publicKey,
  //     //   0,
  //     //   fromaccount.publicKey,
  //     //   fromaccount.publicKey,
  //     // ),

  //     createAssociatedTokenAccountInstruction(
  //       toaccount.publicKey,
  //       assosiatedaccount,
  //       toaccount.publicKey,
  //       tokenYMint
  //     )


  //   )

  //   let tx = await provider.sendAndConfirm(mint_tx,[toaccount]);

  //   console.log(tx);

  // })


  // it("Mint token to ",async () => {

  //   const [sushilpPDA,lpbump] =await  anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("sushilptoken"),
  //   ],lptoken.programId);



  //   const [vaultPDA,vaultbump] =await anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("vault"),
  //   ],lptoken.programId); 

  //   let assosiatedaccount = await getAssociatedTokenAddressSync(sushilpPDA,vaultPDA,true);


  //   let txh = await lptoken.methods.mintTokenTo(new anchor.BN(10 * anchor.web3.LAMPORTS_PER_SOL)).accounts({
  //     authority:fromaccount.publicKey,
  //     sushilptoken:sushilpPDA,
  //     to:assosiatedaccount,
  //     tokenProgram:TOKEN_PROGRAM_ID,
  //     vault:vaultPDA,
  //     systemProgram:anchor.web3.SystemProgram.programId,
  //     rent:anchor.web3.SYSVAR_RENT_PUBKEY
  //   }).signers([fromaccount]).rpc();

  //   console.log(txh);


  // })





  // it("Register SushiLP Token",async()=>{

  //   const [sushilpPDA,lpbump] =await  anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("sushilptoken"),
  //   ],lptoken.programId);

  //   const [vaultPDA,vaultbump] =await anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("vault"),
  //   ],lptoken.programId);

  //   console.log(vaultPDA);

  //   const [authorityPDA,authorityBump] =await anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("testauth"),
  //   ],lptoken.programId);

  //   let assosiatedaccount = await getAssociatedTokenAddressSync(sushilpPDA,vaultPDA,true);

  //   let txh = await lptoken.methods.registerLptokenAccount().accounts({
  //     user:toaccount.publicKey,
  //     sushilptoken:sushilpPDA,
  //     vault:vaultPDA,
  //     associatedTokenProgram:ASSOCIATED_TOKEN_PROGRAM_ID,
  //     sushiLptokenAccount:assosiatedaccount,
  //     systemProgram:anchor.web3.SystemProgram.programId,
  //     tokenProgram:TOKEN_PROGRAM_ID,
  //     rent:anchor.web3.SYSVAR_RENT_PUBKEY
  //   }).signers([toaccount]).rpc().catch((e)=>{
  //     console.log(e);
  //   });

  //   console.log(txh);

  // })


  // it("Transfer token",async()=>{

  //   const [sushilpPDA,lpbump] =await  anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("sushilptoken"),
  //   ],lptoken.programId);

  //     const [vaultPDA,vaultbump] =await anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("vault"),
  //   ],lptoken.programId);


  //   let from_ata = await getAssociatedTokenAddressSync(sushilpPDA,vaultPDA,true);
  //   let to_ata = await getAssociatedTokenAddressSync(sushilpPDA,fromaccount.publicKey);

  //   let txh = await lptoken.methods.transferToken(new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL)).
  //   accounts({
  //     from:toaccount.publicKey,
  //     fromAta:from_ata,
  //     toAta:to_ata,
  //     tokenProgram:TOKEN_PROGRAM_ID,
  //     vault:vaultPDA
  //   }).signers([toaccount]).rpc().catch((e)=>{
  //     console.log(e);
  //   });

  // })


  // it("Freeze user account",async()=>{

  //   const [sushilpPDA,lpbump] =await  anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("sushilptoken"),
  //   ],lptoken.programId);

  //   const [vaultPDA,vaultbump] =await anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("vault"),
  //   ],lptoken.programId);

  //   let account_to_freeze = await getAssociatedTokenAddressSync(sushilpPDA,toaccount.publicKey);

  //   let txh = await lptoken.methods.freezeUserAccount().
  //   accounts({
  //     authority:fromaccount.publicKey,
  //     sushilptoken:sushilpPDA,
  //     vault:vaultPDA,
  //     accountToBeFreeze:account_to_freeze,
  //     systemProgram:anchor.web3.SystemProgram.programId,
  //     tokenProgram:TOKEN_PROGRAM_ID
  //   }).signers([fromaccount]).rpc();

  //   console.log(txh);

  // })




  // it("Unfreeze user account",async()=>{

  //   const [sushilpPDA,lpbump] =await  anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("sushilptoken"),
  //   ],lptoken.programId);

  //   const [vaultPDA,vaultbump] =await anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("vault"),
  //   ],lptoken.programId);

  //   let account_to_unfreeze = await getAssociatedTokenAddressSync(sushilpPDA,toaccount.publicKey);

  //   let txh = await lptoken.methods.unfreezeUserAccount().
  //   accounts({
  //     authority:fromaccount.publicKey,
  //     sushilptoken:sushilpPDA,
  //     vault:vaultPDA,
  //     accountToBeUnfreeze:account_to_unfreeze,
  //     systemProgram:anchor.web3.SystemProgram.programId,
  //     tokenProgram:TOKEN_PROGRAM_ID
  //   }).signers([fromaccount]).rpc();

  //   console.log(txh);

  // })


  // it("Burn token",async()=>{

  //   const [sushilpPDA,lpbump] =await  anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("sushilptoken"),
  //   ],lptoken.programId);

  //   const [vaultPDA,vaultbump] =await anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("vault"),
  //   ],lptoken.programId);

  //   let from_ata = await getAssociatedTokenAddressSync(sushilpPDA,toaccount.publicKey);

  //   let txh = await lptoken.methods.burnToken(new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL)).
  //   accounts({
  //     authority:toaccount.publicKey,
  //     sushilptoken:sushilpPDA,
  //     vault:vaultPDA,
  //     fromAta:from_ata,
  //     systemProgram:anchor.web3.SystemProgram.programId,
  //     tokenProgram:TOKEN_PROGRAM_ID
  //   }).signers([toaccount]).rpc();

  //   console.log(txh);

  // })






  // it("Get data", async () => {

  //   const [sushilpPDA, lpbump] = anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("sushilptoken"),
  //     anchor.utils.bytes.utf8.encode("tokena"),
  //     anchor.utils.bytes.utf8.encode("tokenb"),
  //   ], lptoken.programId);

  //   // console.log(sushilpPDA);

  //   const [vaultPDA, vaultbump] = anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("vault"),
  //     anchor.utils.bytes.utf8.encode("tokena"),
  //     anchor.utils.bytes.utf8.encode("tokenb"),
  //   ], lptoken.programId);

  //   // let assosiatedaccount = await getAssociatedTokenAddressSync(sushilpPDA,fromaccount.publicKey,true);

  //   // let data = await provider.connection.getTokenAccountBalance(assosiatedaccount);

  //   // console.log(data);

  //   const [tokenXMint, Xbump] = anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("token"),
  //     anchor.utils.bytes.utf8.encode("SEth"),
  //   ], lptoken.programId);

  //   const [tokenYMint, Ybump] = anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("token"),
  //     anchor.utils.bytes.utf8.encode("SBtc"),
  //   ], lptoken.programId);

  //   const [sushilp_PDA, sushilpbump] = anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("sushilptokenTS"),
  //     tokenXMint.toBuffer(),
  //     tokenYMint.toBuffer()
  //   ], lptoken.programId);

  //   const [token_meta_dataPDA, metadatabump] = anchor.web3.PublicKey.findProgramAddressSync([
  //     anchor.utils.bytes.utf8.encode("TokenPairMetadataTS"),
  //     tokenXMint.toBuffer(),
  //     tokenYMint.toBuffer()
  //   ], sushiswap.programId);

  //   console.log(tokenXMint);

  //   let d = await sushiswap.account.tokenPairMetadata.fetch(token_meta_dataPDA);

  //   console.log(d.kLast.toString());
  //   console.log(d.reserveX.toNumber());
  //   console.log(d.reserveY.toNumber());


  //   let assosiatedaccount_for_sushilp = await getAssociatedTokenAddressSync(sushilp_PDA, sushilp_PDA,true);

  //   let ass = await provider.connection.getTokenAccountBalance(assosiatedaccount_for_sushilp);

  //   console.log(ass);


  //   let assosiatedaccount_for_sushilp_uer = await getAssociatedTokenAddressSync(sushilp_PDA, toaccount.publicKey);

  //   let ass_user = await provider.connection.getTokenAccountBalance(assosiatedaccount_for_sushilp_uer);

  //   console.log(ass_user);

  //   let assaccount = await getAssociatedTokenAddressSync(tokenXMint,token_meta_dataPDA,true);

  //   let data = await provider.connection.getTokenAccountBalance(assaccount);

  //   console.log(data);


  //   let assaccount1 = await getAssociatedTokenAddressSync(tokenYMint, token_meta_dataPDA,true);

  //   let data1 = await provider.connection.getTokenAccountBalance(assaccount1);

  //   console.log(data1);


  // })




});



