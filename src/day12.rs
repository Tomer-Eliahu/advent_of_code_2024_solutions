use std::fs;

//Part 1 solution notes:
//The area of a region is the number of plots in it.
//The perimeter of a region is the number of sides of garden plots
//in the region that do not touch another garden plot in the same region.

//Replace letters we have gone over by a Char we will take as a flag to ignore.
//Iteratre over the map, have a function given a location, that returns the price of a region.
//This function must also remove said region from the map (so we avoid repeated work).
//This function takes a region and calculates its area and perimeter (and multiplies them to get the price).
//We do this by breadth-first search. Before starting the search we need to hold
//The type of plot (the char), number of plots count (area) and sum of perimeter so far.

pub mod garden_map {

    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::collections::VecDeque;
    use std::sync::Mutex;
    use std::sync::OnceLock;

    //A helper struct for part 2
    #[derive(Clone, Copy)]
    struct CanLook {
        up: bool,
        down: bool,
        right: bool,
        left: bool,
    }

    impl CanLook {
        fn get_up(&self) -> bool {
            self.up
        }
        fn get_down(&self) -> bool {
            self.down
        }
        fn get_left(&self) -> bool {
            self.left
        }
        fn get_right(&self) -> bool {
            self.right
        }
    }

    impl Default for CanLook {
        fn default() -> Self {
            CanLook {
                up: true,
                down: true,
                right: true,
                left: true,
            }
        }
    }

    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    ///A Matrix holds a vector of vectors of something
    pub struct Matrix<T> {
        data: Vec<Vec<T>>,
        row_num: usize,
        col_num: usize,
    }

    impl<T> Matrix<T> {
        //Since the data is guaranteed to be rectanguler we can deduce row_num and col_num from data
        fn new(data: Vec<Vec<T>>) -> Result<Matrix<T>, &'static str> {
            if data.is_empty() {
                return Err("empty input");
            }

            let row_num = data.len();
            let col_num = data[0].len();

