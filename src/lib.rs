#[macro_use]
extern crate napi_derive;
use napi::{CallContext, JsObject, JsNumber, Result};

#[inline(always)]
fn fibonacci_native(n: i64) -> i64 {
    let mut x : i64 = 1;
    let mut y : i64 = 1;
    let mut temp : i64;

    for _ in 1..n {
        temp = x;
        x = x+y;
        y = temp
    }

    y
}

#[js_function(1)] // ------> arguments length, omit for zero
fn fibonacci(ctx: CallContext) -> Result<JsNumber> {
    let n = ctx.get::<JsNumber>(0)?.get_int64()?;
    ctx.env.create_int64(fibonacci_native(n))
}

/// `exports` is `module.exports` object in NodeJS
#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
    exports.create_named_method("fibonacci", fibonacci)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_test() {
        let result = fibonacci_native(50);
        assert_eq!(result, 12586269025);
    }
}
