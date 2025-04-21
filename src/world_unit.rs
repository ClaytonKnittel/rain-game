use std::ops::{Add, Div, Mul, Neg, Sub};

use bevy::math::Vec2;

use crate::win_info::WinInfo;

#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct WorldUnit(f32);

impl WorldUnit {
  const SCREEN_ASPECT_RATIO: f32 = 720. / 1280.;

  const UNITS_PER_SCREEN_WIDTH: f32 = 50.;
  const UNITS_PER_SCREEN_HEIGHT: f32 = Self::UNITS_PER_SCREEN_WIDTH * Self::SCREEN_ASPECT_RATIO;

  pub const TOP: Self = Self::normalized_y(1.);
  pub const BOTTOM: Self = Self::normalized_y(-1.);
  pub const LEFT: Self = Self::normalized_x(-1.);
  pub const RIGHT: Self = Self::normalized_x(1.);

  pub const fn new(units: f32) -> Self {
    Self(units)
  }

  pub const fn normalized_x(x: f32) -> Self {
    debug_assert!(-1. <= x && x <= 1.);
    Self(x * Self::UNITS_PER_SCREEN_WIDTH / 2.)
  }

  pub const fn normalized_y(y: f32) -> Self {
    debug_assert!(-1. <= y && y <= 1.);
    Self(y * Self::UNITS_PER_SCREEN_HEIGHT / 2.)
  }

  fn scale(win_info: &WinInfo) -> Vec2 {
    let window_width = win_info
      .width
      .min(win_info.height / Self::SCREEN_ASPECT_RATIO);
    let window_height = window_width * Self::SCREEN_ASPECT_RATIO;
    Vec2 {
      x: window_width / Self::UNITS_PER_SCREEN_WIDTH,
      y: window_height / Self::UNITS_PER_SCREEN_HEIGHT,
    }
  }

  pub fn to_x(self, win_info: &WinInfo) -> f32 {
    self.0 * Self::scale(win_info).x
  }

  pub fn to_y(self, win_info: &WinInfo) -> f32 {
    self.0 * Self::scale(win_info).y
  }
}

impl Add<Self> for WorldUnit {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    Self(self.0 + rhs.0)
  }
}

impl Sub<Self> for WorldUnit {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self {
    Self(self.0 - rhs.0)
  }
}

impl Mul<f32> for WorldUnit {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self {
    Self(self.0 * rhs)
  }
}

impl Div<f32> for WorldUnit {
  type Output = Self;

  fn div(self, rhs: f32) -> Self {
    Self(self.0 / rhs)
  }
}

impl Neg for WorldUnit {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Self(-self.0)
  }
}

#[derive(Clone, Copy, Default)]
pub struct WorldVec2 {
  pub x: WorldUnit,
  pub y: WorldUnit,
}

impl WorldVec2 {
  pub const fn normalized(x: f32, y: f32) -> Self {
    Self {
      x: WorldUnit::normalized_x(x),
      y: WorldUnit::normalized_y(y),
    }
  }

  pub fn to_absolute(self, win_info: &WinInfo) -> Vec2 {
    Vec2 { x: self.x.0, y: self.y.0 } * WorldUnit::scale(win_info)
  }
}
