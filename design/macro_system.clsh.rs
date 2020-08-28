#!/bin/clash

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
[END AGENT Watchdog]

[
    [INLINE Rust(bin="/path/to/rustc", mgr="/path/to/cargo") AS metal][
        pub mod metal {
            extern crate use ion;
            type Str = ion::types::Str;
            
            use object_pool::Pool;
            const MAX_SIZE: usize = 64;

            macro_rules! call_and_shrink {
                ($value:ident, $callback:ident) => {{
                    let result = $callback($value);
                    if $value.len() > MAX_SIZE {
                        $value.truncate(MAX_SIZE);
                        $value.shrink_to_fit();
                    }   
                    $value.clear();
                    result
                }}; 
            }

            thread_local! {
                static STRINGS: Pool<Str> = Pool::new(256, || Str::with_capacity(MAX_SIZE));
            }

            pub struct IonPool;

            impl IonPool {
                pub fn string<T, F: FnMut(&mut Str) -> T>(mut callback: F) -> T { 
                    STRINGS.with(|pool| match pool.pull() {
                        Some(ref mut string) => call_and_shrink!(string, callback),
                        None => callback(&mut Str::new()),
                    })  
                }   
            }
        }
    ]
    [EXPORT INLINE Rust AS metal USING][
        [TYPE `metal::Str`].Str,
        [TYPE `metal::Pool`].Pool,
        [TYPE `metal::uszie`].MAX_SIZE,
        [META ($value, $callback) FROM ([value]=@[0] [callback]=@[1])].call_and_shrink,
        [FUNC () -> `metal::Pool<metal::usize || metal::Str>`].thread_local,
        [TYPE `metal::IonPool`].IonPool => [
            [FUNC <generic<T>, generic<F: [FUNC ($_ : `&mut metal::Str`) -> generic<T>]>
                ($callback) FROM ([callback]=@[0]) -> generic<T>].string
        ]
    ]
    [END INLINE Rust]
]

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

