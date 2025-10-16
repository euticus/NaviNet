use crate::{mock::*, Event};
use frame_support::assert_ok;

#[test]
fn register_identity_works() {
    new_test_ext().execute_with(|| {
        // Initialize block number to 1 so events are registered
        System::set_block_number(1);

        // Register an identity
        assert_ok!(Identity::register_identity(RuntimeOrigin::signed(1)));

        // Check that the event was emitted
        System::assert_last_event(Event::IdentityRegistered { who: 1 }.into());
    });
}
