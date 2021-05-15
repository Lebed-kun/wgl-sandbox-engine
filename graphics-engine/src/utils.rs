macro_rules! try_unwrap {
    (@dev; $ex:expr, $error_msg:expr $(, $cond:expr)?) => {
        {
            let val = $ex;
            if val.is_none() {
                println!(
                    "Error in module \"{}\" in line {}: \n\"{}\"",
                    file!(),
                    line!(),
                    $error_msg
                );
                return None;
            }

            $(
                let val = val.filter($cond);
                if val.is_none() {
                    println!(
                        "Error in module \"{}\" in line {}: \n\"{}\"",
                        file!(),
                        line!(),
                        $error_msg
                    );
                    return None;
                }
            )?

            val.unwrap()
        }
    };

    ($ex:expr $(, $cond:expr)?) => {
        {
            let val = $ex;
            if val.is_none() {
                return None;
            }

            $(
                let val = val.filter($cond);
                if val.is_none() {
                    return None;
                }
            )?

            val.unwrap()
        }
    };
}