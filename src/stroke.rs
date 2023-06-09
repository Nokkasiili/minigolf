use crate::vector2d::Vector2D;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, PartialEq, FromPrimitive, Copy, Clone)]
pub enum ShootingMode {
    Normal,
    Reverse,
    Right,
    Left,
}

impl ShootingMode {
    pub fn next(self) -> ShootingMode {
        let i = (self as i32 + 1) % 4;
        FromPrimitive::from_i32(i).unwrap_or(ShootingMode::Normal)
    }
}

struct Stroke {}

impl Stroke {
    pub fn calculate_stroke_power(
        origin: Vector2D<f32>,
        mouse_position: Vector2D<f32>,
    ) -> Vector2D<f32> {
        let displacement = mouse_position - origin;
        let distance = displacement.length();
        let mut scale = (distance - 5.0) / 30.0;

        scale = scale.clamp(0.075, 6.5);
        let normalized_displacement = displacement.normalize();
        let power = normalized_displacement * scale;
        power
    }

    pub fn calculate_speed(
        origin: Vector2D<f32>,
        mouse_coords: Vector2D<f32>,
        mode: ShootingMode,
    ) -> Vector2D<f32> {
        let stroke_power = Self::calculate_stroke_power(origin, mouse_coords);

        let mut speed = match mode {
            ShootingMode::Normal => stroke_power,
            ShootingMode::Reverse => -stroke_power,
            ShootingMode::Right => Vector2D::new(stroke_power.y, -stroke_power.x),
            ShootingMode::Left => Vector2D::new(-stroke_power.y, stroke_power.x),
        };

        let speed_length = speed.length();
        let mut speed_length_divided = speed_length / 6.5;
        speed_length_divided *= speed_length_divided;

        // TODO: Add randomization logic

        let speed_offset = speed_length_divided / 100000.0 - 0.25;
        speed += Vector2D::new(speed_offset, speed_offset);
        speed
    }
}

#[cfg(test)]
mod tests {
    use crate::stroke::ShootingMode;
    use crate::stroke::Stroke;
    use crate::vector2d::Vector2D;

    #[test]
    fn shooting_mode_next_test() {
        assert_eq!(ShootingMode::Normal.next(), ShootingMode::Reverse);
        assert_eq!(ShootingMode::Reverse.next(), ShootingMode::Right);
        assert_eq!(ShootingMode::Right.next(), ShootingMode::Left);
        assert_eq!(ShootingMode::Left.next(), ShootingMode::Normal);
    }

    #[test]
    fn stroke_power_test() {
        let point = Vector2D::new(52.5, 187.5);

        let mouse_coords = [
            Vector2D::new(89.0, 327.0),
            Vector2D::new(109.0, 373.0),
            Vector2D::new(99.0, 349.0),
        ];

        let results = [
            Vector2D::new(1.1744787325618775, 4.4887611833529295),
            Vector2D::new(1.8347721973612816, 6.023898099301199),
            Vector2D::new(1.5038857916963326, 5.223173233525973),
        ];

        for (&coords, &result) in mouse_coords.iter().zip(results.iter()) {
            let power = Stroke::calculate_stroke_power(point, coords);
            assert!(approx_eq(power.x, result.x));
            assert!(approx_eq(power.y, result.y));
        }
    }
    #[test]
    fn apply_shot_test() {
        let locations = vec![
            Vector2D::new(37.5, 52.5),
            Vector2D::new(34.528161559285664, 161.68100780584),
            Vector2D::new(37.26683591869343, 253.38915675186678),
            Vector2D::new(309.1215920962632, 184.95071762843094),
        ];
        let mouse_coords = vec![
            Vector2D::new(285.0, 205.0),
            Vector2D::new(27.0, 209.0),
            Vector2D::new(222.0, 354.0),
            Vector2D::new(328.0, 194.0),
        ];
        let shooting_modes = vec![
            ShootingMode::Normal,
            ShootingMode::Reverse,
            ShootingMode::Right,
            ShootingMode::Left,
        ];
        let results = vec![
            Vector2D::new(5.283868950354069, 3.159761474460588),
            Vector2D::new(-0.025247113706855367, -1.6627026133673766),
            Vector2D::new(2.8589116702757917, -5.958293636331584),
            Vector2D::new(-0.479600774949725, 0.2289881419932628),
        ];

        for ((&location, &mouse_coord), (&shooting_mode, &expected_result)) in locations
            .iter()
            .zip(mouse_coords.iter())
            .zip(shooting_modes.iter().zip(results.iter()))
        {
            let speed = Stroke::calculate_speed(location, mouse_coord, shooting_mode);
            assert!(approx_eq(speed.x, expected_result.x));
            assert!(approx_eq(speed.y, expected_result.y));
        }
    }

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.00001
    }
}
