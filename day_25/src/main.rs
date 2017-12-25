fn main() {
    //sanity_check();
    let p1 = part_one();
    println!("part one: {}", p1);
}

// 640k should be enough for anybody
const TAPE_LENGTH: usize = 640_000;

fn part_one() -> usize {
    let nb_runs = 12399302;
    let mut machine = Machine::new();
    for _ in 0..nb_runs {
        machine.step();
    }
    machine.run_diagnostic()
}

struct Machine {
    tape: [bool; TAPE_LENGTH],
    cur_pos: usize,
    state: State,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    A, B, C, D, E, F
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right
}

struct Transition {
    state: State,
    mov: Direction,
    update: bool,
}

impl Machine {
    fn new() -> Self {
        Machine {
            tape: [false; TAPE_LENGTH],
            cur_pos: TAPE_LENGTH / 2,
            state: State::A,
        }
    }

    fn step(&mut self) {
        use Direction::*;
        let cur_val = self.tape[self.cur_pos];
        let transition = match (self.state, cur_val) {
            (State::A, false) => Transition { state: State::B, mov: Right, update: true },
            (State::A, true) => Transition { state: State::C, mov: Right, update: false },

            (State::B, false) => Transition { state: State::A, mov: Left, update: false },
            (State::B, true) => Transition { state: State::D, mov: Right, update: false },

            (State::C, false) => Transition { state: State::D, mov: Right, update: true },
            (State::C, true) => Transition { state: State::A, mov: Right, update: true },

            (State::D, false) => Transition { state: State::E, mov: Left, update: true },
            (State::D, true) => Transition { state: State::D, mov: Left, update: false },

            (State::E, false) => Transition { state: State::F, mov: Right, update: true },
            (State::E, true) => Transition { state: State::B, mov: Left, update: true },

            (State::F, false) => Transition { state: State::A, mov: Right, update: true },
            (State::F, true) => Transition { state: State::E, mov: Right, update: true },
        };

        self.tape[self.cur_pos] = transition.update;
        if transition.mov == Left { self.cur_pos -= 1 } else { self.cur_pos += 1 }
        self.state = transition.state;
    }

    fn run_diagnostic(&self) -> usize {
        let mut ones = 0;
        for i in 0..TAPE_LENGTH {
            if self.tape[i] { ones += 1 }
        }
        ones
    }
}

fn sanity_check() {
    let mut machine = Machine::new();
    for _ in 0..10 {
        machine.step();
        println!("{}, {}, {:?}",
                 machine.tape[machine.cur_pos],
                 machine.cur_pos,
                 machine.state);
    }
}

