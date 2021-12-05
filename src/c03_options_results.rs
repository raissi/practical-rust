

fn sum(s: &String) -> Result<i32, String>{

    let mut sum = 0;
    for c in s.chars() {
        let cc = c.to_digit(10);
        if cc.is_none() {
            return Err(format!("Encountered illegal char: {}", c))
        }
        sum += cc.unwrap() as i32;
    }
    Ok(sum)

}

fn prod_sums(a: &String, b: &String) -> Result<i32, String>{
    let s_a = sum(a)?;
    let s_b = sum(b)?;
    Ok(s_a * s_b)
}

#[cfg(test)]
mod tests {
    use super::*;

   

    #[test]
    fn compue_sum() {
        let s = sum(&String::from("123"));
        assert_eq!(s, Ok(6));

        let e = sum(&String::from("12!3"));
        assert_eq!(true, e.is_err());
    }

    #[test]
    fn compue_prods() {
        let s = prod_sums(&String::from("3"), &String::from("4"));
        assert_eq!(s, Ok(12));

        let e = prod_sums(&String::from("3O"), &String::from("10"));
        assert_eq!(true, e.is_err());
    }

}