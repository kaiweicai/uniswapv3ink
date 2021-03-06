import BN from 'bn.js'
import { expect } from 'chai';
import { artifacts, network, patract } from 'redspot';

const { getContractFactory, getRandomSigner } = patract;

const { api, getAddresses, getSigners } = network;

import { setupContract, fromSigner, setupProxy } from './helpers'

describe('positionManager initialize', () => {
  after(() => {
    return api.disconnect();
  });

  async function setup() {
    // await api.isReady;
    // const signerAddresses = await getAddresses();
    // const Alice = signerAddresses[0];
    // const sender = await getRandomSigner(Alice, '20000 UNIT');
    // const contractFactory = await getContractFactory('weth9_contract', sender.address);
    // const contract = await contractFactory.deploy('new', 'None','None');
    // // const abi = artifacts.readArtifact('metadata');
    // const receiver = await getRandomSigner();
    // return { sender, contractFactory, contract, receiver, Alice };
  }

  // it('test initialize pool',async () =>{
  //   await api.isReady;
  //   const {defaultSigner:defaultSigner1,alice:alice1, query:factoryQuery,tx:factoryTx,contract:factoryContract,abi:factoryAbi} = await setupContract("factory","new");
  //   console.log("factory is:",1);
  //   console.log("defaultSigner is:",defaultSigner1.address.toString());
  //   console.log("alice1 is:",alice1.address.toString());
  //   const { contract:weth9Contract} = await setupContract('weth9_contract','new','weth9','weth9');
  //   console.log("factory is:",2);
  //   // pub fn new(factory: AccountId, weth9: AccountId,tokenDescriptor:AccountId) -> Self {
  //   const { contract:positionDescriptor} = await setupContract('NonfungibleTokenPositionDescriptor','new',weth9Contract.address,"_nativeCurrencyLabelBytes");
  //   console.log("factory is:",3);
  //   // pub fn new(factory: AccountId, weth9: AccountId,tokenDescriptor:AccountId) -> Self {
  //   const { query:positionManagerQuery,tx:positionManagerTx,alice,defaultSigner,contract:positionMangerContract } = await setupContract('NonfungiblePositionManager','new',factoryContract.address,weth9Contract.address,positionDescriptor.address,{value:1000000000});
  //   console.log("factory is:",4);
  //   const { contract:CHECoinContract} = await setupContract('stable_coin_contract','new',"CHE","CHE");
  //   const { contract:AAACoinContract} = await setupContract('stable_coin_contract','new',"AAA","AAA");
  //     // &mut self,token0: AccountId,token1: AccountId,fee: u32,sqrt_price_x96: Uint160,) -> Address 
  //     //fee 500,3000,10000
  //   console.log("positionMangerContract.address is:",positionMangerContract.address.toHuman());
    
  //   let token0;
  //   let token1;
  //   if(CHECoinContract.address.toHuman()<AAACoinContract.address.toHuman()) {
  //     token0 = CHECoinContract.address;
  //     token1 = AAACoinContract.address;
  //   }else{
  //     token1 = CHECoinContract.address;
  //     token0 = AAACoinContract.address;
  //   }
  //   console.log("token0.address is:",token0.toHuman());
  //   console.log("token1.address is:",token1.toHuman());
  //   console.log("factoryContract.address is:",factoryContract.address.toHuman());
  //   // factory:Address,token0: Address, token1: Address, fee: Uint24, tick_spacing: Int24
  //   const { abi:poolAbi} = await setupContract('pool','new',factoryContract.address,token0,token1,500,0);
    
    
  //   var pool_code_hash = (await poolAbi).source.hash;
  //   console.log("pool_code_hash is:",pool_code_hash);
  //   // pool_code_hash = pool_code_hash.substring(2);
  //   console.log("pool_code_hash is:",pool_code_hash);
    
  //   await factoryTx.initial(pool_code_hash);
  //   const poolCodeHash = await factoryQuery.getPoolCodeHash();
  //   console.log("poolCodeHash is:",poolCodeHash.output?.toHuman());
  //   // &mut self,fee:u32,token_a:Address,token_b:Address
  //   // var poolAddress = await factoryTx.createPool(500,token0,token1);
  //   await positionMangerContract.connect(alice);
  //   await expect(positionManagerTx.testEvent()).to.emit(positionMangerContract,"TestEvent")
  //   .withArgs(1);
  //   await positionManagerTx.createAndInitializePoolIfNecessary(token0,token1,500,1000000000000);
  //   // await expect(positionManagerTx.createAndInitializePoolIfNecessary(token0,token1,500,1000000000000))
  //   // .to.emit(factoryContract,"PoolCreated")
  //   // .withArgs(token0,token1,500,10,"0x111");

  //   // console.log("poolAddress is:",poolAddress.txHash?.toString());
  // });

  it('test position manager mint',async () =>{
    await api.isReady;
    const {defaultSigner:defaultSigner1,alice:alice1, query:factoryQuery,tx:factoryTx,contract:factoryContract,abi:factoryAbi} = await setupContract("factory","new");
    console.log("defaultSigner is:",defaultSigner1.address.toString());
    console.log("alice1 is:",alice1.address.toString());
    const { contract:weth9Contract} = await setupContract('weth9_contract','new','weth9','weth9');
    // pub fn new(factory: AccountId, weth9: AccountId,tokenDescriptor:AccountId) -> Self {
    const { contract:positionDescriptor,query:positionDescriptorQuery,tx:positionDescriptorTx} = await setupContract('NonfungibleTokenPositionDescriptor','new',weth9Contract.address,"_nativeCurrencyLabelBytes");
    // pub fn new(factory: AccountId, weth9: AccountId,tokenDescriptor:AccountId) -> Self {
    const { query:positionManagerQuery,tx:positionManagerTx,alice,defaultSigner,contract:positionMangerContract,bob } = await setupContract('NonfungiblePositionManager','new',factoryContract.address,weth9Contract.address,positionDescriptor.address,{value:1000000000});
    const { query:swapRouterQuery,tx:swapRouterTx,contract:swapRouterContract} = await setupContract('SwapRouterContract','new',factoryContract.address,weth9Contract.address,positionDescriptor.address,{value:1000000000});
    const { contract:CHECoinContract,tx:cheCoinTx} = await setupContract('stable_coin_contract','new',"CHE","CHE");
    const { contract:AAACoinContract,tx:AAACoinTx} = await setupContract('stable_coin_contract','new',"AAA","AAA");
      // &mut self,token0: AccountId,token1: AccountId,fee: u32,sqrt_price_x96: Uint160,) -> Address 
      //fee 500,3000,10000
    console.log("positionMangerContract.address is:",positionMangerContract.address.toHuman());
    
    let token0Address;
    let token1Address;
    if(CHECoinContract.address.toHuman()<AAACoinContract.address.toHuman()) {
      token0Address = CHECoinContract.address;
      token1Address = AAACoinContract.address;
    }else{
      token1Address = CHECoinContract.address;
      token0Address = AAACoinContract.address;
    }
    console.log("token0.address is:",token0Address.toHuman());
    console.log("token1.address is:",token1Address.toHuman());
    console.log("factoryContract.address is:",factoryContract.address.toHuman());
    console.log("weth9Contract.address is:",weth9Contract.address.toHuman());
    
    // factory:Address,token0: Address, token1: Address, fee: Uint24, tick_spacing: Int24
    const { abi:poolAbi} = await setupContract('pool','new',factoryContract.address,token0Address,token1Address,500,0);
    
    
    var pool_code_hash = (await poolAbi).source.hash;
    console.log("pool_code_hash is:",pool_code_hash);
    // pool_code_hash = pool_code_hash.substring(2);
    
    await factoryTx.initial(pool_code_hash);
    const poolCodeHash = await factoryQuery.getPoolCodeHash();
    console.log("poolCodeHash is:",poolCodeHash.output?.toHuman());
    // &mut self,fee:u32,token_a:Address,token_b:Address
    // var poolAddress = await factoryTx.createPool(500,token0,token1);
    await positionMangerContract.connect(alice);
    // await expect(positionManagerTx.testEvent()).to.emit(positionMangerContract,"TestEvent")
    // .withArgs(1);
    //{value:1000000000} will transfer native token to 
    console.log("-----------------------1");
    await positionManagerTx.createAndInitializePoolIfNecessary(token0Address,token1Address,500,new BN("120621891405341611593710811006"),{value:1000000000});
    console.log("-----------------------2");
    // await expect(positionManagerTx.createAndInitializePoolIfNecessary(token0Address,token1Address,500,1000000000000))
    // .to.emit(factoryContract,"PoolCreated");
    
    // console.log("mintParams:",mintParams);
    // console.log("cheCoinTx is:",cheCoinTx);
    await cheCoinTx.mint(alice.address,1000000);
    console.log("-----------------------3");
    await AAACoinTx.mint(alice.address,1000000);
    console.log("-----------------------4");
    await cheCoinTx.approve(positionMangerContract.address,1000000);
    console.log("-----------------------5");
    await AAACoinTx.approve(positionMangerContract.address,1000000);
    console.log("-----------------------6");
    await positionManagerTx.mint(token0Address,token1Address,500,100,10000,1000,1000,10,0,alice.address,10);
    let poolAddress;
    console.log("-----------------------1");
    if(token0Address.toHuman()<weth9Contract.address.toHuman()) {
      await positionManagerTx.createAndInitializePoolIfNecessary(token0Address,weth9Contract.address,500,new BN("120621891405341611593710811006"),{value:1000000000});
      console.log("-----------------------2");
      await positionManagerTx.mint(token0Address,weth9Contract.address,500,200,10000,1000,1000,10,10,alice.address,10,{value:100000000000});
      console.log("-----------------------3");
      poolAddress = await factoryQuery.getPool(500,token0Address,weth9Contract.address);
      console.log("-----------------------4");
      console.log("poolAddress is:",poolAddress.output?.toHuman());
    }else{
      await positionManagerTx.createAndInitializePoolIfNecessary(weth9Contract.address,token0Address,500,new BN("120621891405341611593710811006"),{value:1000000000});
      console.log("-----------------------5");
      await positionManagerTx.mint(weth9Contract.address,token0Address,500,200,10000,1000,1000,10,10,alice.address,10,{value:100000000000});
      console.log("-----------------------6");
      poolAddress = await factoryQuery.getPool(500,weth9Contract.address,token0Address);
      console.log("-----------------------7");
      console.log("poolAddress is:",poolAddress.output?.toHuman());
    }




    
    
    
    let uri = await positionDescriptorQuery.tokenURI(positionMangerContract.address,1);
    console.log("-----------------------uri is",uri.output?.toHuman());



    
    // await expect(positionManagerQuery.positions(1)).to.
    let tokenId = { "u128": 1 };
    let position1 = await positionManagerQuery.positions(tokenId);
    console.log("-----------------------9");
    console.log("position1 is:",position1.output?.[2].toHuman());
    // interface IncreaseLiquidityParams{
    //   tokenId:Number,amount0Desired:Number,amount1Desired:Number,amount0Min:Number,amount1Min:Number,deadline:Number,
    // };
    // let increaseParam:IncreaseLiquidityParams={
    //   tokenId:1,amount0Desired:100,amount1Desired:100,amount0Min:10,amount1Min:10,deadline:111111,
    // };
    await positionManagerTx.increaseLiquidity(tokenId,100,100,1,1,9652429262733);

    await cheCoinTx.approve(swapRouterContract.address,1000000);
    await AAACoinTx.approve(swapRouterContract.address,1000000);
    // &mut self, tokenIn: Address,tokenOut: Address,fee: Uint24,recipient: Address,deadline: U256,amountIn: U256,amountOutMinimum: U256,sqrtPriceLimitX96: U160,
    await swapRouterTx.exactInputSingle(token0Address,token1Address,500,alice.address,9652429262733,50,10,new BN("12062189140534161159371081100"));
    // &mut self,tokenIn: Address,tokenOut: Address,fee: Uint24,recipient: Address,deadline: u64,amountOut: U256,amountInMaximum: U256,sqrtPriceLimitX96: U160,
    await swapRouterTx.exactOutputSingle(token1Address,token0Address,500,alice.address,9652429262733,50,1000,new BN("120621891405341611593710891006"));
    console.log("-----------------------10");

    // tokenId: u128,liquidity: u128,amount0Min: U256,amount1Min: U256,deadline: u64,
    // await positionManagerTx.setFactory(factoryContract.address);
    await positionManagerTx.decreaseLiquidity(tokenId,2125,6,52,9652429262733);
    console.log("-----------------------11");
    try{
      // tokenId: u128,recipient: Address,amount0Max: u128,amount1Max: u128,
      // TODO add collect method
      await positionManagerTx.collect(tokenId,alice.address,2000,2000);
    }catch(e){
      console.log(e);
    }
    console.log("after collect!");
    await positionManagerTx.getApproved(tokenId);
    console.log("-----------------------12");
    await positionManagerTx.GetAndIncrementNonce(tokenId);
    console.log("-----------------------13");
    await positionManagerTx.permit(bob.address,tokenId,9652429262733,0,"0","0");
    console.log("-----------------------14");
    await positionManagerTx.burn(tokenId);
    console.log("-----------------------15");
    // await expect(positionManagerTx.createAndInitializePoolIfNecessary(token0,token1,500,1000000000000))
    // .to.emit(factoryContract,"PoolCreated")
    // .withArgs(token0,token1,500,10,"0x111");

    // console.log("poolAddress is:",poolAddress.txHash?.toString());

    
  });

});