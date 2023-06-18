fn alphabet_war(s: &str) -> &'static str {
    let a = s.chars().enumerate().map(|(i,e)| if s.chars().nth(i.saturating_sub(1)).unwrap_or(' ')=='*' || s.chars().nth(i+1).unwrap_or(' ')=='*' {' '} else {e}).collect::<Vec<_>>();
    let x = a.iter().map(|e| match e {
        'w' => 4, 'p' => 3, 'b' => 2, 's' => 1, _ => 0
    }).sum::<i32>();    
    let y = a.iter().map(|e| match e {
        'm' => 4, 'q' => 3, 'd' => 2, 'z' => 1, _ => 0
    }).sum::<i32>();
    if x==y {"Let's fight again!"} else if x>y {"Left side wins!"} else {"Right side wins!"}
}