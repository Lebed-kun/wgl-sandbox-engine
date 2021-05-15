use wasm_bindgen::prelude::*;
use web_sys::console;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    };
}

macro_rules! try_unwrap {
    (@dev; $ex:expr, $error_msg:expr $(, $cond:expr)?) => {
        {
            let val = $ex;
            if val.is_none() {
                log!(
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
                    log!(
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