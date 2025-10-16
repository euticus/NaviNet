use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn register_resource_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Create a network first (will have network_id = 0)
        assert_ok!(NetworkFactory::create_network(
            RuntimeOrigin::signed(1),
            true,
            vec![1, 2, 3]
        ));

        assert_ok!(AccessGate::register_resource(
            RuntimeOrigin::signed(1),
            0, // network_id
            vec![1, 2, 3],
            1,
            1000, // base_stake
            100,  // duration_blocks
            None  // ppu
        ));

        System::assert_last_event(
            Event::ResourceRegistered {
                network_id: 0,
                resource_id: 0,
            }
            .into(),
        );
    });
}

#[test]
fn stake_for_access_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Create a network (will have network_id = 0)
        assert_ok!(NetworkFactory::create_network(
            RuntimeOrigin::signed(1),
            true,
            vec![1, 2, 3]
        ));

        // Register a resource (will have resource_id = 0)
        assert_ok!(AccessGate::register_resource(
            RuntimeOrigin::signed(1),
            0, // network_id
            vec![1, 2, 3],
            1,
            1000, // base_stake
            100,  // duration_blocks
            None  // ppu
        ));

        // Stake for access
        assert_ok!(AccessGate::stake_for_access(
            RuntimeOrigin::signed(2),
            0, // network_id
            0, // resource_id
            None
        ));

        // Verify membership was created
        let membership = crate::Memberships::<Test>::get(0, 2).unwrap();
        assert_eq!(membership.network_id, 0);
        assert_eq!(membership.who, 2);
        assert_eq!(membership.tier_idx, None);
    });
}

#[test]
fn unstake_if_expired_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Create a network (will have network_id = 0)
        assert_ok!(NetworkFactory::create_network(
            RuntimeOrigin::signed(1),
            true,
            vec![1, 2, 3]
        ));

        // Stake for access
        assert_ok!(AccessGate::stake_for_access(
            RuntimeOrigin::signed(2),
            0, // network_id
            0, // resource_id
            None
        ));

        // Try to unstake before expiry - should fail
        assert_noop!(
            AccessGate::unstake_if_expired(RuntimeOrigin::signed(3), 0, 2),
            Error::<Test>::MembershipNotExpired
        );

        // Advance block number past expiry
        System::set_block_number(200);

        // Now unstake should work
        assert_ok!(AccessGate::unstake_if_expired(
            RuntimeOrigin::signed(3),
            0, // network_id
            2
        ));

        // Verify membership was removed
        assert!(crate::Memberships::<Test>::get(0, 2).is_none());
    });
}

#[test]
fn pay_per_use_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Create a network (will have network_id = 0)
        assert_ok!(NetworkFactory::create_network(
            RuntimeOrigin::signed(1),
            true,
            vec![1, 2, 3]
        ));

        // Register a resource with PPU enabled (will have resource_id = 0)
        assert_ok!(AccessGate::register_resource(
            RuntimeOrigin::signed(1),
            0, // network_id
            vec![1, 2, 3],
            1,
            1000,     // base_stake
            100,      // duration_blocks
            Some(50)  // ppu
        ));

        // Pay per use
        assert_ok!(AccessGate::pay_per_use(RuntimeOrigin::signed(2), 0, 0));

        System::assert_last_event(
            Event::PayPerUsePaid {
                network_id: 0,
                who: 2,
                resource_id: 0,
                amount: 50,
            }
            .into(),
        );
    });
}

#[test]
fn pay_per_use_fails_when_not_enabled() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Create a network (will have network_id = 0)
        assert_ok!(NetworkFactory::create_network(
            RuntimeOrigin::signed(1),
            true,
            vec![1, 2, 3]
        ));

        // Register a resource without PPU (will have resource_id = 0)
        assert_ok!(AccessGate::register_resource(
            RuntimeOrigin::signed(1),
            0, // network_id
            vec![1, 2, 3],
            1,
            1000, // base_stake
            100,  // duration_blocks
            None  // ppu
        ));

        // Pay per use should fail
        assert_noop!(
            AccessGate::pay_per_use(RuntimeOrigin::signed(2), 0, 0),
            Error::<Test>::PayPerUseNotEnabled
        );
    });
}

#[test]
fn register_resource_basic_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Create a network (will have network_id = 0)
        assert_ok!(NetworkFactory::create_network(
            RuntimeOrigin::signed(1),
            true,
            vec![1, 2, 3]
        ));

        assert_ok!(AccessGate::register_resource(
            RuntimeOrigin::signed(1),
            0, // network_id
            vec![1, 2, 3],
            1,
            1000,     // base_stake
            100,      // duration_blocks
            Some(25)  // ppu
        ));

        // Verify resource was stored
        let resource = crate::Resources::<Test>::get(0, 0).unwrap();
        assert_eq!(resource.pricing.base_stake, 1000);
        assert_eq!(resource.pricing.duration_blocks, 100);
        assert_eq!(resource.pricing.ppu, Some(25));
    });
}
