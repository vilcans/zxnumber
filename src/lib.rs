//! Handles the number format used by the ZX81 and ZX Spectrum.
//!
//! A number is stored as 5 bytes in one of two formats:
//! A "small integer" which is an integer between -65535 and 65535,
//! or a floating point with a sign bit, a 7 bit exponent and 32 bit mantissa.

/// Contains a number in ZX format.
pub struct ZXNumber {
    exponent: u8,
    mantissa: [u8; 4],
}

impl ZXNumber {
    /// Convert a floating point number to ZX format.
    pub fn from_f64(n: f64) -> Self {
        if n.trunc() == n && n >= -0xffff as f64 && n <= 0xffff as f64 {
            Self::small_int(n as i32)
        } else {
            Self::full_fp(n)
        }
    }
    /// Get the 5 bytes in the format used by the ZX ROM.
    pub fn raw(&self) -> [u8; 5] {
        [
            self.exponent,
            self.mantissa[0],
            self.mantissa[1],
            self.mantissa[2],
            self.mantissa[3],
        ]
    }
    fn small_int(n: i32) -> Self {
        assert!(n <= 0xffff && n >= -0xffff);
        let mantissa = if n >= 0 {
            let n = n as u32;
            [0x00, n as u8, (n >> 8) as u8, 0x00]
        } else {
            let n = n as u32;
            [0xff, n as u8, (n >> 8) as u8, 0x00]
        };
        Self {
            exponent: 0,
            mantissa,
        }
    }
    fn full_fp(value: f64) -> ZXNumber {
        let (sign, value) = if value >= 0.0 {
            (0x00u8, value)
        } else {
            (0x80u8, -value)
        };
        dbg!(value.log2());
        let exponent = value.log2().floor() as i32;
        assert!(exponent >= -128 && exponent < 126);

        let m_unscaled = value / (2.0f64.powi(exponent)) - 1.0;
        let mantissa = (m_unscaled * (0x80000000u32 as f64)).round() as u32;

        Self {
            exponent: exponent as u8 + 0x81,
            mantissa: [
                (mantissa >> 24) as u8 | sign,
                (mantissa >> 16) as u8,
                (mantissa >> 8) as u8,
                mantissa as u8,
            ],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_10_to_zx() {
        let n = ZXNumber::from_f64(10.0);
        assert_eq!(n.raw(), [0, 0, 10, 0, 0]);
    }
    #[test]
    fn convert_negative_10_to_zx() {
        let n = ZXNumber::from_f64(-10.0);
        assert_eq!(n.raw(), [0, 0xff, 246, 255, 0]);
    }
    #[test]
    fn convert_pi_2_to_zx() {
        let n = ZXNumber::from_f64(1.5707963267948966);
        assert_eq!(n.raw(), [129, 73, 15, 218, 162]);
    }
    #[test]
    fn convert_65536_to_zx() {
        let n = ZXNumber::from_f64(65536.0);
        assert_eq!(n.raw(), [145, 0, 0, 0, 0]);
    }
    #[test]
    fn convert_negative_65536_to_zx() {
        let n = ZXNumber::from_f64(-65536.0);
        assert_eq!(n.raw(), [145, 128, 0, 0, 0]);
    }
    #[test]
    fn convert_negative_65537_to_zx() {
        let n = ZXNumber::from_f64(-65537.0);
        assert_eq!(n.raw(), [145, 128, 0, 128, 0]);
    }
}
