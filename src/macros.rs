macro_rules! instance {
    ($instance_id:expr) => {
        crate::ENGINES.get_or_init(Default::default).entry($instance_id).or_default()
    }
}

pub(crate) use instance;
