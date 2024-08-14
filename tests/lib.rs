use scrypto_test::prelude::*;
use nft_minter::{nft_minter_test::*, types::ImageNft};
use std::fs;
use rand::prelude::*;

#[test]
fn mint_nft() -> Result<(), RuntimeError> {
    let mut env = TestEnvironment::new();
    let package_address =
        PackageFactory::compile_and_publish(this_package!(), &mut env, CompileProfile::Fast)?;
    
    let mut nft_minter = NftMinter::instantiate(package_address, &mut env)?;

    env.disable_auth_module();
    env.disable_limits_module();

    // Create images directory if necessary
    let _ = fs::create_dir_all("test_images");

    for i in 1..50 {
        // let random_string: String = rand::thread_rng()
        //     .sample_iter::<char, _>(Standard)
        //     .take(64)
        //     .collect();
        // println!("{random_string}");
        let mut data = [0u8; 128];
        rand::thread_rng().fill_bytes(&mut data);
        // let seed = random_string.into_bytes();
        let nft_bucket = nft_minter.mint_nft(data.to_vec(), &mut env)?;

        let resource_manager = ResourceManager(nft_bucket.resource_address(&mut env)?);
        let nft_data = resource_manager.get_non_fungible_data::<_, _, ImageNft>(
            nft_bucket
                .non_fungible_local_ids(&mut env)?
                .first()
                .unwrap()
                .clone(),
            &mut env,
        )?;

        fs::write(format!("test_images/{i}.svg"), nft_data.svg_data.as_bytes()).expect("Failed to write SVG file.");
    }

    Ok(())
}