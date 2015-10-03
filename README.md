# dining-philosophers-rust

_Dining Philosophers_ is one of the projects included in [The Rust Programming Language](https://doc.rust-lang.org/book/) book.

This classic concurrency problem was originally conceived by Dijkstra in 1965, but here we'll use a lightly adapted version from [this book](http://www.usingcsp.com/cspbook.pdf) by Tony Hoare in 1985 (_The Dining Philosophers_ example can be found at page 55).

> In ancient times, a wealthy philanthropist endowed a College to accommodate five eminent philosophers. Each philosopher had a room in which he could engage in his professional activity of thinking; there was also a common dining room, furnished with a circular table, surrounded by five chairs, each labelled by the name of the philosopher who was to sit in it. They sat anticlockwise around the table. To the left of each philosopher there was laid a golden fork, and in the centre stood a large bowl of spaghetti, which was constantly replenished. A philosopher was expected to spend most of his time thinking; but when he felt hungry, he went to the dining room, sat down in his own chair, picked up his own fork on his left, and plunged it into the spaghetti. But such is the tangled nature of spaghetti that a second fork is required to carry it to the mouth. The philosopher therefore had also to pick up the fork on his right. When he was finished he would put down both his forks, get up from his chair, and continue thinking. Of course, a fork can be used by only one philosopher at a time. If the other philosopher wants it, he just has to wait until the fork is available again.

This problem shows off a few different elements of concurrency. The reason is that it's actually slightly tricky to implement: a simple implementation can deadlock.
