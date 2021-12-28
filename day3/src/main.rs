use std::cmp::Ordering;

struct BitArray {
    bits: Vec<bool>,
}

impl BitArray {
    fn new(bits: &str) -> Self {
        BitArray {
            bits: vec!(),
        }
    }

    fn negate() {
        todo!()
    }

    fn to_decimal(&self) -> i64 {
        let mut decimal = 0;
        for bit in &self.bits {
            decimal = (decimal | *bit as i64) << 1;
        }
        decimal >> 1
    }
}

fn main() -> anyhow::Result<()> {
   
    let mut counter: [i64; 12] = [0; 12];

    let input = include_str!("input.txt")
        .lines()
        .for_each(|line| {
            line
                .chars()
                .enumerate()
                .for_each(|(i, val)| {
                    counter[i] += match val {
                        '0' => -1,
                        '1' => 1,
                        _ => panic!("not binary")
                    };
                });
        });

    let mut mcb = 0;
    counter
        .into_iter()
        .map(|val| match val.cmp(&0) {
            Ordering::Greater => 1,
            Ordering::Less => 0,
            Ordering::Equal => panic!("we got a an issue!")
        })
        .for_each(|entry| {
            mcb = (mcb | entry) << 1
        });
    
    //let mut lcb = 0;
    //counter
    //    .into_iter()
    //    .map(|val| match val.cmp(&0) {
    //        Ordering::Greater => 0,
    //        Ordering::Less => 1,
    //        Ordering::Equal => panic!("we got a an issue!")
    //    })
    //    .for_each(|entry| {
    //        lcb = (lcb | entry) << 1
    //    });
    //

    mcb = mcb >> 1;
    let lcb = !mcb;

    println!("{:?}", mcb * lcb);
    println!("{:?}", mcb);
    println!("{:?}", lcb);

    Ok(())
}
