use crate::{mock::*, Event};
use frame_support::assert_ok;

#[test]
fn create_network_works() {
    new_test_ext().execute_with(|| {
        // Initialize block number to 1 so events are registered
        System::set_block_number(1);

        // Create a network
        assert_ok!(NetworkFactory::create_network(RuntimeOrigin::signed(1)));

        // Check that the event was emitted
        System::assert_last_event(
            Event::NetworkCreated {
                network_id: 1,
                owner: 1,
            }
            .into(),
        );
    });
}
