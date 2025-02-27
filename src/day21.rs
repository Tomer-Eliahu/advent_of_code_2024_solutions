use std::fs;

use keypad::CodeHandler;

//This was a super confusing problem. If you are trying to follow this I recommend reading the puzzle prompt first
//and having a pen and paper ready.

//Part 1 solution notes:

//This is how the numeric keypad looks

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A | <-- Robot_A starts here
//     +---+---+

//When the robot_A arrives at the numeric keypad, its robotic arm is pointed at the A button in the bottom right corner.

//Might be useful to make a function that takes as input where the robot is now and where it wants to go
//and returns a direction selection to get there
//So if the Robot was at 2 and wanted to go to 9
//this_func(2,9) -> UP (any good direction; just pick one)
//After Robot_A moves UP it is at 5 so we call
//this_func(5,9) -> UP -> this_func(8,9) -> Right -> this_func(9,9) -> Returns a signal we are done (like am Err).

//To implement this_func we cna keep track of the row,col of each key on th numeric keypad.
//Then spitting out a direction is just comparing 2 tuples. Also note the first input to this func
//is simply where RobotA is pointing at right now.

//This also makes it clear *why* it is ok to pick any good valid direction.
//To get from 1 to A we need to move 2 to the right and 1 down (the order does not matter as long
//as we never point at the gap). We can use Rust's type system to make sure we never move down from 1 or left from 0.

//** Note the directions returned are what we need to key-in for Robot_B to execute the movement (see below) **.

//We know if we wanted to travel the shortest path to key-in 029A
//This problem can be boiled down to:
//* find the shortest path from A (where Robot_A starts) to 0
//* find the shortest path 0 to 2
//* find the shortest path 2 to 9
//* find the shortest path 9 to A
//Then the shortest path to key-in 029A is all thoses * paths concatenated

//The controls for Robot_A look like:

//     +---+---+
//     | ^ | A | <-- Robot_B starts here
// +---+---+---+
// | < | v | > |
// +---+---+---+

//When the robot arrives at the directional keypad, its robot arm is pointed at the A button in the upper right corner.

//The controls for Robot_B are the same as the controls for Robot_A.

//We can make a version of this_func for Robot_B.

//** Note the directions returned are what we need to key-in for Robot_C (see below) **.

//Robot_C controls Robot_B and it has the same controls and starts at the same location as Robot_B.

//The good news is that we can directly type the input to Robot_C (thankfully no Robot_D).

//So what we have is:
//We control --> Robot_C controls--> Robot_B controls --> Robot_A controls --> numeric keypad.
//
// Important Note: No robots can point at a gap ever! (they have to be pointing at a valid key at all times).
//
// So what we can do:
//
// take input to numeric keypad -->(repeated calls to this_func yield) input to Robot_A
// --> (calls to a different version of this_func yield) input to Robot_B
// --> input to Robot_C (what we type in)

//Then we also want a final function to compute the complexity per code:
//This is the len of a
//shortest possible input to Robot_C * (the input to numeric keypad stripped of A's and remove leading zeros)

//The answer to part 1 is the sum of the complexity score of all codes.

pub mod keypad {
    use std::{collections::HashMap, marker::PhantomData};

    ///This is how the numeric keypad looks
    ///
    ///     +---+---+---+
    ///     | 7 | 8 | 9 |
    ///     +---+---+---+
    ///     | 4 | 5 | 6 |
    ///     +---+---+---+
    ///     | 1 | 2 | 3 |
    ///     +---+---+---+
    ///         | 0 | A | <-- Robot_A starts here
    ///         +---+---+
    ///
    ///
    struct NumKeyPad {
        pointed_at: char,
        ///A HashMap that translates chars to their position on the keypad
        num_to_pos: HashMap<char, (usize, usize)>,
    }

    impl NumKeyPad {
        fn new() -> NumKeyPad {
            let pointed_at = 'A';

            let mut num_to_pos = HashMap::new();
            num_to_pos.insert('0', (0, 1));
            num_to_pos.insert('A', (0, 2));
            num_to_pos.insert('1', (1, 0));
            num_to_pos.insert('2', (1, 1));
            num_to_pos.insert('3', (1, 2));
            num_to_pos.insert('4', (2, 0));
            num_to_pos.insert('5', (2, 1));
            num_to_pos.insert('6', (2, 2));
            num_to_pos.insert('7', (3, 0));
            num_to_pos.insert('8', (3, 1));
            num_to_pos.insert('9', (3, 2));

            Self {
                pointed_at,
                num_to_pos,
            }
        }

