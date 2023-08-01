use crate::*;
use std::f32::consts::*;

/// How far the eye can see
const FOV_RANGE: f32 = 0.25;
/// How wide the eye can see, in radiants
const FOV_ANGLE: f32 = PI + FRAC_PI_4;
/// How much photoreceptors there are in one eye.
/// This directly affects the number of neurons in the input layer.
const CELLS: usize = 9;

/// A structure containing the vision cells and FOV parameters, 
/// and capable of processing vision
#[derive(Debug)]
pub struct Eye {
    fov_range: f32,
    fov_angle: f32,
    cells: usize
}

impl Eye {
    fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);

        Self { fov_range, fov_angle, cells }
    }

    pub fn cells(&self) -> usize {
        self.cells
    }

    /// Given the bird's position and rotation, and the available food,
    /// returns the activations of each cell.
    /// Each cell's activation corresponds to the sum of the "energy"
    /// of food in the correct FOV. 
    /// The "energy" depends on the distance of the food to the bird.
    pub fn process_vision(
        &self, 
        position: na::Point2<f32>,
        rotation: na::Rotation2<f32>,
        foods: &[Food]
    ) -> Vec<f32> {
        let mut cells = vec![0.0; self.cells];

        for food in foods {
            // Asserts that the food is close enough to be seen
            let vec_bird_food = food.position - position;
            let dist = vec_bird_food.norm();

            if dist >= self.fov_range {
                continue;
            }

            // Asserts that the food in in the right angle of view
            let angle = na::Rotation2::rotation_between(
                &na::Vector2::y(), &vec_bird_food).angle();

            let angle = angle - rotation.angle();
            let angle = na::wrap(angle, -PI, PI);

            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 {
                continue;
            }

            // Computes the index of the cell that sees the food
            let angle = angle + self.fov_angle / 2.0; // angle \in [0, FOV_ANGLE]
            let cell = angle / self.fov_angle; // cell \in [0, 1]
            let cell = cell * (self.cells as f32); // cell \in [0, CELLS]
            let cell = (cell as usize).min(cells.len() - 1); // if the angle was exactly FOV_ANGLE, round up to the closest acceptable value

            // Update the magnitude of the cell activation
            let activation_energy = (self.fov_range - dist) / self.fov_range; // the closer the food, the higher the energy
            cells[cell] += activation_energy;
        }

        cells
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self::new(FOV_RANGE, FOV_ANGLE, CELLS)
    }
}

#[cfg(test)] 
mod tests {
    use super::*;

    struct TestCase {
        foods: Vec<Food>,
        fov_range: f32,
        fov_angle: f32,
        x: f32,
        y: f32,
        rot: f32,
        expected_vision: &'static str
    }

    const TEST_EYE_CELLS: usize = 13;

    impl TestCase {
        fn run(self) {
            let eye = Eye::new(
                self.fov_range, 
                self.fov_angle, 
                TEST_EYE_CELLS
            );

            let actual_vision = eye.process_vision(
                na::Point2::new(self.x, self.y), 
                na::Rotation2::new(self.rot), 
                &self.foods
            );

            let actual_vision = actual_vision
                .into_iter()
                .map(|cell| {
                    if cell >= 0.7 { "#" } // close food
                    else if cell >= 0.3 { "+" } // far food
                    else if cell > 0.0 { "." } // very far food
                    else { " " }
                })
                .collect::<Vec<_>>()
                .join("");

            assert_eq!(actual_vision, self.expected_vision);
        }
    }

    fn food(x: f32, y: f32) -> Food {
        Food {
            position: na::Point2::new(x, y)
        }
    }

