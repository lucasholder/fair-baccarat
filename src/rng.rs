use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::mac::MacResult;
use crypto::sha2::Sha256;

pub struct ProvablyFairRNG {
    client_seed: String,
    server_seed: String,
    nonce: u64,
    current_round: u64,
    current_round_cursor: usize,
    current_round_mac: Option<MacResult>,
}

impl ProvablyFairRNG {
    pub fn new(client_seed: &str, server_seed: &str, nonce: u64) -> ProvablyFairRNG {
        return ProvablyFairRNG {
            client_seed: client_seed.to_string(),
            server_seed: server_seed.to_string(),
            nonce,
            // TODO: group this under iter field?
            current_round: 0,
            current_round_cursor: 0,
            current_round_mac: None,
        };
    }

    fn update_current_round_buffer(&mut self) {
        let mut hmac = Hmac::new(Sha256::new(), self.server_seed.as_bytes());
        let string = format!("{}:{}:{}", self.client_seed, self.nonce, self.current_round);
        hmac.input(string.as_bytes());
        let mac = hmac.result();
        self.current_round_mac = Some(mac);
    }
}

impl std::iter::Iterator for ProvablyFairRNG {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        // 32 = number of bytes in self.current_round_buffer (aka size of hmac signature)
        // TODO: use sizeof pragma?
        let mac = match &self.current_round_mac {
            None => {
                self.update_current_round_buffer();
                return self.next();
            }
            Some(v) => v,
        };

        let buf = mac.code();
        let result = buf[self.current_round_cursor];
        if self.current_round_cursor == 31 {
            self.current_round_cursor = 0;
            self.current_round += 1;
            self.current_round_mac = None;
        } else {
            self.current_round_cursor += 1;
        }
        return Some(result);
    }
}

pub struct ProvablyFairRNGFloat {
    provably_fair_rng: ProvablyFairRNG,
}

impl ProvablyFairRNGFloat {
    pub fn new(provably_fair_rng: ProvablyFairRNG) -> ProvablyFairRNGFloat {
        ProvablyFairRNGFloat { provably_fair_rng }
    }
}

// technique for converting groups of bytes into a float
fn bytes_to_float(bytes: &[u8]) -> f64 {
    bytes
        .iter()
        .fold((0., 0), |(result, i), value| {
            let value = *value as f64;
            let divider: u64 = 256_u64.pow(i + 1);
            let partial_result = value / divider as f64;
            (result + partial_result, i + 1)
        })
        .0
}

// TODO: instead overload ProvablyFairRNG with ProvablyFairRNG<f64> ?
impl std::iter::Iterator for ProvablyFairRNGFloat {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        let bytes_per_float = 4;
        let bytes = &mut [0; 4];
        for i in 0..bytes_per_float {
            let byte = self.provably_fair_rng.next().unwrap();
            bytes[i] = byte;
        }
        let result = bytes_to_float(bytes);
        return Some(result);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn provably_fair_rng() {
        let client_seed = "some client seed";
        let server_seed = "some server seed";
        let nonce = 1;
        let mut rng = ProvablyFairRNG::new(client_seed, server_seed, nonce);
        let expected_values = vec![
            151, 136, 121, 135, 209, 159, 189, 233, 43, 248, 146, 253, 71, 34, 215, 176, 139, 160,
            47, 225, 233, 221, 169, 198, 187, 103, 171, 31, 87, 118, 23, 138, 198, 14, 60, 130,
            130, 198, 153, 83,
        ];
        for val in expected_values {
            assert_eq!(rng.next(), Some(val));
        }
    }

    #[test]
    fn provably_fair_rng_float() {
        let client_seed = "some client seed";
        let server_seed = "some server seed";
        let nonce = 1;
        let rng_bytes = ProvablyFairRNG::new(client_seed, server_seed, nonce);
        let mut rng = ProvablyFairRNGFloat::new(rng_bytes);
        let expected_values = vec![
            0.5919261889066547,
            0.81884371698834,
            0.17176169087179005,
            0.277875404804945,
            0.5454130100551993,
            0.913538561668247,
            0.732050604885444,
            0.34164569014683366,
            0.7736547295935452,
            0.5108428790699691,
        ];
        for val in expected_values {
            let actual = rng.next();
            // println!("{} == {} ?", actual.unwrap(), val);
            assert_eq!(actual, Some(val));
        }
    }
}
