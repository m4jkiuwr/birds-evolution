use crate::*;
use std::f32::consts::*;

const FOV_RANGE: f32 = 0.25;
const FOV_ANGLE: f32 = PI + FRAC_PI_4;
const CELLS: usize = 9;

#[derive(Debug)]
pub struct Eye {
    range: f32,
    angle: f32,
    cells: usize,
}

impl Eye {
    pub fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);

        Self {
            range: fov_range,
            angle: fov_angle,
            cells,
        }
    }
    pub fn cells(&self) -> usize {
        self.cells
    }

    pub fn process_vision(
        &self,
        position: na::Point2<f32>,
        rotation: na::Rotation2<f32>,
        foods: &[Food],
    ) -> Vec<f32> {
        let mut cells = vec![0.0; self.cells];
        for food in foods {
            let vec_to_food = food.position - position;
            let dist = vec_to_food.norm();
            let food_angle = na::wrap(
                na::Rotation2::rotation_between(&na::Vector2::y(), &vec_to_food).angle()
                    - rotation.angle(),
                -PI,
                PI,
            );

            if dist >= self.range || food_angle > self.angle / 2.0 || food_angle < -self.angle / 2.0
            {
                continue;
            }
            let food_angle = food_angle + self.angle / 2.0;
            let cell_idx = (food_angle / (self.angle / self.cells as f32)) as usize;
            let cell_idx = cell_idx.min(self.cells - 1);

            let energy = (self.range - dist) / self.range;

            cells[cell_idx] += energy;
        }
        cells
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self {
            range: FOV_RANGE,
            angle: FOV_ANGLE,
            cells: CELLS,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn make_readable(vision: Vec<f32>) -> String {
        vision
            .iter()
            .map(|&cell| {
                if cell >= 0.7 {
                    "#"
                } else if cell >= 0.3 {
                    "+"
                } else if cell > 0.0 {
                    "."
                } else {
                    " "
                }
            })
            .collect()
    }
    fn food(x: f32, y: f32) -> Food {
        Food {
            position: na::Point2::new(x, y),
        }
    }

    struct TestCase {
        foods: Vec<Food>,
        range: f32,
        angle: f32,
        cells: usize,
        x: f32,
        y: f32,
        rot: f32,
        expected_vision: &'static str,
    }

    impl TestCase {
        fn run(self) {
            let eye = Eye::new(self.range, self.angle, self.cells);

            let actual_vision = eye.process_vision(
                na::Point2::new(self.x, self.y),
                na::Rotation2::new(self.rot),
                &self.foods,
            );
            let processed_vision = make_readable(actual_vision);

            assert_eq!(processed_vision, self.expected_vision)
        }
    }

    #[test_case(1.0, "      +      ")]
    #[test_case(0.9, "      +      ")]
    #[test_case(0.8, "      +      ")]
    #[test_case(0.7, "      .      ")]
    #[test_case(0.6, "      .      ")]
    #[test_case(0.5, "             ")]
    #[test_case(0.4, "             ")]
    #[test_case(0.3, "             ")]
    #[test_case(0.2, "             ")]
    #[test_case(0.1, "             ")]
    fn fov_ranges(range: f32, expected_vision: &'static str) {
        TestCase {
            range,
            expected_vision,
            foods: vec![food(0.5, 1.0)],
            cells: 13,
            angle: FRAC_PI_2,
            x: 0.5,
            y: 0.5,
            rot: 0.0,
        }
        .run()
    }

    #[test_case(0.00*PI, "    +")]
    #[test_case(0.25*PI, "   + ")]
    #[test_case(0.50*PI, "  +  ")]
    #[test_case(0.75*PI, " +   ")]
    #[test_case(0.99*PI, "+    ")]
    #[test_case(1.25*PI, "     ")]
    #[test_case(1.50*PI, "     ")]
    #[test_case(1.75*PI, "     ")]
    #[test_case(2.00*PI, "    +")]
    #[test_case(2.25*PI, "   + ")]
    #[test_case(2.50*PI, "  +  ")]
    fn rotations(rot: f32, expected_vision: &'static str) {
        TestCase {
            foods: vec![food(0.0, 0.5)],
            range: 1.0,
            angle: PI,
            x: 0.5,
            y: 0.5,
            cells: 5,
            rot,
            expected_vision,
        }
        .run()
    }
}
