# ballotinator
Automatic room ballotting, written in Rust. 

## functionality
The Ballotinator will be able to read a CSV of group-submitted information and automatically sort the groups into rooms according to their scores and preferences, then output the resulting accommodation setup as a CSV.

Internally, the Ballotinator gives runs through groups in order of average score and gives each group its most preferred available accommodation.

## why?
Room balloting is a stressful week-long extravaganza, and the thought of rooms being assigned by algorithm to (hopefully optimally) simulate your week's worth of careful ballotting seemed very appealing during it. As I wanted to learn Rust anyways, this provided an interesting challenge to guide my learning.

Note that I implemented this in Rust so I could learn the language. Python is likely a much more appropriate language for such a project otherwise, as it's arguably faster to develop smaller projects like this in.
