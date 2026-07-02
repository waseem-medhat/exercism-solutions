let leap_year year = 
    let divisible div =
        year mod div = 0
    in
    (divisible 400) || (not (divisible 100) && (divisible 4))