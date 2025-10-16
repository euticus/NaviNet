use crate::{mock::*, CoinKind, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_network_with_navi_works() {
    new_test_ext().execute_with(|| {
        // Initialize block number to 1 so events are registered
        System::set_block_number(1);

        // Create a network using NAVI
        assert_ok!(NetworkFactory::create_network(
            RuntimeOrigin::signed(1),
            true, // use_navi
            b"ipfs://QmTest".to_vec()
        ));

        // Check that the event was emitted
        System::assert_last_event(
            Event::NetworkCreated {
                network_id: 0,
                owner: 1,
                use_navi: true,
                asset_id: None,
            }
            .into(),
        );

        // Check storage
        let network = NetworkFactory::networks(0).unwrap();
        assert_eq!(network.owner, 1);
        assert_eq!(network.coin_kind, CoinKind::UseNavi);
        assert_eq!(network.asset_id, None);
        assert_eq!(network.metadata_uri.to_vec(), b"ipfs://QmTest".to_vec());

        // Check NextNetworkId incremented
        assert_eq!(NetworkFactory::next_network_id(), 1);
    });
}

#[test]
fn create_network_with_custom_asset_works() {
    new_test_ext().execute_with(|| {
        // Initialize block number to 1 so events are registered
        System::set_block_number(1);

        // Create a network with custom asset
        assert_ok!(NetworkFactory::create_network(
            RuntimeOrigin::signed(2),
            false, // use_navi = false, so mint asset
            b"ipfs://QmCustom".to_vec()
        ));

        // Check that the event was emitted
        let network_id = 0;
        let expected_asset_id = network_id as u32;
        System::assert_last_event(
            Event::NetworkCreated {
                network_id,
                owner: 2,
                use_navi: false,
                asset_id: Some(expected_asset_id),
            }
            .into(),
        );

        // Check storage
        let network = NetworkFactory::networks(0).unwrap();
        assert_eq!(network.owner, 2);
        assert_eq!(network.coin_kind, CoinKind::MintAsset);
        assert_eq!(network.asset_id, Some(expected_asset_id));
    });
}

#[test]
fn create_multiple_networks_increments_id() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Create first network
        assert_ok!(NetworkFactory::create_network(
            RuntimeOrigin::signed(1),
            true,
            b"network1".to_vec()
        ));
        assert_eq!(NetworkFactory::next_network_id(), 1);

        // Create second network
        assert_ok!(NetworkFactory::create_network(
            RuntimeOrigin::signed(2),
            false,
            b"network2".to_vec()
        ));
        assert_eq!(NetworkFactory::next_network_id(), 2);

        // Verify both networks exist
        assert!(NetworkFactory::networks(0).is_some());
        assert!(NetworkFactory::networks(1).is_some());
    });
}

#[test]
fn metadata_uri_too_long_fails() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Create metadata URI that's too long (> 256 bytes)
        let long_uri = vec![b'x'; 257];

        // Should fail with MetadataUriTooLong error
        assert_noop!(
            NetworkFactory::create_network(RuntimeOrigin::signed(1), true, long_uri),
            crate::Error::<Test>::MetadataUriTooLong
        );
    });
}

#[test]
fn treasury_account_is_derived_correctly() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Create a network
        assert_ok!(NetworkFactory::create_network(
            RuntimeOrigin::signed(1),
            true,
            b"test".to_vec()
        ));

        // Get the network and check treasury is derived correctly
        let network = NetworkFactory::networks(0).unwrap();
        let expected_treasury = NetworkFactory::treasury_account(0);
        assert_eq!(network.treasury, expected_treasury);
    });
}
