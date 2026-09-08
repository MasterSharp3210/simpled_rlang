# simpled_rlang
simpled_rlang is a common library that makes handling input reading and ANSI escape codes much simpler in Rust.

# How to use
To use this library you have to download the master project and copy the directory next your project. Than, in your main cargo toml file, you have to write this string:
'''[dependencies]
simpled_rlang = { path = "../simpled_rlang" }'''

# In your project
For example, there is a soft syntax: '''let input: u32 = read'''. For the ANSII codes we have for example: '''println("Hello, world!".Bold())'''
