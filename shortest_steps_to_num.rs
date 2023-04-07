fn shortest_steps_to_num (n: u16) -> u16 {
    if n==1 {0} else {1+shortest_steps_to_num(if n%2==0 {n/2} else {n-1})}
}