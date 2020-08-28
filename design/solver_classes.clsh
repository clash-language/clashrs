#!/bin/clash
class Node {
    pub fn init(self, state, parent=None, action=None, path_cost=0) { 
        prop self.state = state
        prop self.parent = parent
        prop self.action = action
        prop self.path_cost = path_cost
        prop self.depth = 0
        if parent {
            self.depth = self.parent.depth + 1
        }
    }

    impl Self {
        # There is a useful semantic here. `impl`, when inside of a class,
        # the first parameter will always be the type of `Self`
        pub fn repr(self) {
            return f"<Node {self.state}>"
        }

        pub fn lt(self, node) {
            return self.state < node.state
        }
        pub fn expand(self, problem) {
            return [self.child_node(problem, action) for action in problem.actions(self.state)]
        }
        pub fn solution(self) {
            return [node.action for node in self.path()[1:]]
        }
        pub fn path(self) {
            node, path_back = self, []
            while node {
                path_back.append(node)
                node = node.parent
            }
            return list(reversed(path_back))
        }
        pub fn eq(self, other) -> Bool {
            return [IS Node].other and self.state == other
        }
        pub fn hash(self) {
            return hash(self.state)
        }
    }
}

class Problem {
    pub fn init(self, initial, goal=None) {
        prop self.initial = initial
        prop self.goal = goal
    }
    impl Self {
        pub fn actions(self, state) {
            raise(NotImplemented)
        }
        pub fn result(self, state, action) {
            raise(NotImplemented)
        }
        pub fn goal_test(self, state) {
            if [IS List].self.goal {
                return state in self.goal
            } else {
                return state == self.goal
            }
        }
        pub fn path_cost(self, c, state1, action, state2) {
            # Definitely override this, otherwise:
            return c + 1
        }
        pub fn value(self, state) {
            raise(NotImplemented)
        }
    }
}

class SimpleProblemSolvingAgent {
    pub fn init(self, initial_state=None) {
        prop self.state = initial_state
        prop self.seq = []
    }
    impl Self {
        pub fn call(self, percept) {
            self.state = self.update_state(self.state, percept)
            if not self.seq {
                goal = self.formulate_goal(self.state)
                problem = self.formulate_problem(self.state, goal)
                self.seq = self.search(problem)
                if not self.seq {
                    return None
                }
            }
            return self.seq.pop()
        }
        fn update_state(self, state, percept) {
            raise(NotImplemented)
        }
        fn formulate_goal(self, state) {
            raise(NotImplemented)
        }
        fn search(self, problem) {
            raise(NotImplemented)
        }
    }
}

pub fn breadth_first_tree_search(problem) {
    frontier = list(Node(problem.initial))
    while frontier {
        node = frontier.popleft()
        if problem.goal_test(node.state) {
            return node
        }
        frontier.extend(node.expand(problem))
    }
    return None
}











