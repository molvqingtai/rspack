use swc_core::common::Span;

pub trait SpanExt {
  fn real_lo(&self) -> u32;

  fn real_hi(&self) -> u32;
}

impl SpanExt for Span {
  fn real_lo(&self) -> u32 {
    self.lo().0 - 1
  }

  fn real_hi(&self) -> u32 {
    self.hi().0 - 1
  }
}
