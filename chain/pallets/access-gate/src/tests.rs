use crate::{mock::*, Event};
use frame_support::assert_ok;

#[test]
fn grant_access_works() {
    new_test_ext().execute_with(|| {
        // Initialize block number to 1 so events are registered
        System::set_block_number(1);

        // Grant access to a resource
        assert_ok!(AccessGate::grant_access(RuntimeOrigin::signed(1), 42));

        // Check that the event was emitted
        System::assert_last_event(
            Event::AccessGranted {
                who: 1,
                resource_id: 42,
            }
            .into(),
        );
    });
}
