use std::env;
use std::process;
use std::sync::Arc;

use bitcoin::consensus::deserialize;
use bitcoinkernel::{
    BlockManagerOptions, ChainType, ChainstateLoadOptions, ChainstateManager,
    ChainstateManagerOptions, Context, ContextBuilder,
};

fn create_context() -> Arc<Context> {
    Arc::new(
        ContextBuilder::new()
            .chain_type(ChainType::MAINNET)
            .build()
            .unwrap(),
    )
}

fn scan_blocks(chainman: &ChainstateManager, start_height: i32, end_height:i32) {
    let mut block_index = chainman.get_block_index_by_height(end_height);
    loop {
        if block_index.as_ref().unwrap().height() < start_height  {
            break;
        }
        let raw_block: Vec<u8> = chainman
            .read_block_data(block_index.as_ref().unwrap())
            .unwrap()
            .into();

        let _block: bitcoin::Block = deserialize(&raw_block).unwrap();
        block_index = match block_index.unwrap().prev() {
            Ok(block_index_prev) => Ok(block_index_prev),
            Err(_) => break,
        };

    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} you must provide data dir, start and end height", args[0]);
        process::exit(1);
    }

    let context = create_context();
    let data_dir = args[1].clone();
    let start_height: i32 = args[2].parse().expect("Invalid start height");
    let end_height: i32 = args[3].parse().expect("Invalid end height");
    let blocks_dir = data_dir.clone() + "/blocks";
    let chainman = ChainstateManager::new(
        ChainstateManagerOptions::new(&context, &data_dir).unwrap(),
        BlockManagerOptions::new(&context, &blocks_dir).unwrap(),
        ChainstateLoadOptions::new(),
        Arc::clone(&context),
    )
    .unwrap();
    chainman.import_blocks().unwrap();
    scan_blocks(&chainman, start_height, end_height);
}
