param ($Day = $(throw "Day parameter is required."))

cargo run | rg "Result ${Day}-"