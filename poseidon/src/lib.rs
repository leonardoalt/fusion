#![allow(non_snake_case)]
mod constants;
use zokrates_field::Pow;
use crate::constants::Constants;
pub use zokrates_field::{Field, Bn128Field};


fn ark(mut state: Vec<Bn128Field>, c: &Vec<Bn128Field>, it: u32) -> Vec<Bn128Field> {
    let N = state.len() as u32;
    for i in 0..N {
        state[i as usize] = state[i as usize].clone() + c[it as usize + i as usize].clone();
    };
    return state
}

fn sbox(mut state: Vec<Bn128Field>, f: u32, p: u32, r: u32) -> Vec<Bn128Field> {
    let N = state.len() as u32;
    state[0] = Pow::pow(state[0].clone(), 5);
    for i in 1..N {
        state[i as usize] = if (r < f/2) || (r >= f/2 + p) {
            Pow::pow(state[i as usize].clone(), 5)
        } else {
            state[i as usize].clone()
        }
    }
    return state
}

fn mix(state: Vec<Bn128Field>, m: &Vec<Vec<Bn128Field>>) -> Vec<Bn128Field> {
    let N = state.len() as u32;
    let mut out: Vec<Bn128Field> = vec![Field::try_from_str("0", 10).unwrap(); N as usize];
    for i in 0..N {
        let mut acc = Field::try_from_str("0", 10).unwrap();
        for j in 0..N {
            acc = acc + (state[j as usize].clone() * m[i as usize][j as usize].clone())
        }
        out[i as usize] = acc;
    }
    return out
}

pub fn hash_BN_128(input: Vec<Bn128Field>) -> Bn128Field {
    let con = Constants::default();
    let N: u32 = input.len() as u32;
    if !(N > 0 && N <= 6) {
        println!("through an error");
    }

    let t:u32 = N + 1;
    let rounds_p: Vec<u32> = vec![56, 57, 56, 60, 60, 63, 64, 63];

    let f:u32 = 8;
    let p:u32 = rounds_p[t as usize - 2];

    let c = &con.POSEIDON_C[t as usize -2];
    let m = &con.POSEIDON_M[t as usize -2];

    let mut state: Vec<Bn128Field> = vec![Field::try_from_str("0", 10).unwrap(); t as usize];
    for i in 1..t {
        state[i as usize] = input[i as usize - 1].clone();
    }

    for r in 0..f+p {
        state = ark(state, c, r * t);
        state = sbox(state, f, p, r);
        state = mix(state, m);
    }

    return state[0].clone()
}




#[cfg(test)]
mod tests {
    use super::hash_BN_128;
    use zokrates_field::{Bn128Field, Field};

    #[test]
    fn poseidon_1() {
        let input: Vec<Bn128Field> = vec![
            Field::try_from_str("1", 10).unwrap(),
        ]; 

        let hash: Bn128Field = hash_BN_128(input);
        println!("{}", hash);

        assert!(hash == Field::try_from_str("18586133768512220936620570745912940619677854269274689475585506675881198879027", 10).unwrap());

        let input: Vec<Bn128Field> = vec![
            Field::try_from_str("42", 10).unwrap(),
        ]; 

        let hash: Bn128Field = hash_BN_128(input);
        println!("{}", hash);

        assert!(hash == Field::try_from_str("12326503012965816391338144612242952408728683609716147019497703475006801258307", 10).unwrap());
    }

    #[test]
    fn poseidon_2() {
        let input: Vec<Bn128Field> = vec![
            Field::try_from_str("1", 10).unwrap(),
            Field::try_from_str("2", 10).unwrap(),
        ]; 

        let hash: Bn128Field = hash_BN_128(input);
        println!("{}", hash);

        assert!(hash == Field::try_from_str("7853200120776062878684798364095072458815029376092732009249414926327459813530", 10).unwrap());
    }

    #[test]
    fn poseidon_3() {
        let input: Vec<Bn128Field> = vec![
            Field::try_from_str("1", 10).unwrap(),
            Field::try_from_str("2", 10).unwrap(),
            Field::try_from_str("3", 10).unwrap(),
        ]; 

        let hash: Bn128Field = hash_BN_128(input);
        println!("{}", hash);

        assert!(hash == Field::try_from_str("6542985608222806190361240322586112750744169038454362455181422643027100751666", 10).unwrap());
    }
}
