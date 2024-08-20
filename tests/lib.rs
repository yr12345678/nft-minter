use nft_minter::{nft_minter_test::*, types::NFTImage};
use radix_common::network::NetworkDefinition;
use rand::prelude::*;
use scrypto_test::prelude::*;
use scrypto_test::utils::dump_manifest_to_file_system;
use std::fs;

#[test]
fn cannot_minft_with_same_seed() -> Result<(), RuntimeError> {
    // Arrange
    let mut env = TestEnvironment::new();
    let package_address =
        PackageFactory::compile_and_publish(this_package!(), &mut env, CompileProfile::Fast)?;

    let mut nft_minter = NftMinter::instantiate(package_address, &mut env)?;

    env.disable_auth_module();

    let mut data = [0u8; 128];
    rand::thread_rng().fill_bytes(&mut data);

    // Act
    let _first_mint = nft_minter.mint_nft(data.to_vec(), &mut env)?;
    let second_mint = nft_minter.mint_nft(data.to_vec(), &mut env);

    // Assert
    assert!(second_mint.is_err());

    Ok(())
}

#[test]
fn cannot_minft_with_wrong_seed_length() -> Result<(), RuntimeError> {
    // Arrange
    let mut env = TestEnvironment::new();
    let package_address =
        PackageFactory::compile_and_publish(this_package!(), &mut env, CompileProfile::Fast)?;

    let mut nft_minter = NftMinter::instantiate(package_address, &mut env)?;

    env.disable_auth_module();

    let mut data = [0u8; 3];
    rand::thread_rng().fill_bytes(&mut data);

    // Act
    let result = nft_minter.mint_nft(data.to_vec(), &mut env);

    // Assert
    assert!(result.is_err());

    Ok(())
}

// Mint a load of NFTs for test/review purposes
#[test]
fn mint_nft_batch() -> Result<(), RuntimeError> {
    let mut env = TestEnvironment::new();
    let package_address =
        PackageFactory::compile_and_publish(this_package!(), &mut env, CompileProfile::Fast)?;

    let mut nft_minter = NftMinter::instantiate(package_address, &mut env)?;

    env.disable_auth_module();
    env.disable_limits_module();

    // Create images directory if necessary
    let _ = fs::create_dir_all("test_images");

    for i in 1..1001 {
        let mut data = [0u8; 128];
        rand::thread_rng().fill_bytes(&mut data);
        let nft_bucket = nft_minter.mint_nft(data.to_vec(), &mut env)?;

        let resource_manager = ResourceManager(nft_bucket.resource_address(&mut env)?);
        let nft_data = resource_manager.get_non_fungible_data::<_, _, NFTImage>(
            nft_bucket
                .non_fungible_local_ids(&mut env)?
                .first()
                .unwrap()
                .clone(),
            &mut env,
        )?;

        println!("{:#?}", nft_data);

        fs::write(
            format!("test_images/{i}.svg"),
            hex::decode(nft_data.svg_data).unwrap(),
        )
        .expect("Failed to write SVG file.");
    }

    Ok(())
}

// Build a manifest that mints 10 NFTs for testing on Stonknet
#[test]
fn build_mint_manifest() -> Result<(), RuntimeError> {
    let mut manifest = ManifestBuilder::new();

    for _ in 0..10 {
        let mut data = [0u8; 128];
        rand::thread_rng().fill_bytes(&mut data);

        let component_address = GlobalAddress::try_from_bech32(
            &AddressBech32Decoder::new(&NetworkDefinition::stokenet()),
            "component_tdx_2_1cz7tlcgchcmruknrwq5t42atcvkdgyfjm7w40vn24xn22j2e2w7sz7",
        )
        .unwrap();

        manifest =
            manifest.call_method(component_address, "mint_nft", manifest_args!(data.to_vec()));
    }

    manifest = manifest.deposit_batch(
        GlobalAddress::try_from_bech32(
            &AddressBech32Decoder::new(&NetworkDefinition::stokenet()),
            "account_tdx_2_1292jyxrlexx6m877v038jmyjs0cna83l3suppctuy257x5a4unjqds",
        )
        .unwrap(),
    );

    let result = dump_manifest_to_file_system(
        manifest.object_names(),
        &manifest.build(),
        "./manifests",
        Some("mint_manifest"),
        &NetworkDefinition::stokenet(),
    );

    println!("{:?}", result);

    Ok(())
}
