type TimestampMillis = u64;

pub trait Environment {
    fn now(&self) -> TimestampMillis;
}

pub struct CanisterEnv {}

impl CanisterEnv {
  pub fn new() -> Self {
    CanisterEnv {}
  }
}

impl Environment for CanisterEnv {
    fn now(&self) -> TimestampMillis {
        ic_cdk::api::time()
    }
}

pub struct EmptyEnv {}

impl Environment for EmptyEnv {
  fn now(&self) -> TimestampMillis {
      0
  }
}

pub struct TestEnv {
    pub now: u64,
}

impl Environment for TestEnv {
    fn now(&self) -> u64 {
        self.now
    }
}
