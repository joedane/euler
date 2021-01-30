use rand::prelude::*;
use rand::seq::SliceRandom;


const GO:usize = 0;
const JAIL:usize = 10;
const NUM_CARDS:usize = 16;
const C1:usize = 11;
const E3:usize = 24;
const H2:usize = 39;
const R1:usize = 5;
const U1:usize = 12;
const U2:usize = 28;
const G2J:usize = 30;
const CC1:usize = 2;
const CC2:usize = 17;
const CC3:usize = 33;
const CH1:usize = 7;
const CH2:usize = 22;
const CH3:usize = 36;

const SPACES:usize = 40;
const DIE_FACES:usize = 4;

#[derive(Debug)]
enum CC {
    Ignore,
    Goto(usize)
}

#[derive(Debug)]
enum Chance {
    Ignore,
    Goto(usize),
    GotoNextR,
    GotoNextU,
    GoBack3
}


fn main() {

    let mut position:usize = 0;
    
    let mut landings: [u32; SPACES]  = [0; SPACES];
    let mut rolls: [u32; 2*DIE_FACES - 1] = [0; 2*DIE_FACES - 1];
    
    let mut num_rolls: u32 = 0;
    const MAX_ROLLS:u32 = 10_000_000;
    let mut rng = SmallRng::seed_from_u64(13013013);
    let mut cc_cards: [CC; NUM_CARDS] = [ CC::Goto(GO), CC::Goto(JAIL), CC::Ignore, CC::Ignore,
                                              CC::Ignore, CC::Ignore, CC::Ignore, CC::Ignore,
                                              CC::Ignore, CC::Ignore, CC::Ignore, CC::Ignore,
                                              CC::Ignore, CC::Ignore, CC::Ignore, CC::Ignore ];

    let mut chance_cards: [Chance; NUM_CARDS] = [ Chance::Goto(GO), Chance::Goto(JAIL), Chance::Goto(C1), Chance::Goto(E3),
                                                  Chance::Goto(H2), Chance::Goto(R1), Chance::GotoNextR, Chance::GotoNextR,
                                                  Chance::GotoNextU, Chance::GoBack3, Chance::Ignore, Chance::Ignore,
                                                  Chance::Ignore, Chance::Ignore, Chance::Ignore, Chance::Ignore ];

    // RR at 5, 15, 25, 35
    let next_rr: [usize; SPACES] = [5, 5, 5, 5, 5,
                                    15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
                                    25, 25, 25, 25, 25, 25, 25, 25, 25, 25,
                                    35, 35, 35, 35, 35, 35, 35, 35, 35, 35,
                                    5, 5, 5, 5, 5];

    
    let mut cc_pos:usize = 0;
    let mut chance_pos:usize = 0;
    
    cc_cards.shuffle(&mut rng);
    chance_cards.shuffle(&mut rng);
    
    loop {

        if num_rolls == MAX_ROLLS {
            break;
        }

        let roll = rng.gen_range(1..DIE_FACES+1) + rng.gen_range(1..DIE_FACES+1);
        num_rolls += 1;
        rolls[roll-2] += 1;
//        println!("starting at {}, rolled a {}", position, roll);
        position = (position + roll) % SPACES;
//        println!("tentative new position: {}", position);

        if position == CC1 || position == CC2 || position == CC3 {
//            println!("CC: {:?}", cc_cards[cc_pos]);
            position = match cc_cards[cc_pos] {
                CC::Goto(pos) => pos,
                CC::Ignore => position
            };
            if cc_pos == NUM_CARDS-1 {
                cc_cards.shuffle(&mut rng);
                cc_pos = 0;
            } else {
                cc_pos += 1;
            }
        }
        else if position == CH1 || position == CH2 || position == CH3 {
//            println!("CH: {:?}", chance_cards[chance_pos]);
            position = match chance_cards[chance_pos] {
                Chance::Goto(pos) => pos,
                Chance::GotoNextR => { next_rr[position] }
                Chance::GotoNextU => { if position >= U1 && position < U2 { U2 } else { U1 }},
                Chance::GoBack3 => (position + SPACES - 3) % SPACES,
                Chance::Ignore => position
            };
            if chance_pos == NUM_CARDS-1 {
                chance_cards.shuffle(&mut rng);
                chance_pos = 0;
            } else {
                chance_pos += 1;
            }
        }
        else if position == G2J {
            position = JAIL;
        }

        landings[position] += 1;
    }
    /*
    println!("spaces ...");
    for pos in 0..SPACES {
        println!("{}: {}", pos, landings[pos] as f32 / MAX_ROLLS as f32); 
    }
    */
    let mut idx = Vec::new();
    for i in 0..SPACES {
        idx.push(i as usize);
    }
    idx.sort_by(|a, b| landings[*b].cmp(&landings[*a]));
    for i in 0..idx.len() {
        println!("{}:\t{}", idx[i], landings[idx[i]] as f32 / MAX_ROLLS as f32);
    }
    /*
    println!("rolls ...");
    for r in 0..11 {
        println!("{}: {}", r+2, rolls[r] as f32 / MAX_ROLLS as f32);
    }
     */
}
