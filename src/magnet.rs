use crate::map::Map;
use crate::tile::{Special, Tile};

pub struct MagnetForces {
    forces: Vec<[i32; 2]>,
}
pub struct Magnet {
    repel: bool,
    i: usize,
}

impl Magnet {
    fn extract_magnets(tiles: &[Tile]) -> Vec<Magnet> {
        let mut magnets = Vec::new();

        for (i, tile) in tiles.iter().enumerate() {
            let repel = match tile.special {
                Some(Special::MagnetAttract) => false,
                Some(Special::MagnetRepel) => true,
                _ => continue,
            };

            magnets.push(Magnet { repel, i });
        }

        magnets
    }
}

impl MagnetForces {
    pub const MAGNETHEIGHT: usize = Map::HEIGHT * Map::TILESIZE / 5;
    pub const MAGNETWIDTH: usize = Map::WIDTH * Map::TILESIZE / 5;

    pub fn get_force(&self, x: usize, y: usize) -> Option<[i32; 2]> {
        let index = (y * (Map::WIDTH * Map::TILESIZE / 5)) + (x / 5);
        self.forces.get(index).cloned()
    }

    pub fn calculate_forces(magnets: &[Magnet]) -> Self {
        let mut forces = vec![[0, 0]; Self::MAGNETWIDTH * Self::MAGNETHEIGHT];

        for y in (2..Map::HEIGHT * Map::TILESIZE).step_by(5) {
            for x in (2..Map::WIDTH * Map::TILESIZE).step_by(5) {
                let mut total_force = [0, 0];
                for magnet in magnets {
                    let (magnet_x, magnet_y) = Map::index_to_xy(magnet.i);
                    let screen_x = (magnet_x * Map::TILESIZE) + 8;
                    let screen_y = (magnet_y * Map::TILESIZE) + 8;
                    let delta_x = screen_x as i32 - x as i32;
                    let delta_y = screen_y as i32 - y as i32;
                    let distance = ((delta_x * delta_x + delta_y * delta_y) as f32).sqrt();
                    if distance <= 127.0 {
                        let normalized_x = (delta_x.abs() as f32) / distance;
                        let strength = 127.0 - distance;

                        let mut force_x = if delta_x < 0 {
                            (-1.0 * strength * normalized_x) as i32
                        } else {
                            (1.0 * strength * normalized_x) as i32
                        };

                        let mut force_y = if delta_y < 0 {
                            (-1.0 * strength * (1.0 - normalized_x)) as i32
                        } else {
                            (1.0 * strength * (1.0 - normalized_x)) as i32
                        };

                        if magnet.repel {
                            force_x = -force_x;
                            force_y = -force_y;
                        }

                        total_force[0] += force_x;
                        total_force[1] += force_y;
                    }
                }
                let array_index = ((y / 5) * (Map::WIDTH * Map::TILESIZE / 5)) + (x / 5);
                forces[array_index] = total_force;
            }
        }

        Self { forces }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_force() {
        let forces = vec![[1, 2]; MagnetForces::MAGNETWIDTH * MagnetForces::MAGNETHEIGHT];
        let magnet_forces = MagnetForces { forces };

        // Test a valid position
        assert_eq!(magnet_forces.get_force(5, 10), Some([1, 2]));

        // Test an out-of-bounds position
        assert_eq!(magnet_forces.get_force(100, 100), None);
    }

    #[test]
    fn test_calculate_forces() {
        let magnets = vec![
            Magnet {
                repel: false,
                i: Map::xy_to_index(48, 0),
            },
            Magnet {
                repel: true,
                i: Map::xy_to_index(0, 24),
            },
        ];

        let magnet_forces = MagnetForces::calculate_forces(&magnets);

        // Assert the forces for specific positions
        assert_eq!(magnet_forces.forces[121], [5, 0]);
        assert_eq!(magnet_forces.get_force(121 * 5, 0), Some([5, 0]));
        assert_eq!(magnet_forces.forces[10879], [-29, 93]);
    }
}
