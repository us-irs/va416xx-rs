target remote localhost:2331

monitor halt

# *try* to stop at the user entry point (it might be gone due to inlining)
break main

load
