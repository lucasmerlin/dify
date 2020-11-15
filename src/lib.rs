const KY: f32 = 0.5053;
const KI: f32 = 0.299;
const KQ: f32 = 0.1957;

#[derive(Debug, PartialEq)]
pub struct YIQ {
    pub y: f32, // luminance, in range [0, 1]
    pub i: f32, // hue of color, in range ~ [-0.5, 0.5]
    pub q: f32, // saturation of color, in range ~ [-0.5, 0.5]
}

impl YIQ {
    pub fn from_rgb(rgb: &[u8; 3]) -> Self {
        let matrix: [[f32; 3]; 3] = [
            [0.29889531, 0.58662247, 0.11448223],
            [0.59597799, -0.27417160, -0.32180189],
            [0.21147019, -0.52261711, 0.31114694],
        ];

        let r = rgb[0] as f32;
        let g = rgb[1] as f32;
        let b = rgb[2] as f32;

        let y = matrix[0][0] * r + matrix[0][1] * g + matrix[0][2] * b;
        let i = matrix[1][0] * r + matrix[1][1] * g + matrix[1][2] * b;
        let q = matrix[2][0] * r + matrix[2][1] * g + matrix[2][2] * b;

        Self { y, i, q }
    }

    // in the performance critical applications, square root can be omiitted
    pub fn squared_distance(&self, other: &Self) -> f32 {
        let dy = other.y - self.y;
        let di = other.i - self.i;
        let dq = other.q - self.q;

        // compensate for irregularities, introduce coefficients
        KY * dy.powi(2) + KI * di.powi(2) + KQ * dq.powi(2)
    }

    // taking the square root of the distance gives better perceptual results
    pub fn square_root_distance(&self, other: &Self) -> f32 {
        self.squared_distance(other).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::YIQ;

    #[test]
    fn test_from_rgb() {
        let expected = YIQ {
            y: 0.0,
            i: 0.0,
            q: 0.0,
        };
        let actual = YIQ::from_rgb(&[0, 0, 0]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_squared_distance_same() {
        let a = YIQ {
            y: 0.5,
            i: -0.1,
            q: 0.1,
        };
        let b = YIQ {
            y: 0.5,
            i: -0.1,
            q: 0.1,
        };
        assert_eq!(a.squared_distance(&b), 0.0);
    }

    #[test]
    fn test_squared_distance_not_same() {
        let a = YIQ {
            y: 0.5,
            i: 0.1,
            q: -0.1,
        };
        let b = YIQ {
            y: 0.5,
            i: -0.1,
            q: 0.1,
        };
        assert_eq!(a.squared_distance(&b), 0.019788);
    }

    #[test]
    fn test_square_root_distance_same() {
        let a = YIQ {
            y: 0.5,
            i: -0.1,
            q: 0.1,
        };
        let b = YIQ {
            y: 0.5,
            i: -0.1,
            q: 0.1,
        };
        assert_eq!(a.square_root_distance(&b), 0.0);
    }

    #[test]
    fn test_square_root_distance_not_same() {
        let a = YIQ {
            y: 0.5,
            i: 0.1,
            q: -0.1,
        };
        let b = YIQ {
            y: 0.5,
            i: -0.1,
            q: 0.1,
        };
        assert_eq!(a.square_root_distance(&b), 0.14066982);
    }
}