        ///Takes as input which key to move to.
        /// Returns a vector of chars: the input that Robot B needs to key-into
        /// Robot A's controls to make this *movement* **and** button press (on the NumPad) happen.
        ///  
        /// Note this updates which key Robot_A is pointing at.
        ///
        ///Note The controls for Robot_A look like:
        ///
        ///          +---+---+
        ///          | ^ | A | <-- Robot_B starts here
        ///      +---+---+---+
        ///      | < | v | > |
        ///      +---+------++
        ///
        /// So if Robot_A needs to move up twice and right once, it
        /// is less traveling for Robot_B (thus a shorter input for *Robot_C*)
        /// if it makes Robot_A do Move up, Move up, Move right
        /// than Move up, Move right, Move up.
        ///
        /// When Robot A_moves up Robot B is on '^' and Robot_C is on 'A'.
        ///
        ///
        /// Similarly, if Robot_C makes Robot_B do Move up, Move Up, Move Right,
        /// rather than Move up, Move Right, Move up then it results in a shorter input for US (we operate robot C).
        ///
        /// When Robot_B move up Robot C is on '^' and WE press 'A'.
        ///
        fn move_to_and_press(&mut self, target: char) -> Vec<char> {
            let mut key_into_a: Vec<char> = vec![];
            let mut cur_pos = *self.num_to_pos.get(&self.pointed_at).unwrap();
            let target_pos = *self.num_to_pos.get(&target).unwrap();

            //Do the first move or see if we can repeat the last_move
            let mut last_move = Self::general_move(&target_pos, &mut cur_pos, &mut key_into_a);

            let zero_pos = *self.num_to_pos.get(&'0').unwrap();
            let one_pos = *self.num_to_pos.get(&'1').unwrap();

            //We should always prioritize doing the last move again if possible (and needed).
            while let Some(char_move) = last_move {
                last_move = match char_move {
                    '^' => {
                        //Robot A can always move up
                        if cur_pos.0 < target_pos.0 {
                            //move up (we can and need to)
                            cur_pos.0 += 1;
                            key_into_a.push('^');
                            Some('^')
                        } else {
                            Self::general_move(&target_pos, &mut cur_pos, &mut key_into_a)
                        }
                    }
                    '>' => {
                        //Robot A can always move right
                        if cur_pos.1 < target_pos.1 {
                            cur_pos.1 += 1;
                            key_into_a.push('>');
                            Some('>')
                        } else {
                            Self::general_move(&target_pos, &mut cur_pos, &mut key_into_a)
                        }
                    }
                    'v' => {
                        //Need to also make sure Robot_A is not pointing at '1' on the numpad
                        if cur_pos.0 > target_pos.0 && cur_pos != one_pos {
                            //move down
                            cur_pos.0 -= 1;
                            key_into_a.push('v');
                            Some('v')
                        } else {
                            Self::general_move(&target_pos, &mut cur_pos, &mut key_into_a)
                        }
                    }
                    '<' => {
                        //Need to also make sure Robot_A is not pointing at '0'
                        if cur_pos.1 > target_pos.1 && cur_pos != zero_pos {
                            //move left
                            cur_pos.1 -= 1;
                            key_into_a.push('<');
                            Some('<')
                        } else {
                            Self::general_move(&target_pos, &mut cur_pos, &mut key_into_a)
                        }
                    }

                    _ => unreachable!(),
                }
            }

            //We now know Robot_A is pointing at the target
            self.pointed_at = target;
            key_into_a.push('A'); //We want Robot A to press the button it is pointing at.

            key_into_a
        }

