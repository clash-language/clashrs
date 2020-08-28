#!/bin/clash
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
            [FUNC <generic<T>,generic<F: [FUNC ($_ : `&mut metal::Str`) -> generic<T>]> ($callback) FROM ([callback]=@[0]) -> generic<T>].string
        ]
    ]
    [END INLINE Rust]
]

