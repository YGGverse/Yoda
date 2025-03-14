use crate::profile::Profile;
use std::sync::Arc;

pub fn new_for_profile_identity_id(
    profile: &Arc<Profile>,
    profile_identity_id: i64,
    auth_url: &str,
) -> bool {
    profile
        .identity
        .auth
        .is_matches(auth_url, profile_identity_id) // @TODO direct call?
}