        ///When we do not need to repeat Robot A's last movement (to optimize shorter input for Robot C),
        /// do a general guaranteed valid move and return what that move was.
        /// Also adjust key_into_a and cur_pos as well.
        /// A None is returned when the cur_pos is equal to the target_pos (so no move needed)
        fn general_move(
            target_pos: &(usize, usize),
            cur_pos: &mut (usize, usize),
            key_into_a: &mut Vec<char>,
        ) -> Option<char> {
            //Note the following acts correctly even when we are at '0' or '1'
            match (cur_pos.0.cmp(&target_pos.0), cur_pos.1.cmp(&target_pos.1)) {
                //The following are stright forward (we have no choice what to do)
                (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => None,

                (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => {
                    //Moving right is always valid
                    cur_pos.1 += 1;
                    key_into_a.push('>');
                    Some('>')
                }

                (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => {
                    //Note is always valid moving left in this case
                    cur_pos.1 -= 1;
                    key_into_a.push('<');
                    Some('<')
                }

                (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => {
                    //Moving down is valid
                    cur_pos.0 -= 1;
                    key_into_a.push('v');
                    Some('v')
                }

                (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => {
                    //Moving up
                    cur_pos.0 += 1;
                    key_into_a.push('^');
                    Some('^')
                }

                //Need to go left,up or up,left (starting at A on Robot_A's controls ending back at A)
                //Down left left right up right (to press left then up as Robot_B pointing at Robot_A)
                //Left Down Left Right right up (to press up then left as Robot_B pointing at Robot_A)
                //
                //Going left twice in a row means Robot_C has to travel less
                //(going to left more costly than going to right for Robot C).
                //
                //So prioritze going left over going up // 16 up then left

                //If you find it confusing (it really is!!), then it might be helpful to directly count how long
                //the resulting input to Robot C will be.
                (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => {
                    //We also have to account for the case where we need to repeatdely press left.
                    //We want to make sure we do all of our left press in a row

                    if (*cur_pos == (0, 2) && cur_pos.1 == target_pos.1 + 2) || (*cur_pos == (0, 1))
                    {
                        //go up
                        cur_pos.0 += 1;
                        key_into_a.push('^');
                        Some('^')
                    } else {
                        cur_pos.1 -= 1;
                        key_into_a.push('<');
                        Some('<')
                    }

                    //Confirmed better by testing
                }

                //Need to go left,down or down,left
                //Down Left Left Right Right Up (to press left then down as Robot_B pointing at Robot_A) --14 clicks
                //Down Left Left Right Right up  (to press down then left as Robot_B pointing at Robot_A)-- 17 cicks
                //
                //so we pick to prioritize left
                (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => {
                    // cur_pos.1 -= 1;
                    // key_into_a.push('<');
                    // Some('<')

                    cur_pos.0 -= 1;
                    key_into_a.push('v');
                    Some('v')

                    //Confirmed by testing the two options are equivalent
                }

                //Need to go right,up or up, right
                //Down Up Left Right (to press right then up as Robot_B pointing at Robot_A) --15 clicks
                //Left Right Down Up  (to press up then right as Robot_B pointing at Robot_A) -- 15 clicks
                //
                //so we pick to prioritize right
                (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => {
                    // cur_pos.1 += 1;
                    // key_into_a.push('>');
                    // Some('>')

                    cur_pos.0 += 1;
                    key_into_a.push('^');
                    Some('^')

                    //Going up is much better (found out by testing)
                }

                //Need to go right,down or down, right
                //Down Left Right Up (to press right then down as Robot_B pointing at Robot_A) --14 clicks
                //Down Left Right Up (to press down then right as Robot_B pointing at Robot_A) --13
                //
                //so we pick to prioritize down
                (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => {
                    //Need to account for repeatdly needing to move down
                    if (*cur_pos == (3, 0) && cur_pos.0 == target_pos.0 + 3)
                        || (*cur_pos == (2, 0) && cur_pos.0 == target_pos.0 + 2)
                        || (*cur_pos == (1, 0))
                    {
                        //go right
                        cur_pos.1 += 1;
                        key_into_a.push('>');
                        Some('>')
                    } else {
                        cur_pos.0 -= 1;
                        key_into_a.push('v');
                        Some('v')
                    }

                    //Confirmed better by testing
                }
            }
        }
    }

    trait Robot {}

    #[derive(Clone, Copy)]
    struct A;

    impl Robot for A {}

    #[derive(Clone, Copy)]
    struct B;

    impl Robot for B {}
    ///The controls for Robot_A look like:
    ///
    ///          +---+---+
    ///          | ^ | A | <-- Robot_B starts here
    ///      +---+---+---+
    ///      | < | v | > |
    ///      +---+------++
    ///
    ///
    ///The controls for Robot_B are the same as the controls for Robot_A.
    ///
    ///     
    ///          +---+---+
    ///          | ^ | A | <-- Robot_C starts here
    ///      +---+---+---+
    ///      | < | v | > |
    ///      +---+------++
    ///
    /// So we keep track of which robot this is the controls for using an inner type A or B.
    /// Because types A and B are zero-sized this is a 0 cost abstraction.
    #[derive(Clone)]
    struct ControlsRobot<T: Robot> {
        ///What Robot_B is pointing at on the controls of Robot_A
        pointed_at: char,
        ///A HashMap that translates chars to their position on the keypad
        num_to_pos: HashMap<char, (usize, usize)>,
        _marker: PhantomData<T>,
    }

    //We could have also just have made 1 Controls type and have NumKeyPad be a possible inner type as well,
    //but I find this separation clearer.

    //We do the following so we have the correct documentation for each inner Robot type.
    impl ControlsRobot<A> {
        ///Takes as input which key to move to.
        /// Returns a vector of chars: the input that Robot C needs to key-into
        /// Robot B's controls to make this *movement* **and** button press (on the Robot A's Contorls) happen.
        ///  
        /// Note this updates which key Robot_B is pointing at.
        fn move_to_and_press(&mut self, target: char) -> Vec<char> {
            ControlsRobot::inner_move_to_and_press(self, target)
        }
    }

    impl ControlsRobot<B> {
        ///Takes as input which key to move to.
        /// Returns a vector of chars: the input that **We** need to key-into
        /// Robot C's controls to make this *movement* **and** button press (on the Robot B's Contorls) happen.
        ///  
        /// Note this updates which key Robot_B is pointing at.
        fn move_to_and_press(&mut self, target: char) -> Vec<char> {
            ControlsRobot::inner_move_to_and_press(self, target)
        }
    }

    impl<T: Robot> ControlsRobot<T> {
        fn new() -> ControlsRobot<T> {
            let pointed_at = 'A';

            let mut num_to_pos = HashMap::new();
            num_to_pos.insert('<', (0, 0));
            num_to_pos.insert('v', (0, 1));
            num_to_pos.insert('>', (0, 2));
            num_to_pos.insert('^', (1, 1));
            num_to_pos.insert('A', (1, 2));

            Self {
                pointed_at,
                num_to_pos,
                _marker: PhantomData,
            }
        }

        fn inner_move_to_and_press(&mut self, target: char) -> Vec<char> {
            let mut key_into: Vec<char> = vec![];
            let mut cur_pos = *self.num_to_pos.get(&self.pointed_at).unwrap();
            let target_pos = *self.num_to_pos.get(&target).unwrap();

            //Do the first move
            let mut last_move = Self::general_move(&target_pos, &mut cur_pos, &mut key_into);

            let left_pos = *self.num_to_pos.get(&'<').unwrap();
            let up_pos = *self.num_to_pos.get(&'^').unwrap();

            //We should always prioritize doing the last move again if possible (and needed)
            while let Some(char_move) = last_move {
                last_move = match char_move {
                    '>' => {
                        //Robot can always move right
                        if cur_pos.1 < target_pos.1 {
                            cur_pos.1 += 1;
                            key_into.push('>');
                            Some('>')
                        } else {
                            Self::general_move(&target_pos, &mut cur_pos, &mut key_into)
                        }
                    }
                    'v' => {
                        //Robot can always move down
                        if cur_pos.0 > target_pos.0 {
                            cur_pos.0 -= 1;
                            key_into.push('v');
                            Some('v')
                        } else {
                            Self::general_move(&target_pos, &mut cur_pos, &mut key_into)
                        }
                    }
                    '<' => {
                        //Need to also make sure Robot is not pointing at '^'
                        if cur_pos.1 > target_pos.1 && cur_pos != up_pos {
                            //move left
                            cur_pos.1 -= 1;
                            key_into.push('<');
                            Some('<')
                        } else {
                            Self::general_move(&target_pos, &mut cur_pos, &mut key_into)
                        }
                    }
                    '^' => {
                        //Need to also make sure Robot is not pointing at '<'
                        if cur_pos.0 < target_pos.0 && cur_pos != left_pos {
                            //move up
                            cur_pos.0 += 1;
                            key_into.push('^');
                            Some('^')
                        } else {
                            Self::general_move(&target_pos, &mut cur_pos, &mut key_into)
                        }
                    }

                    _ => unreachable!(),
                }
            }

            //We now know Robot is pointing at the target
            self.pointed_at = target;
            key_into.push('A'); //We want Robot to press the button it is pointing at.

            key_into
        }

        ///When we do not need to repeat the last movement (to minimize eventual input length),
        /// do a general guaranteed valid move and return what that move was.
        /// Also ajust key_into and cur_pos as well.
        /// A None is returned when the cur_pos is equal to the target_pos (so no move needed).
        fn general_move(
            target_pos: &(usize, usize),
            cur_pos: &mut (usize, usize),
            key_into: &mut Vec<char>,
        ) -> Option<char> {
            //Note the following acts correctly even when we are at '<' or '^'
            match (cur_pos.0.cmp(&target_pos.0), cur_pos.1.cmp(&target_pos.1)) {
                //The following are stright forward (we have no choice what to do)
                (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => None,

                (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => {
                    //Moving right is always valid
                    cur_pos.1 += 1;
                    key_into.push('>');
                    Some('>')
                }

                (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => {
                    //Note is always valid moving left in this case
                    cur_pos.1 -= 1;
                    key_into.push('<');
                    Some('<')
                }

                (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => {
                    //Moving down is valid
                    cur_pos.0 -= 1;
                    key_into.push('v');
                    Some('v')
                }

                (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => {
                    //Moving up
                    cur_pos.0 += 1;
                    key_into.push('^');
                    Some('^')
                }

                (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => {
                    //go up (traveling from '>' to '^' it is cheaper to go up left than left up)
                    // cur_pos.0 += 1;
                    // key_into.push('^');
                    // Some('^')

                    //Acutally this is better (by testing we found out)
                    cur_pos.1 -= 1;
                    key_into.push('<');
                    Some('<')
                }

                (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => {
                    //traveling from 'A' to 'v' or from '^' to '<' it is cheaper
                    //to go down then left (also from 'A' to '<')

                    //We want to make sure we don't go left down left!
                    if *target_pos == (0, 0) {
                        cur_pos.0 -= 1;
                        key_into.push('v');
                        Some('v')
                    } else {
                        cur_pos.1 -= 1;
                        key_into.push('<');
                        Some('<')
                    }
                }

                (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => {
                    //traveling from '<' to '^' (have to go right then up) or '<' to 'A' (should press right twice so go right then up)
                    // or from 'v' to 'A'(up first 5, right first 5 as well)
                    if *cur_pos == (0, 0) {
                        cur_pos.1 += 1;
                        key_into.push('>');
                        Some('>')
                    } else {
                        cur_pos.0 += 1;
                        key_into.push('^');
                        Some('^')
                    }
                    //Actually the above is better

                    // cur_pos.1 += 1;
                    // key_into.push('>');
                    // Some('>')
                }

                (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => {
                    //traveling from '^' to '>'

                    //going right first always cheaper

                    // cur_pos.1 += 1;
                    // key_into.push('>');
                    // Some('>')

                    //Actually this is better
                    cur_pos.0 -= 1;
                    key_into.push('v');
                    Some('v')
                }
            }
        }
    }

    pub struct CodeHandler {
        numpad: NumKeyPad,
        robot_a_controls: ControlsRobot<A>,
        robot_b_controls: ControlsRobot<B>,
    }

    impl CodeHandler {
        pub fn new() -> CodeHandler {
            let numpad = NumKeyPad::new();
            let robot_a_controls: ControlsRobot<A> = ControlsRobot::new();
            let robot_b_controls: ControlsRobot<B> = ControlsRobot::new();

            Self {
                numpad,
                robot_a_controls,
                robot_b_controls,
            }
        }

        ///Takes the code to input into the *numeric keypad*. Returns the code complexity.
        ///
        /// A code's complexity is
        /// the len_C * (the code stripped of A's and remove leading zeros)
        /// where len_C is the len of a shortest possible input to Robot_C such that code is executed on the
        /// numeric keypad.
        ///
        /// Note that because all codes always end in an A that means all Robots return to their starting positions
        /// after each code is completed. This means one CodeHandler suffices to calc_complexity of multiple codes
        /// one after the other.
        pub fn calc_complexity_part1(&mut self, code: &str) -> usize {
            let code_as_number = code
                .replace('A', "")
                .parse::<usize>()
                .expect("should be a number");

            // So what we do is:
            //
            // take input to numeric keypad (one char at a time) --> input to Robot_A
            // --> input to Robot_B
            // --> input to Robot_C (what we type in) --> calculate the len.

            let mut first = vec![];
            for target in code.chars() {
                first.extend(self.numpad.move_to_and_press(target));
            }

            let mut second = vec![];
            for &target in first.iter() {
                second.extend(self.robot_a_controls.move_to_and_press(target));
            }

            let mut third = vec![];
            for &target in second.iter() {
                third.extend(self.robot_b_controls.move_to_and_press(target));
            }

            // println!(
            //     "\n\n for code {}

            // the first was: \n\n {:#?}

            // the second was: \n\n {:#?}

            // the third was: \n\n {:#?}
            // ",
            //     code, first, second, third
            // );

            (third.len()) * code_as_number
        }

        //Part 2

        //The brute force apporach takes too long. What if instead we simply obsereved
        //how many '<' 'v' '>' '^' 'A' Robot_ generates from each possible move.
        //Then to calculate the next robot input length would simply be a matter of
        //getting the counts from the inputs to robot _ and then doing some multiplications.

        //Look at the following

        //To key into robot B v<<A We need to key into Robot_C <vA<AA>>^A  (Note Robots B and C start and end at A)
        //So we just go over all possiblities (all the ways to travel on Robot A's controls that end
        //in a button push on Robot A's controls so that Robots C is on A and Robot D is on A):

        //Traveling from A to all possible keys and then pressing the key
        //A (going from A to A)
        //<A (going from A to ^)
        //vA (going from A to >)
        //v<A (going from A to v)
        //<vA (going from A to v)
        //v<<A (going from A to <)
        //<v<A (going from A to <)

        //Traveling from ^ to all possible keys and pressing the key
        //A
        //>A
        //vA
        //>vA
        //v>A
        //v<A

        //Traveling from > to all possible keys and pressing the key
        //A
        //^A
        //<A
        //^<A
        //<^A
        //<<A

        //traveling from v to all possible keys and pressing the key
        //A
        //<A
        //>A
        //^A
        //>^A
        //^>A

        //Traveling from < to all possible keys and pressing the key
        //A
        //>A
        //>>A
        //>^A
        //>>^A
        //>^>A

        //Look At the set of all of these and see what they translate to in terms of counts for the input to
        //Robot D

        ///Takes the code to input into the *numeric keypad*. Returns the code complexity.
        ///
        /// The difference from part 1 is that at part 1 we had
        /// keypad <-- RobotA contorls <-- Robot_B controls <-- Robot_C controls <-- We control
        ///
        ///
        /// At part two instead of 2 ControlsRobot<T> instances (i.e. Robot B and C in part 1)
        /// We have 25 instances (26 Robots overall)
        pub fn calc_complexity_part2(&mut self, code: &str) -> usize {
            let code_as_number = code
                .replace('A', "")
                .parse::<usize>()
                .expect("should be a number");

            let movements = [
                "A", "<A", "^A", ">A", "vA", "v<A", "<vA", ">vA", "v>A", "^<A", "<^A", ">^A",
                "^>A", "<<A", ">>A", "v<<A", "<v<A", ">>^A", ">^>A",
            ];

            let mut robot_b: ControlsRobot<A> = ControlsRobot::new();

            //The key is (mov, pattern) and the value is the number of this pattern this mov generates
            //So as doing this mov v<<A spits out v<A<AA>>^A (the input the next Robot needs to execute v<<A).
            //It spits out one v<A one <A one A one >>^A and 0 all other patterns
            let mut dict: HashMap<(&str, &str), usize> = HashMap::with_capacity(40);

            //We need to keep track of how many of each pattern we have in each iteration
            let mut counts: HashMap<&str, usize> = HashMap::with_capacity(20);

            //Key Idea: We know each of these movements generates some combination of the themselves.
            //So we can just keep counts.

            //We know after executing each of these, that Robot C is pointing back at A on Robot B's controls

            //Go over all possible ways to travel on Robot A's controls

            for mov in movements {
                let key = mov;

                let mut output = vec![];

                for target in mov.chars() {
                    output.extend(robot_b.move_to_and_press(target));
                }

                let output: String = output.iter().collect();

                //prints out stuff like The key is v<<A and the output is v<A<AA>>^A
                println!("\n\nThe key is {key} and the output is {output}");

                //Note that doing string replacements on the output is too slow.
                //So we need to keep count of patterns.

                //So for key v<<A it spits out 1 v<A one <A one A one >>^A and 0 all other patterns

                //This is expensive but we only have to do it once. This lets us avoid indexing
                //the HashMap by Strings and use &str instead
                for pattern in output.split_inclusive('A') {
                    for static_pattern in movements {
                        if static_pattern == pattern {
                            dict.entry((key, static_pattern))
                                .and_modify(|e| *e += 1)
                                .or_insert(1);
                        }
                    }
                    if !movements.contains(&pattern) {
                        panic!("Missing pattern");
                    }
                }
            }

            //So now dict is initialized. I also verified manually it is correct

            let mut first = vec![];
            for target in code.chars() {
                first.extend(self.numpad.move_to_and_press(target));
            }

            let mut second = vec![];
            for &target in first.iter() {
                second.extend(self.robot_a_controls.move_to_and_press(target));
            }

            let second: String = second.into_iter().collect();

            //Initialize counts
            for pattern in second.split_inclusive('A') {
                counts.entry(pattern).and_modify(|e| *e += 1).or_insert(1);
            }

            Self::progres_by_robot(&mut counts, &dict, &movements);
            //I tested it and if we did return (Self::calc_len(counts)) * code_as_number here
            //then we would get the same answer as part 1

            //When there were 3 robots, we used 3 function calls.
            //Now there are 26 robots, so use 26 function calls in total. we already used 3
            for _robot_num in 4..=26 {
                Self::progres_by_robot(&mut counts, &dict, &movements);
            }

            (Self::calc_len(counts)) * code_as_number
        }

        ///Takes the counts of each pattern that appear in the input to the current Robot
        /// And a multiplier_dict (for this move and this pattern, this is how many of this pattern this move generates).
        /// Updates counts so that it becomes the counts of each pattern that would appear in the input to the next Robot
        fn progres_by_robot(
            counts: &mut HashMap<&str, usize>,
            dict: &HashMap<(&str, &str), usize>,
            movements_str: &[&'static str; 19],
        ) {
            let mut new_counts: HashMap<&str, usize> = HashMap::with_capacity(20);

            for (mov, &mut old_num) in counts.iter_mut() {
                for ((mov_inner, pattern), &to_add) in dict {
                    //Notice this procedure yields the new counts as old_num of mov is used up
                    // to generate these new patterns
                    if mov == mov_inner {
                        new_counts
                            .entry(pattern)
                            .and_modify(|e| *e += old_num * to_add)
                            .or_insert(old_num * to_add);
                    }
                }
            }

            //We need to do things this way so that we can avoid having to index by Strings (and use &str instead)
            //We need to make counts equal to new_counts
            for mov_str in movements_str {
                if let Some(&new) = new_counts.get(mov_str) {
                    counts.insert(mov_str, new);
                } else {
                    //if mov_str is an entry in counts but is not in new_counts, then we need to set it to 0.
                    counts.insert(mov_str, 0);
                }
            }
        }

        ///Takes the counts of each pattern and returns the sum of (count_of_pattern * pattern_len)
        fn calc_len(counts: HashMap<&str, usize>) -> usize {
            let mut sum = 0;
            for (pattern, count) in counts {
                let len = pattern.chars().count();
                sum += len * count;
            }
            sum
        }
    }
}

///Returns the sum of the complexity score of all codes.
fn solution_part1(file_path: &str) -> usize {
    let mut code_handler = CodeHandler::new();

    fs::read_to_string(file_path)
        .expect("failed to open file")
        .lines()
        .map(|code| code_handler.calc_complexity_part1(code))
        .sum()
}

///Returns the sum of the complexity score of all codes.
fn solution_part2(file_path: &str) -> usize {
    let mut code_handler = CodeHandler::new();

    fs::read_to_string(file_path)
        .expect("failed to open file")
        .lines()
        .map(|code| code_handler.calc_complexity_part2(code))
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn answer() {
        dbg!(solution_part1("puzzle_inputs/day21.txt"));
        dbg!(solution_part2("puzzle_inputs/day21.txt"));
    }

    #[test]
    fn example_part1() {
        let result = solution_part1("puzzle_inputs/day21example.txt");
        assert_eq!(result, 126384);
    }
}
