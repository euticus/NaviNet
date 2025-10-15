use crate::{mock::*, Event};
use frame_support::assert_ok;

#[test]
fn update_weights_works() {
    new_test_ext().execute_with(|| {
        // Initialize block number to 1 so events are registered
        System::set_block_number(1);

        // Update weights for a network
        assert_ok!(ServiceBasket::update_weights(RuntimeOrigin::signed(1), 1));

        // Check that the event was emitted
        System::assert_last_event(Event::WeightsUpdated { network_id: 1 }.into());
    });
}
