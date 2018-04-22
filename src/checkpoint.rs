use hex::{HexPoint, HexVector};


// 0 is from right up to but not including the first checkpoint,
// 1 is from the first checkpoint up to but not including the second, etc.
// always in the range 0..6
pub type Section = i32;

pub fn point_to_section(hex_point: HexPoint) -> Section {
    if hex_point.q == 0 {
        if hex_point.r < 0 {
            2
        } else {
            5
        }
    } else if hex_point.q > 0 {
        if hex_point.r > 0 {
            5
        } else if -hex_point.r < hex_point.q {
            0
        } else {
            1
        }
    } else {
        if hex_point.r < 0 {
            2
        } else if -hex_point.q > hex_point.r {
            3
        } else {
            4
        }
    }
}

pub fn at_checkpoint(hex_point: HexPoint) -> bool {
    hex_point.q == 0 || hex_point.r == 0 || hex_point.q == -hex_point.r
}

pub fn forward(hex_point: HexPoint) -> HexVector {
    match point_to_section(hex_point) {
        0 => HexVector::new(0, -1),
        1 => HexVector::new(-1, 0),
        2 => HexVector::new(-1, 1),
        3 => HexVector::new(0, 1),
        4 => HexVector::new(1, 0),
        5 => HexVector::new(1, -1),
        _ => unreachable!(),
    }
}

pub fn backward(hex_point: HexPoint) -> HexVector {
    let mut section = point_to_section(hex_point);
    if at_checkpoint(hex_point) {
        section = (section + 5) % 6;
    }
    match section {
        0 => HexVector::new(0, 1),
        1 => HexVector::new(1, 0),
        2 => HexVector::new(1, -1),
        3 => HexVector::new(0, -1),
        4 => HexVector::new(-1, 0),
        5 => HexVector::new(-1, 1),
        _ => unreachable!(),
    }
}


// same as Section, but increases beyond 5 when completing laps.
pub type Checkpoint = i32;

pub fn checkpoint_to_section(checkpoint: Checkpoint) -> Section {
    if checkpoint >= 0 {
        checkpoint % 6
    } else {
        5 + (checkpoint + 1) % 6
    }
}

pub fn update_checkpoint(old_checkpoint: Checkpoint, new_hex_point: HexPoint) -> Checkpoint {
  let old_section = checkpoint_to_section(old_checkpoint);
  let new_section = point_to_section(new_hex_point);
  if new_section == (old_section + 1) % 6 {
      old_checkpoint + 1
  } else if new_section == (old_section + 2) % 6 {
      old_checkpoint + 2
  } else if new_section == (old_section + 5) % 6 {
      old_checkpoint - 1
  } else if new_section == (old_section + 4) % 6 {
      old_checkpoint - 2
  } else {
      old_checkpoint
  }
}


// number of _completed_ laps, so 0 during the first lap, 1 during the second, etc.
// negative if the racer drives the wrong way.
pub type Lap = i32;

pub fn checkpoint_to_lap(checkpoint: Checkpoint) -> Lap {
    if checkpoint >= 0 {
        checkpoint / 6
    } else {
        (checkpoint + 1) / 6 - 1
    }
}
