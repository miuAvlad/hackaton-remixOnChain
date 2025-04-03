#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

mod erc721;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::{U256,Address},storage::StorageU256,msg, prelude::*, stylus_proc::entrypoint, ArbResult};
use core::f32::consts::PI;

use crate::erc721::{Erc721, Erc721Params, Erc721Error};
use sha2::{Sha256, Digest};


fn sha256_hash(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}
struct StylusNFTParams;
impl Erc721Params for StylusNFTParams {
    const NAME: &'static str = "AudioNFT";
    const SYMBOL: &'static str = "ANFT";

    fn token_uri(token_id: U256) -> String {
        format!("{}{}{}", "https://my-nft-metadata.com/", token_id, ".json")
    }
}
pub enum MyError {
    Unknown,
    
}

// Define persistent storage using the Solidity ABI.
// `RemixOnChain` va fi entrypoint-ul contractului.
sol_storage! {
    #[entrypoint]
    pub struct RemixOnChain {
        uint256 totalMinted;
        #[borrow]
        Erc721<StylusNFTParams> erc721;
    }
}

#[public]
#[inherit(Erc721<StylusNFTParams>)]
impl RemixOnChain {
    pub fn generate_wave(&self, seed: u64) -> ArbResult {
        Ok(generate_wave_impl(seed).into_bytes())
    } 
    pub fn mint_if_valid(&mut self, seed: u64) -> ArbResult {
     
        let cid_string: String = generate_wave_impl(seed);

        let cid_bytes = cid_string.as_bytes();
        

        let mut hasher = Sha256::new();
        hasher.update(cid_bytes);
        let result = hasher.finalize();
        let mut cid_hash = [0u8; 32];
        cid_hash.copy_from_slice(&result);
        
     


        // how do events work?

        // evm::log(Log {
        //     sender: Address::from([0x11; 20]),
        //     message: "Hello world!".to_string(),
        // });
    
        // Returnăm hash-ul CID-ului ca Vec<u8>
        Ok(cid_hash.to_vec())
    }

    /// Burns an NFT
    pub fn burn(&mut self, token_id: U256) -> Result<(), Erc721Error> {
        // This function checks that msg::sender() owns the specified token_id
        self.erc721.burn(msg::sender(), token_id)?;
        Ok(())
    }

    /// Total supply
    pub fn total_supply(&mut self) -> Result<U256, Erc721Error> {
        Ok(self.erc721.total_supply.get())
    }
}

fn generate_wave_impl(seed: u64) -> String {
    let sample_rate = 44100.0;
    let duration_secs = 2.0;
    let total_samples = (sample_rate * duration_secs) as usize;
    let mut samples: Vec<f32> = Vec::with_capacity(total_samples);

    let seed_bytes = seed.to_le_bytes();

   
    let val1 = u16::from_le_bytes([seed_bytes[0], seed_bytes[1]]);
    let val2 = u16::from_le_bytes([seed_bytes[2], seed_bytes[3]]);
    let val3 = u16::from_le_bytes([seed_bytes[4], seed_bytes[5]]);
    let f1 = 100.0 + (val1 as f32 % 901.0);
    let f2 = 100.0 + (val2 as f32 % 901.0);
    let f3 = 100.0 + (val3 as f32 % 901.0);


    let mut phase1 = (seed_bytes[6] as f32) / 255.0;
    let mut phase2 = (seed_bytes[7] as f32) / 255.0;
    let mut phase3 = 0.49;

    let phase_inc1 = f1 / sample_rate;
    let phase_inc2 = f2 / sample_rate;
    let phase_inc3 = f3 / sample_rate;

    for _ in 0..total_samples {
        let sin_val = (2.0 * core::f32::consts::PI * phase1).sin();
        let saw_val = 2.0 * phase2 - 1.0;
        let square_val = if phase3 < 0.5 { 1.0 } else { -1.0 };
        let sample = (sin_val + saw_val + square_val) / 3.0;
        samples.push(sample);
        phase1 = (phase1 + phase_inc1) % 1.0;
        phase2 = (phase2 + phase_inc2) % 1.0;
        phase3 = (phase3 + phase_inc3) % 1.0;
    }

    
    let mut encoded = Vec::with_capacity(samples.len() * 4);
    for sample in samples {
        encoded.extend_from_slice(&sample.to_le_bytes());
    }

  
    let hash = sha256_hash(&encoded);
    
    let cid = hex::encode(hash);
    cid
}

#[cfg(test)]
mod tests {
    use super::*;

   
    #[test]
    fn test_generate_wave_different_seeds() {
        
        let encoded_waveform1 = generate_wave_impl(42);
        let encoded_waveform2 = generate_wave_impl(43);


        assert_ne!(
            encoded_waveform1,
            encoded_waveform2,
            "Waveform-urile generate pentru seed-uri diferite ar trebui să fie diferite."
        );
    }
}