            Ok(Matrix {
                data,
                row_num,
                col_num,
            })
        }
    }

    pub type Map = Matrix<char>;

    impl Map {
        pub fn build(map: Vec<Vec<char>>) -> Map {
            Matrix::new(map).expect("The garden should not be empty")
        }

        ///Returns the total price of fencing of all regions on the map.
        ///If discount is true (part 2 of the puzzle) it returns the price ***now with bulk discount***.
        pub fn price_map(mut self, discount: bool) -> usize {
            let price_function = if discount {
                Self::correct_price_region_with_discount
            } else {
                Self::price_region
            };

            let mut price = 0;

            //We do this to avoid unsafe code
            let (rows, cols) = (self.row_num, self.col_num);

            let map:&mut Matrix<char> = &mut self;
            
            //We find the price of each individual region and sum their prices
            for row_index in 0..rows{
                for col_index in 0..cols  {
                    if map.data[row_index][col_index] != '.' {
                        price += price_function(map, (row_index, col_index));
                    }
                }
            }

            price
        }

        ///Returns the price of a region and crosses it off the map (by replacing its plots with '.').
        fn price_region(&mut self, start_location: (usize, usize)) -> usize {
            let plot_type = self.data[start_location.0][start_location.1];

            //The perimeter of a region is the number of sides of garden plots
            //in the region that do not touch another garden plot in the same region
            //(i.e. a garden plot with the same type as plot_type).
            let mut perimeter = 0;

            //We do Breadth-first search.
            let mut queue = VecDeque::new();

            let mut explored = HashSet::new();

            queue.push_back(start_location);

            //Using explored allows us to avoid visiting nodes repeatdly.
            //Note that the length of explored after the search is the area of this region
            //(as explored will be a set of all locations in this region)
            explored.insert(start_location);

            while let Some(location) = queue.pop_front() {
                //We could cross out this location on the map now but then we also have to check
                //that every plot we encounter that is not the plot_type is not '.' to avoid
                //unneccessary additions to the perimeter.
                //Instead we iterate on explored below.
                //So we don't do this: self.data[location.0][location.1] = '.';

                //see if one space up, down, left, or right from the location
                //has the same plot_type, if so enqueue it.

                //Look up. Note this is safe as && short-circuits in Rust
                //(the RHS is evaluated only if  location.0  > 0 is true).
                if location.0 > 0 && self.data[location.0 - 1][location.1] == plot_type {
                    if explored.insert((location.0 - 1, location.1)) {
                        queue.push_back((location.0 - 1, location.1));
                    }

                    //Note we do not add 1 to the perimeter here.
                } else {
                    perimeter += 1;
                }

                //Look down
                if location.0 + 1 < self.row_num
                    && self.data[location.0 + 1][location.1] == plot_type
                {
                    if explored.insert((location.0 + 1, location.1)) {
                        queue.push_back((location.0 + 1, location.1));
                    }
                } else {
                    perimeter += 1;
                }

                //Look Right
                if location.1 + 1 < self.col_num
                    && self.data[location.0][location.1 + 1] == plot_type
                {
                    if explored.insert((location.0, location.1 + 1)) {
                        queue.push_back((location.0, location.1 + 1));
                    }
                } else {
                    perimeter += 1;
                }

                //Look Left
                if location.1 > 0 && self.data[location.0][location.1 - 1] == plot_type {
                    if explored.insert((location.0, location.1 - 1)) {
                        queue.push_back((location.0, location.1 - 1));
                    }
                } else {
                    perimeter += 1;
                }
            }

            //we only want to print this out when we run small examples.
            //This whole block conditionally complies.
            // #[cfg(test)]
            // {
            //     println!("For plot type {} the area is {} and the permieter is {}", plot_type, explored.len(), perimeter);
            //     println!("the map before crossing out this region is: {:#?}", self.data);
            // }

            //cross off this region from the map and return the price
            let area = explored
                .into_iter()
                .map(|(row, col)| self.data[row][col] = '.')
                .count();

            area * perimeter
        }

        ///Returns the *bulk discounted* price
        ///  of a region and crosses it off the map (by replacing its plots with '.').
        ///
        ///
        ///Idea:
        ///We do Depth-First Search but this time
        ///When we can't look right (add +1 to side_count)
        ///go down the map untill you can (hit a node that can look right---> but we don't enqueue it)
        ///disabling all nodes in the way ability to look right themselvs!!
        ///Do the same going up.
        ///
        ///so for each node we need to keep track of directions it can look
        ///at the start -- all nodes can look in all directions.
        ///
        ///The idea is basically to mark the sides on the map (I really wish I did this idea first :),
        ///it is so much easier to understand and code and also fully correct.
        fn correct_price_region_with_discount(&mut self, start_location: (usize, usize)) -> usize {
            //Note this line is executed as if it was defined in this module (so only executed once)
            //This is because "The static initializer is a constant expression evaluated at compile time"
            //(from the Rust reference).
            static CELL: Mutex<OnceLock<HashMap<(usize, usize), CanLook>>> =
                Mutex::new(OnceLock::new());

            //Note we only need to do this once for all regions (not once per region; because no plot is in 2 regions)
            //so we utilize statics.
            //we utilize Mutex to avoid an unsafe block (we needed a thing that implements Sync so that it can be
            //a static and that provides interior mutability).

            CELL.lock()
                .expect("Our program is single threaded")
                .get_or_init(|| {
                    let mut nodes: HashMap<(usize, usize), CanLook> = HashMap::new();

                    for (row_index, row) in self.data.iter().enumerate() {
                        for (col_index, _) in row.iter().enumerate() {
                            //all nodes start with being able to look in all directions
                            nodes.insert((row_index, col_index), CanLook::default());
                        }
                    }
                    nodes
                });

            let mut mutex_guard = CELL.lock().expect("Our program is single threaded");

            let nodes = mutex_guard.get_mut().expect(
                "The cell should not be empty as we have always initialized it before this call",
            );

            //DFS
            let mut stack = vec![start_location];
            let mut explored = HashSet::new();

            let plot_type = self.data[start_location.0][start_location.1];

            let mut sides_count = 0;

            explored.insert(start_location);

            while let Some(location) = stack.pop() {
                //see if one space up, down, left, or right from the location
                //has the same plot_type.
                let look_up = location.0 > 0 && self.data[location.0 - 1][location.1] == plot_type;

                let look_down = location.0 + 1 < self.row_num
                    && self.data[location.0 + 1][location.1] == plot_type;

                let look_right = location.1 + 1 < self.col_num
                    && self.data[location.0][location.1 + 1] == plot_type;

                let look_left =
                    location.1 > 0 && self.data[location.0][location.1 - 1] == plot_type;

                //we copy CanLook of the current node because we need to reborrow nodes mutably later
                let can_look = *nodes
                    .get(&location)
                    .expect("All nodes should have been inserted by now");

                if !look_up {
                    //We found a boundary looking up
                    if can_look.up {
                        //meaning this is a new boundary
                        sides_count += 1;

                        //update can look up for this node and all nodes to the left and right of it to false
                        //Interesting note: nodes is a &mut T which does not implement copy. Yet nodes is not being
                        //moved by this function call as Rust reborrows it. i.e. instead of nodes what Rust does is
                        //nodes_look = &mut *nodes (which is a new mutable reference on the underlying data of nodes;
                        //that new reference is moved; after the function call it is freeded and we can use nodes again)
                        self.update_looks(nodes, location, Direction::Up);
                    }
                } else if explored.insert((location.0 - 1, location.1)) {
                    //We found another member of this region. Enqueue it if this is the first time we encountered it
                    stack.push((location.0 - 1, location.1));
                }

                if !look_down {
                    if can_look.down {
                        sides_count += 1;
                        self.update_looks(nodes, location, Direction::Down);
                    }
                } else if explored.insert((location.0 + 1, location.1)) {
                    //We found another member of this region. Enqueue it if this is the first time we encountered it
                    stack.push((location.0 + 1, location.1));
                }

                if !look_right {
                    if can_look.right {
                        sides_count += 1;
                        self.update_looks(nodes, location, Direction::Right);
                    }
                } else if explored.insert((location.0, location.1 + 1)) {
                    //We found another member of this region. Enqueue it if this is the first time we encountered it
                    stack.push((location.0, location.1 + 1));
                }

                if !look_left {
                    if can_look.left {
                        sides_count += 1;
                        self.update_looks(nodes, location, Direction::Left);
                    }
                } else if explored.insert((location.0, location.1 - 1)) {
                    //We found another member of this region. Enqueue it if this is the first time we encountered it
                    stack.push((location.0, location.1 - 1));
                }
            }

            //we only want to print this out when we run small examples.
            // #[cfg(test)]
            // {
            //     println!(
            //         "For plot type {} the area is {} and the sides_count is {}",
            //         plot_type,
            //         explored.len(),
            //         sides_count
            //     );
            //     println!("the map before crossing out this region is: {:#?}", self.data);
            // }

            //cross off this region from the map and return the price
            let area = explored
                .into_iter()
                .map(|(row, col)| self.data[row][col] = '.')
                .count();

            area * sides_count
        }

        ///Travel only on nodes that have the same plot type as the plot type in start_location.
        /// Depending on direction--> go in Perpendicular directions updating all nodes (of the same type) that we travel on
        /// to be nodes that can't look in the value of direction parameter.
        ///
        /// For example update looks(.. start_location = (0,1) <-- Note the value of col is 1,  direction= Up)
        /// for map:
        /// A A A A B
        /// B B B B B
        ///
        /// will update all the A's to say they can no longer look up (note the top right B is left untouched).
        /// In this case we travel both left from the starting location as well as right.
        fn update_looks(
            &self,
            nodes_look: &mut HashMap<(usize, usize), CanLook>,
            start_location: (usize, usize),
            direction: Direction,
        ) {
            let plot_type = self.data[start_location.0][start_location.1];

            match direction {
                Direction::Up => {
                    //update this starting node can no longer look up (we make sure we avoid double counting)

                    nodes_look.get_mut(&start_location).unwrap().up = false;

                    //update can look up for everything to the left to false until we hit a node that doesn't have
                    //an up boundary
                    //Note we are careful to only travel on nodes that have the same type as us
                    if start_location.1 > 0
                        && self.data[start_location.0][start_location.1 - 1] == plot_type
                    {
                        let mut current_location = (start_location.0, start_location.1 - 1);

                        //check if current_location has an up boundary
                        while !(current_location.0 > 0
                            && self.data[current_location.0 - 1][current_location.1] == plot_type)
                        {
                            nodes_look.get_mut(&current_location).unwrap().up = false;

                            if current_location.1 > 0
                                && self.data[current_location.0][current_location.1 - 1]
                                    == plot_type
                            {
                                current_location = (current_location.0, current_location.1 - 1);
                            } else {
                                break;
                            }
                        }
                    }

                    //update can look up for everything to the right to false
                    if start_location.1 + 1 < self.col_num
                        && self.data[start_location.0][start_location.1 + 1] == plot_type
                    {
                        let mut current_location = (start_location.0, start_location.1 + 1);

                        //check if current_location has an up boundary
                        while !(current_location.0 > 0
                            && self.data[current_location.0 - 1][current_location.1] == plot_type)
                        {
                            nodes_look.get_mut(&current_location).unwrap().up = false;

                            if current_location.1 + 1 < self.col_num
                                && self.data[current_location.0][current_location.1 + 1]
                                    == plot_type
                            {
                                current_location = (current_location.0, current_location.1 + 1);
                            } else {
                                break;
                            }
                        }
                    }
                }
                Direction::Down => {
                    nodes_look.get_mut(&start_location).unwrap().down = false;

                    if start_location.1 > 0
                        && self.data[start_location.0][start_location.1 - 1] == plot_type
                    {
                        let mut current_location = (start_location.0, start_location.1 - 1);

                        while !(current_location.0 + 1 < self.row_num
                            && self.data[current_location.0 + 1][current_location.1] == plot_type)
                        {
                            nodes_look.get_mut(&current_location).unwrap().down = false;

                            if current_location.1 > 0
                                && self.data[current_location.0][current_location.1 - 1]
                                    == plot_type
                            {
                                current_location = (current_location.0, current_location.1 - 1);
                            } else {
                                break;
                            }
                        }
                    }

                    if start_location.1 + 1 < self.col_num
                        && self.data[start_location.0][start_location.1 + 1] == plot_type
                    {
                        let mut current_location = (start_location.0, start_location.1 + 1);

                        while !(current_location.0 + 1 < self.row_num
                            && self.data[current_location.0 + 1][current_location.1] == plot_type)
                        {
                            nodes_look.get_mut(&current_location).unwrap().down = false;

                            if current_location.1 + 1 < self.col_num
                                && self.data[current_location.0][current_location.1 + 1]
                                    == plot_type
                            {
                                current_location = (current_location.0, current_location.1 + 1);
                            } else {
                                break;
                            }
                        }
                    }
                }
                Direction::Left => {
                    nodes_look.get_mut(&start_location).unwrap().left = false;

                    if start_location.0 > 0
                        && self.data[start_location.0 - 1][start_location.1] == plot_type
                    {
                        let mut current_location = (start_location.0 - 1, start_location.1);

                        while !(current_location.1 > 0
                            && self.data[current_location.0][current_location.1 - 1] == plot_type)
                        {
                            nodes_look.get_mut(&current_location).unwrap().left = false;

                            if current_location.0 > 0
                                && self.data[current_location.0 - 1][current_location.1]
                                    == plot_type
                            {
                                current_location = (current_location.0 - 1, current_location.1);
                            } else {
                                break;
                            }
                        }
                    }

                    if start_location.0 + 1 < self.row_num
                        && self.data[start_location.0 + 1][start_location.1] == plot_type
                    {
                        let mut current_location = (start_location.0 + 1, start_location.1);

                        while !(current_location.1 > 0
                            && self.data[current_location.0][current_location.1 - 1] == plot_type)
                        {
                            nodes_look.get_mut(&current_location).unwrap().left = false;

                            if current_location.0 + 1 < self.row_num
                                && self.data[current_location.0 + 1][current_location.1]
                                    == plot_type
                            {
                                current_location = (current_location.0 + 1, current_location.1);
                            } else {
                                break;
                            }
                        }
                    }
                }
                Direction::Right => {
                    nodes_look.get_mut(&start_location).unwrap().right = false;

                    if start_location.0 > 0
                        && self.data[start_location.0 - 1][start_location.1] == plot_type
                    {
                        let mut current_location = (start_location.0 - 1, start_location.1);

                        while !(current_location.1 + 1 < self.col_num
                            && self.data[current_location.0][current_location.1 + 1] == plot_type)
                        {
                            nodes_look.get_mut(&current_location).unwrap().right = false;

                            if current_location.0 > 0
                                && self.data[current_location.0 - 1][start_location.1] == plot_type
                            {
                                current_location = (current_location.0 - 1, current_location.1);
                            } else {
                                break;
                            }
                        }
                    }

                    if start_location.0 + 1 < self.row_num
                        && self.data[start_location.0 + 1][start_location.1] == plot_type
                    {
                        let mut current_location = (start_location.0 + 1, start_location.1);

                        while !(current_location.1 + 1 < self.col_num
                            && self.data[current_location.0][current_location.1 + 1] == plot_type)
                        {
                            nodes_look.get_mut(&current_location).unwrap().right = false;

                            if current_location.0 + 1 < self.row_num
                                && self.data[current_location.0 + 1][current_location.1]
                                    == plot_type
                            {
                                current_location = (current_location.0 + 1, current_location.1);
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }

        //-----------------Incorrect algorithm that happens to give the correct solution and passes the example puzzle test

        ///Returns the *bulk discounted* price
        ///  of a region and crosses it off the map (by replacing its plots with '.').
        ///
        ///Some notes:
        ///
        ///Doing Breadth-first search fails in cases like:
        ///
        ///      Start at top-left A
        ///
        ///
        ///                  *   
        ///      A B B B B B A A A A A A
        ///      A B B B B B A B B B B A <--we are here
        ///      A B B B B B A B B B B A <--not explored yet
        ///      A A A A A A A B B B B A
        ///      B B B B B B A B B B B A
        ///      B B B B B B A B B B B A <-- on the queue
        ///      B B B B B B A A A A A A
        ///
        ///
        ///
        ///This A region has 14 sides but our algorithm below reports it as having 16 sides (it double counts
        ///the outer and inner side of the right-most col of A's).
        ///    So while our algorithm below happens to pass the puzzle input, it is not actually a solution!
        ///
        ///To avoid double counting the left side of the row of right-most A's
        ///    We must come up with a different method than this.
        ///
        ///Note that Naive Depth-First Search (replacing queue.push_back with queue.push_front below)
        ///also fails this (reports 15 sides: it double counts the inner side of the col with a star on top).
        ///
        ///So to recap:
        ///    This function is an incorrect algorithm
        ///    that fails the above test case but happens to pass the puzzle!
        fn wrong_price_region_with_discount(&mut self, start_location: (usize, usize)) -> usize {
            let plot_type = self.data[start_location.0][start_location.1];

            let mut sides_count = 0;

            let mut queue = VecDeque::new();

            let mut explored: HashMap<Option<(usize, usize)>, CanLook> = HashMap::new();

            queue.push_back(start_location);

            while let Some(location) = queue.pop_front() {
                //if we already explored this location, skip this iteration
                if explored.contains_key(&Some(location)) {
                    continue;
                }

                //see if one space up, down, left, or right from the location
                //has the same plot_type.
                let look_up = location.0 > 0 && self.data[location.0 - 1][location.1] == plot_type;

                let look_down = location.0 + 1 < self.row_num
                    && self.data[location.0 + 1][location.1] == plot_type;

                let look_right = location.1 + 1 < self.col_num
                    && self.data[location.0][location.1 + 1] == plot_type;

                let look_left =
                    location.1 > 0 && self.data[location.0][location.1 - 1] == plot_type;

                //neighbours locations (None if they don't exist or are not the same plot_type)
                let up = if look_up {
                    Some((location.0 - 1, location.1))
                } else {
                    None
                };
                let down = if look_down {
                    Some((location.0 + 1, location.1))
                } else {
                    None
                };
                let right = if look_right {
                    Some((location.0, location.1 + 1))
                } else {
                    None
                };
                let left = if look_left {
                    Some((location.0, location.1 - 1))
                } else {
                    None
                };

                if !look_up {
                    //check left and right neighbours and see if they also have a boundary when looking up
                    Map::check_neighbours(
                        look_left,
                        look_right,
                        left,
                        right,
                        &mut sides_count,
                        &explored,
                        Direction::Up,
                    );
                } else if !explored.contains_key(&up) {
                    //We found another member of this region. Enqueue it if this is the first time we encountered it
                    queue.push_back((location.0 - 1, location.1));
                    //We also must mark it as explored but we can't insert it to explored yet
                    //because we need to know the values of look_directions for this new member

                    //AB
                    //AB
                    //^
                    //|
                    //we know the values for this A but not the top A (the new member) yet.
                    //So we just queue it.

                    //This is why we added the check for when we pop off the queue
                    //that the element has not been explored yet (because we now allow the queue to potentially
                    //have repeated elements).
                }

                //Look down
                if !look_down {
                    //check left and right neighbours and see if they also have a boundary when looking down
                    Map::check_neighbours(
                        look_left,
                        look_right,
                        left,
                        right,
                        &mut sides_count,
                        &explored,
                        Direction::Down,
                    );
                } else if !explored.contains_key(&down) {
                    queue.push_back((location.0 + 1, location.1));
                }

                //Look Right
                if !look_right {
                    //check up and down neighbours and see if they also have a boundary when looking right
                    Map::check_neighbours(
                        look_up,
                        look_down,
                        up,
                        down,
                        &mut sides_count,
                        &explored,
                        Direction::Right,
                    );
                } else if !explored.contains_key(&right) {
                    queue.push_back((location.0, location.1 + 1));
                }

                //Look Left
                if !look_left {
                    //check up and down neighbours and see if they also have a boundary when looking right
                    Map::check_neighbours(
                        look_up,
                        look_down,
                        up,
                        down,
                        &mut sides_count,
                        &explored,
                        Direction::Left,
                    );
                } else if !explored.contains_key(&left) {
                    queue.push_back((location.0, location.1 - 1));
                }

                //Using explored allows us to avoid doing repeated work on the same nodes.
                //Note that the length of explored after the search is the area of this region
                //(as explored will be a set of all locations in this region)
                explored.insert(
                    Some(location),
                    CanLook {
                        up: look_up,
                        down: look_down,
                        right: look_right,
                        left: look_left,
                    },
                );
            }

            //we only want to print this out when we run small examples.
            // #[cfg(test)]
            // {

            //     println!(
            //         "For plot type {} the area is {} and the sides_count is {}",
            //         plot_type,
            //         explored.len(),
            //         sides_count
            //     );
            //     println!("the map before crossing out this region is: {:#?}", self.data);
            // }

            //cross off this region from the map and return the price
            let area = explored
                .into_keys()
                .map(|key| {
                    let (row, col) = key.expect("All locations in explored should exist");
                    self.data[row][col] = '.'
                })
                .count();

            area * sides_count
        }

        ///Check neighbours a and b of a location and update the sides count accordingly
        fn check_neighbours(
            look_a: bool,
            look_b: bool,
            location_a: Option<(usize, usize)>,
            location_b: Option<(usize, usize)>,
            sides_count: &mut usize,
            explored: &HashMap<Option<(usize, usize)>, CanLook>,
            get_direction: Direction,
        ) {
            //look at left and right neighbours that have the same plot_type
            //Or look at up and down neighbours that have the same plot_type

            let get = match get_direction {
                Direction::Up => CanLook::get_up,
                Direction::Down => CanLook::get_down,
                Direction::Left => CanLook::get_left,
                Direction::Right => CanLook::get_right,
            };

            match (look_a, look_b) {
                (true, true) => match (explored.get(&location_a), explored.get(&location_b)) {
                    (None, None) => {
                        *sides_count += 1;
                    }
                    (None, Some(info)) => {
                        if get(info) {
                            *sides_count += 1;
                        }
                    }
                    (Some(info), None) => {
                        if get(info) {
                            *sides_count += 1;
                        }
                    }
                    (Some(a), Some(b)) => {
                        if get(a) && get(b) {
                            *sides_count += 1;
                        }
                    }
                },
                (true, false) => {
                    if let Some(info) = explored.get(&location_a) {
                        if get(info) {
                            *sides_count += 1
                        }
                    } else {
                        *sides_count += 1;
                    }
                }
                (false, true) => {
                    if let Some(info) = explored.get(&location_b) {
                        if get(info) {
                            *sides_count += 1
                        }
                    } else {
                        *sides_count += 1;
                    }
                }
                (false, false) => {
                    *sides_count += 1;
                }
            }
        }

        //-----------------Incorrect algorithm that happens to give the correct solution and passes the example puzzle test
    }
}

fn solution_part1(file_path: &str) -> usize {
    let data: Vec<_> = fs::read_to_string(file_path)
        .expect("failed to open file")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let map = garden_map::Map::build(data);
    map.price_map(false)
}

fn solution_part2(file_path: &str) -> usize {
    let data: Vec<_> = fs::read_to_string(file_path)
        .expect("failed to open file")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let map = garden_map::Map::build(data);
    map.price_map(true)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn answer() {
        dbg!(solution_part1("puzzle_inputs/day12.txt"));
        dbg!(solution_part2("puzzle_inputs/day12.txt"));
    }

    #[test]
    fn example_part1() {
        let result = solution_part1("puzzle_inputs/day12example.txt");
        assert_eq!(result, 1930);
    }

    #[test]
    fn example_part2() {
        let result = solution_part2("puzzle_inputs/day12example.txt");
        assert_eq!(result, 1206);
    }

    #[test]
    fn my_own_test() {
        //testing this:
        //      A B B B B B A A A A A A
        //      A B B B B B A B B B B A
        //      A B B B B B A B B B B A
        //      A A A A A A A B B B B A
        //      B B B B B B A B B B B A
        //      B B B B B B A B B B B A
        //      B B B B B B A A A A A A
        solution_part2("puzzle_inputs/day12myowntest.txt");
        //Check the region with plot type A has 14 sides and not 16 or 15.
    }
}