    mod different_fov_ranges {
        use super::*;
        use test_case::test_case;

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
        fn test(fov_range: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(0.5, 1.0)],
                fov_angle: FRAC_PI_2,
                x: 0.5,
                y: 0.5,
                rot: 0.0,
                expected_vision,
                fov_range
            }.run()
        }
    }

    mod different_rotations {
        use super::*;
        use test_case::test_case;

        #[test_case(0.00 * PI, "         +   ")] // Food is to our right
        #[test_case(0.25 * PI, "        +    ")]
        #[test_case(0.50 * PI, "      +      ")] // Food is in front of us
        #[test_case(0.75 * PI, "    +        ")]
        #[test_case(1.00 * PI, "   +         ")] // Food is to our left
        #[test_case(1.25 * PI, " +           ")]
        #[test_case(1.50 * PI, "            +")] // Food is behind us
        #[test_case(1.75 * PI, "           + ")] // (we continue to see it
        #[test_case(2.00 * PI, "         +   ")] // due to 360° fov_angle.)
        #[test_case(2.25 * PI, "        +    ")]
        #[test_case(2.50 * PI, "      +      ")]
        fn test(rot: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(0.0, 0.5)],
                fov_range: 1.0,
                fov_angle: 2.0 * PI,
                x: 0.5,
                y: 0.5,
                rot,
                expected_vision,
            }.run()
        }
    }

    mod different_positions {
        use super::*;
        use test_case::test_case;

        // Checking X axis
        #[test_case(0.9, 0.5, "#           #")]
        #[test_case(0.8, 0.5, "  #       #  ")]
        #[test_case(0.7, 0.5, "   +     +   ")]
        #[test_case(0.6, 0.5, "    +   +    ")]
        #[test_case(0.5, 0.5, "    +   +    ")]
        #[test_case(0.4, 0.5, "     + +     ")]
        #[test_case(0.3, 0.5, "     . .     ")]
        #[test_case(0.2, 0.5, "     . .     ")]
        #[test_case(0.1, 0.5, "     . .     ")]
        #[test_case(0.0, 0.5, "             ")]

        // Checking the Y axis
        #[test_case(0.5, 0.0, "            +")]
        #[test_case(0.5, 0.1, "          + .")]
        #[test_case(0.5, 0.2, "         +  +")]
        #[test_case(0.5, 0.3, "        + +  ")]
        #[test_case(0.5, 0.4, "      +  +   ")]
        #[test_case(0.5, 0.6, "   +  +      ")]
        #[test_case(0.5, 0.7, "  + +        ")]
        #[test_case(0.5, 0.8, "+  +         ")]
        #[test_case(0.5, 0.9, ". +          ")]
        #[test_case(0.5, 1.0, "+            ")]
        fn test(x: f32, y: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(1.0, 0.4), food(1.0, 0.6)],
                fov_range: 1.0,
                fov_angle: FRAC_PI_2,
                rot: 3.0 * FRAC_PI_2,
                x,
                y,
                expected_vision,
            }.run()
        }
    }

    mod different_fov_angles {
        use super::*;
        use test_case::test_case;

        #[test_case(0.25 * PI, " +         + ")] // FOV is narrow = 2 foods
        #[test_case(0.50 * PI, ".  +     +  .")]
        #[test_case(0.75 * PI, "  . +   + .  ")] // FOV gets progressively
        #[test_case(1.00 * PI, "   . + + .   ")] // wider and wider...
        #[test_case(1.25 * PI, "   . + + .   ")]
        #[test_case(1.50 * PI, ".   .+ +.   .")]
        #[test_case(1.75 * PI, ".   .+ +.   .")]
        #[test_case(2.00 * PI, "+.  .+ +.  .+")] // FOV is wide = 8 foods
        fn test(fov_angle: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![
                    food(0.0, 0.0),
                    food(0.0, 0.33),
                    food(0.0, 0.66),
                    food(0.0, 1.0),
                    food(1.0, 0.0),
                    food(1.0, 0.33),
                    food(1.0, 0.66),
                    food(1.0, 1.0),
                ],
                fov_range: 1.0,
                x: 0.5,
                y: 0.5,
                rot: 3.0 * FRAC_PI_2,
                fov_angle,
                expected_vision,
            }.run()
        }
    }
}