#!/bin/clash
"""
This example is ripped from Ben Klemens's "21st Century C"
book.
"""

[
    [INLINE C (bin="/bin/gcc") AS c_mod][
        #include <stdio.h>
        
        long long int fibonacci(){
            static long long int first = 0;
            static long long int second = 1;
            long long int out = first+second;
            first=second;
            second=out;
            return out;
        }
        
        int persistent_fib(){
            for (int i=0; i < 50; i++){
                printf("%lli\n", fibonacci());
            }
        }
    ]
    [EXPORT INLINE C AS c_mod USING][
        [FUNC () -> `persistent_fib::int`].persistent_fib
    ]
    [END INLINE C]
]
