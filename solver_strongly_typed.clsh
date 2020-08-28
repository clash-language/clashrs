#!/bin/clash
type State<T> {
    value: T
}

class Node {
    # Unless explicitly declared, the following line is created 
    # during the compilation process
    impl Node::init -> Self::init

    impl Self {
        fn init(self,
                state: &State,
                parent: Option<&Node>   = &None, 
                action: Option<&Action> = &None, 
                path_cost: i32          = 0) -> Node 
        { 
            link self.state = state
            link self.parent = parent.unwrap()
            link self.action = action.unwrap()
            prop self.path_cost = path_cost
            prop self.depth = 0
            if parent {
                self.depth = *(self.parent).depth + 1
            }
        } 
        pub fn repr(self) -> String {
            return f"<Node {self.state}>"
        }
        pub fn lt(self, node: Node) -> Bool {
            return self.state < node.state
        }
        pub fn expand(self, problem: Problem) -> List<Node> {
            return [self.child_node(problem, action) for action in problem.actions(self.state)]
        }
        pub fn solution(self) -> List<Action> {
            return [node.action for node in self.path()[1:]]
        }
        pub fn path(self) -> List<Node> {
            node, path_back = self, []
            while node {
                path_back.append(node)
                node = node.parent
            }
            return list(reversed(path_back))
        }
        pub fn eq(self, other: Node) -> Bool {
            return self.state == other
        }
        pub fn hash(self) -> u32 {
            return hash(self.state)
        }
    }
}
impl Interface {
    pub fn actions(prob, state: State) -> Result<Any> 
        {
            raise(NotImplemented)
        }
        pub fn result(prob, state: State, action: Action) -> Result<Any> 
        {
            raise(NotImplemented)
        }
        pub fn path_cost(prob, 
                    c: i32, 
                    state1: State, 
                    action: Action, 
                    state2: State) -> i32 
        {
            return c + 1
        }
        fn value(prob, state) -> Result<Any> 
        {
            raise(NotImplemented)
        }
}
abstract type Problem(Implements=Interface) {

    link initial -> State
    link goal    -> State

    impl Interface;

    impl {
        pub fn goal_test(prob, state: State) -> Bool {
            if [IS List].prob.goal {
                return state in prob.goal
            } else {
                return state == prob.goal
            }
        }
    }
}

class SimpleProblemSolvingAgent {

    # impl SimpleProblemSolvingAgent::init -> Self::init

