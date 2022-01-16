#[cfg(test)]
mod tests {
    use alignment::alignment::Alignment;

    #[test]
    fn new_test() {
        let first = String::from("toAlign");
        let second = String::from("to Align");
        let alignment = Alignment::new(&first, &second);
        
        // check first column is initialized
        for i in 0..first.chars().count() as i64 + 1 {
            assert_eq!(i * -1, alignment.matrix[i as usize][0].value);
        }

        // check first row is initialized
        for i in 1..second.chars().count() as i64 + 1 {
            assert_eq!(i * -1, alignment.matrix[0][i as usize].value);
        }
    }

    #[test]
    fn calculate_score_test() {
        let first = String::from("toAlign");
        let second = String::from("to Align");
        let mut alignment = Alignment::new(&first, &second);

        match alignment.calculate_score() {
            Ok(()) => (),
            Err(_) => panic!("Failed to calculate Score"),
        };
    }

    #[test]
    fn align_test() {
        let first = String::from("toAlign some stuff");
        let second = String::from("to Align and aStguff");
        let mut alignment = Alignment::new(&first, &second);

        match alignment.calculate_score() {
            Ok(()) => (),
            Err(_) => panic!("Failed to calculate Score"),
        };

        let answer = alignment.align();

        assert_eq!(answer.0, "to_Align some s_t_uff".to_string());
        assert_eq!(answer.1, "to Align and_ aStguff".to_string());

        // println!("{:?}", alignment);
        // println!("{}", answer.0);
        // println!("{}", answer.1);
    }
}
