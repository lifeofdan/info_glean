mod mig_1717224385_init;
/**
 * The following function is automatically modified
 * do not manually edit it
 */
pub(crate) fn setup() -> Vec<Box<dyn dirtybase_contract::db::migration::Migration>> {
    vec![
        Box::new(mig_1717224385_init::Mig1717224385Init),
        // dty_inject
    ]
}