    impl Self {
        fn init(self, initial_state=None) {
            prop self.state = initial_state
            prop self.seq = []
        }
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


pub fn breadth_first_tree_search(problem: Problem) -> Option<Node> {
    frontier: List<Node<Problem>> = list(Node(problem.initial))
    while (frontier as bool) {
        node = frontier.popleft()
        if problem.goal_test(node.state) {
            return node
        }
        frontier.extend(node.expand(problem))
    }
    return None
}

[AGENT Watchdog(stack_ident=`frontier`, exit_status=`exit_status`, upper_limit=201)]
pub fn watched_breadth_first_tree_search(problem: Problem) -> Option<Node> {    
    exit_status: Option<Node> = None
    frontier: List<Node<Problem>> = list(Node(problem.initial))
    while (frontier as bool) {
        node = frontier.popleft()
        if problem.goal_test(node.state) {
            exit_status = node
            return exit_status
        }
        frontier.extend(node.expand(problem))
    }
    return exit_status
}

agent Watchdog<stack_ident, exit_status, upper_limit>
  where stack_ident: Resolvable,
        exit_status: Resolvable,
        upper_limit:  Int = 999> 
{
    attr {
        enum LogLevel { Info, Warning, Critical }
        readonly link exit_status = exit_status.resolve()
        readonly link stack = stack_ident.resolve()
        readonly prop upper_limit: Int = upper_limit
        $: stack_length: Int = len(stack)
    }
    states {
        Acceptable {
            [WHEN 0 < stack_length < upper_limit],
            [WHEN stack_length == 0],
            [WHEN [IS Node].exit_status]
        },
        RecursionDepthHigh {
            [WHEN 200 <= stack_length < upper_limit]
        },
        exclusive RecursionDepthExceeded {
            [WHEN stack_length >= upper_limit]
        }
        """
        This `state` block desugars to an enum with four slots
            - Acceptable
            - RecursionDepthHigh
            - Acceptable_AND_RecursionDepthHigh
            - RecursionDepthExceeded
        Were `RecursionDepthExceeded` not defined as an 
        `exclusive` state, then there would be three additional
        slots!
            - RecursionDepthExceeded_AND_Acceptable_AND_RecursionDepthHigh
            - RecursionDepthExceeded_AND_Acceptable
            - RecursionDepthExceeded_AND_RecursionDepthHigh
        """
    }
    actions {
        [ON Acceptable] {
            readonly link ll: LogLevel = LogLevel::Info
            Watchdog::log(ll, "The stack size has changed.")
        },
        [ON RecursionDepthHigh] {
            readonly link ll: LogLevel = LogLevel::Warning
            Watchdog::log(ll, "The stack is growing dangerously large.")
        },
        [ON RecursionDepthExceeded] {
            readonly link ll: LogLevel = LogLevel::Critical
            Watchdog::log(ll, "The stack cannot grow any further. Taking drastic measures.")
            Watchdog::bark()
        }
    }
    impl {
        fn log(level: LogLevel, msg: Option<String>) {
            match level {
                LogLevel::Info => write_log(msg.unwrap()),
                LogLevel::Warning => {
                    modified_msg: String = f"Warning: {msg.unwrap()}"
                    write_log(modified_msg)
                },
                LogLevel::Critical => {
                    modified_msg: String = f"Critical Error: {msg.unwrap()}"
                    write_log(modified_msg)
                }
            }
        }
        fn write_log(message: String, logfile: String = "/path/to/log/file.log") {
            with open(logfile, "w+") as file_handler as logger {
                logger.write(message)
            }
        }
        fn bark() {
            # Crash the program since there is an unending while loop
            raise MaxDepthRecursion("Please check the implementation for non-terminating while loops.")
        }
    }
}
use std::traits::Raise;
type MaxDepthRecursion implementing Raise {
    err_msg: String
    impl Raise;
}

enum Action {
    Explore,
    Revert,
    Stay
}


# Design still pending...
agent Reaction {
    """
    An experimental, context-sensitive data structure that
    can toggle its own (readonly) state
    provided that well-defined conditions
    in its scope are precisely defined.
 
    Those familiar with Svelte will recognize this for using 
    the `$:` operator, and under the hood
    uses interior mutability in the style of Rust.
    """
    vars {
        """
        `env` is the collection of strongly-typed
        names that the `ctx` needs to have guaranteed
        read access to. If the lifetime of any of these
        variables does not meet or exceed the lifetime of
        this `ctx`, then the program will fail to compile.
        """
        $: name_of_var -> NameOfVar<Variable>
        $: var_array -> List<NameOfVar<Variable>>, "assigned_name_in_the_equals_statement"
        $: anotherName -> Bool
        $: resultVar -> Result<Output, Error>
        $: arrayOfVars -> NameOfVar<Variable>>, "assigned_name_in_the_equals_statement"

        """
        Variables not marked with `$: name = ...` are 
        assumed to be local to the monitor.
        """
    }
    states {
        Explore {
            # The default state is always the first one
            # ...
            [WHEN (Stay OR DoNothingWhileExploring) AND (NOT Done)]
        },
        exclusive secret Revert {
            [WHEN len(arrayOfVars) != len(var_array)],
            [WHEN other_condition],
            # ...
        },
        exclusive Stay {
            # pass
        },
        DoNothingWhileExploring {},
        exclusive Done {
            # A 'hook' for indicating that one of the `actions` is done running
        }
    }
    actions {
        [ON Explore] {

        },
        [ON Revert] {

        },
        [ON Stay] {
            do_something(VariableType(12))
            [SET Done]
        },
        [On DoNothingWhileExploring] {},
    }
    impl {
        fn do_something(variable: VariableType) -> ExitCode {
            return ExitCode(1)
        }
    }
}

