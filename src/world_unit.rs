use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

use bevy::math::{primitives::Rectangle, Vec2};
use ordered_float::NotNan;

use crate::win_info::WinInfo;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct WorldUnit(f32);

impl Eq for WorldUnit {}

impl PartialOrd for WorldUnit {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for WorldUnit {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    NotNan::new(self.0)
      .unwrap()
      .cmp(&NotNan::new(other.0).unwrap())
  }
}

impl WorldUnit {
  const SCREEN_ASPECT_RATIO: f32 = 720. / 1280.;

  const UNITS_PER_SCREEN_WIDTH: f32 = 50.;
  const UNITS_PER_SCREEN_HEIGHT: f32 = Self::UNITS_PER_SCREEN_WIDTH * Self::SCREEN_ASPECT_RATIO;

  pub const ZERO: Self = Self(0.);
  pub const ONE: Self = Self(1.);

  pub const TOP: Self = Self::normalized_y(1.);
  pub const BOTTOM: Self = Self::normalized_y(-1.);
  pub const LEFT: Self = Self::normalized_x(-1.);
  pub const RIGHT: Self = Self::normalized_x(1.);

  pub const SCREEN_WIDTH: Self = Self(Self::UNITS_PER_SCREEN_WIDTH);

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

  pub const fn to_untyped(self) -> f32 {
    self.0
  }

  pub const fn abs(self) -> Self {
    Self(self.0.abs())
  }

  pub const fn squared(self) -> f32 {
    self.0 * self.0
  }
}

impl Add for WorldUnit {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    Self(self.0 + rhs.0)
  }
}

impl AddAssign for WorldUnit {
  fn add_assign(&mut self, rhs: Self) {
    self.0 += rhs.0;
  }
}

impl Sub for WorldUnit {
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

impl Mul<Vec2> for WorldUnit {
  type Output = WorldVec2;

  fn mul(self, rhs: Vec2) -> Self::Output {
    WorldVec2 { x: self * rhs.x, y: self * rhs.y }
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
  pub const ZERO: WorldVec2 = Self { x: WorldUnit::ZERO, y: WorldUnit::ZERO };

  pub const X: WorldVec2 = Self { x: WorldUnit::ONE, y: WorldUnit::ZERO };
  pub const Y: WorldVec2 = Self { x: WorldUnit::ZERO, y: WorldUnit::ONE };

  pub const fn new(x: WorldUnit, y: WorldUnit) -> Self {
    Self { x, y }
  }

  pub const fn new_normalized(x: f32, y: f32) -> Self {
    Self {
      x: WorldUnit::normalized_x(x),
      y: WorldUnit::normalized_y(y),
    }
  }

  const fn from_untyped(vec: Vec2) -> Self {
    Self { x: WorldUnit(vec.x), y: WorldUnit(vec.y) }
  }

  const fn to_untyped(self) -> Vec2 {
    Vec2 {
      x: self.x.to_untyped(),
      y: self.y.to_untyped(),
    }
  }

  pub fn to_absolute(self, win_info: &WinInfo) -> Vec2 {
    Vec2 { x: self.x.0, y: self.y.0 } * WorldUnit::scale(win_info)
  }

  pub fn length_squared(self) -> f32 {
    self.x.0 * self.x.0 + self.y.0 * self.y.0
  }

  pub fn length(self) -> WorldUnit {
    WorldUnit(self.length_squared().sqrt())
  }

  pub fn normalized(self) -> Self {
    let length = self.length().0;
    Self {
      x: WorldUnit(self.x.0 / length),
      y: WorldUnit(self.y.0 / length),
    }
  }

  pub fn try_normalize(self) -> Option<Self> {
    self.to_untyped().try_normalize().map(Self::from_untyped)
  }

  pub fn dot(self, other: Self) -> f32 {
    self.x.0 * other.x.0 + self.y.0 * other.y.0
  }
}

impl Add for WorldVec2 {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self { x: self.x + rhs.x, y: self.y + rhs.y }
  }
}

impl AddAssign for WorldVec2 {
  fn add_assign(&mut self, rhs: Self) {
    self.x += rhs.x;
    self.y += rhs.y;
  }
}

impl Sub for WorldVec2 {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Self { x: self.x - rhs.x, y: self.y - rhs.y }
  }
}

impl Mul<f32> for WorldVec2 {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self::Output {
    Self { x: self.x * rhs, y: self.y * rhs }
  }
}

impl Mul<WorldVec2> for f32 {
  type Output = WorldVec2;

  fn mul(self, rhs: WorldVec2) -> WorldVec2 {
    WorldVec2 { x: rhs.x * self, y: rhs.y * self }
  }
}

impl Neg for WorldVec2 {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Self { x: -self.x, y: -self.y }
  }
}

pub struct WorldRect(Rectangle);

impl WorldRect {
  pub fn new(width: WorldUnit, height: WorldUnit) -> Self {
    Self(Rectangle::new(width.to_untyped(), height.to_untyped()))
  }

  pub fn closest_point(&self, point: WorldVec2) -> WorldVec2 {
    WorldVec2::from_untyped(self.0.closest_point(point.to_untyped()))
  }
}
