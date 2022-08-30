# ballotinator
Automatic room ballotting, written in Rust. 

## functionality
The Ballotinator will be able to read a CSV of group-submitted information and automatically sort the groups into rooms according to their scores and preferences, then output the resulting accommodation setup as a CSV.

Internally, the Ballotinator gives runs through groups in order of average score and gives each group its most preferred available accommodation.

## why?
This is a project I undertook to learn Rust. Python is likely a much more appropriate language for such a project otherwise.
