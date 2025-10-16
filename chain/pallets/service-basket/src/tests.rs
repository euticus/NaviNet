use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::FixedU128;

#[test]
fn set_weights_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Set weights for a network (sum <= 1.0)
        let weights = vec![
            (b"ID".to_vec(), 300_000),    // 30%
            (b"FILE".to_vec(), 500_000),  // 50%
            (b"GOLEM".to_vec(), 200_000), // 20%
        ];

        assert_ok!(ServiceBasket::set_weights(
            RuntimeOrigin::signed(1),
            1,
            weights
        ));

        // Verify basket was created
        let basket = crate::Baskets::<Test>::get(1).unwrap();
        assert_eq!(basket.weights.len(), 3);

        System::assert_last_event(Event::WeightsUpdated { network_id: 1 }.into());
    });
}

#[test]
fn set_weights_fails_when_sum_exceeds_one() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Set weights that sum to > 1.0
        let weights = vec![
            (b"ID".to_vec(), 600_000),   // 60%
            (b"FILE".to_vec(), 500_000), // 50%
        ];

        assert_noop!(
            ServiceBasket::set_weights(RuntimeOrigin::signed(1), 1, weights),
            Error::<Test>::WeightSumExceedsOne
        );
    });
}

#[test]
fn add_proof_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // First set weights to create basket
        let weights = vec![(b"ID".to_vec(), 500_000)];
        assert_ok!(ServiceBasket::set_weights(
            RuntimeOrigin::signed(1),
            1,
            weights
        ));

        // Add proof
        assert_ok!(ServiceBasket::add_proof(
            RuntimeOrigin::signed(1),
            1,
            b"QmTest123".to_vec()
        ));

        // Verify proof was added
        let basket = crate::Baskets::<Test>::get(1).unwrap();
        assert_eq!(basket.proofs.len(), 1);

        System::assert_last_event(Event::ProofAdded { network_id: 1 }.into());
    });
}

#[test]
fn add_proof_fails_when_basket_not_found() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_noop!(
            ServiceBasket::add_proof(RuntimeOrigin::signed(1), 999, b"QmTest".to_vec()),
            Error::<Test>::BasketNotFound
        );
    });
}

#[test]
fn recompute_index_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Set weights
        let weights = vec![
            (b"ID".to_vec(), 300_000),   // 30%
            (b"FILE".to_vec(), 500_000), // 50%
        ];
        assert_ok!(ServiceBasket::set_weights(
            RuntimeOrigin::signed(1),
            1,
            weights
        ));

        // Recompute index
        assert_ok!(ServiceBasket::recompute_index(RuntimeOrigin::signed(1), 1));

        // Verify index was computed (sum of weights = 0.8)
        let basket = crate::Baskets::<Test>::get(1).unwrap();
        let expected_index = FixedU128::from_rational(800_000u128, 1_000_000);
        assert_eq!(basket.index, expected_index);
    });
}

#[test]
fn recompute_index_fails_when_basket_not_found() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_noop!(
            ServiceBasket::recompute_index(RuntimeOrigin::signed(1), 999),
            Error::<Test>::BasketNotFound
        );
    });
}
