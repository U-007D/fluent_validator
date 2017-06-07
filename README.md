Implement `Validator` to get a static (compile-time-optimized) validator usable both with method-syntax and function-syntax styles.  Dynamic validator to be implemented as (if) need arises.

*** Sample 3-stage validator ***

Mutating validators require owning datatypes (eg. String):
```rust
impl Validator<String> for Base64ByteString {
    fn validate(value: String) -> Result<String> {
        Ok(value)

            //value contains data?
            .and_then(|v| match !v.is_empty() {
                true => Ok(v),
                false => Err(Error::EmptyValue(VAL_ERR_EMPTY_VALUE.to_string())),
            })

            //ensure value defines whole bytes (contains a multiple-of-three number of base-64 digits)
            .and_then(|v| v += "=="[..v.len() % 3])

            //value contains only valid base-64 characters?
            .and_then(|v| match v.chars()
                                 .all(|c| c.is_hex_char()) {
                true => Ok(v),
                false => Err(Error::IllegalValue((VAL_ERR_ILLEGAL_BASE_64_DIGIT.to_string()))),
            })

            //value passes validation
            .and_then(|v| Ok(v))
    }
}
```

Non-mutating validators can use any datatypes:
```rust
impl<T: AsRef<str>> Validator<T> for HexByteString {
    fn validate(value: T) -> Result<T> {
        Ok(value.as_ref())

            //value contains data?
            .and_then(|v| match !v.is_empty() {
                true => Ok(v),
                false => Err(Error::EmptyValue(VAL_ERR_EMPTY_VALUE.to_string())),
            })

            //value defines whole bytes (contains an even number of hex digits)?
            .and_then(|v| match v.len() % 2 == 0 {
                true => Ok(v),
                false => Err(Error::InvalidSize(VAL_ERR_INVALID_SIZE.to_string())),
            })

            //value contains only valid hexadecimal characters?
            .and_then(|v| match v.chars()
                                 .all(|c| c.is_hex_char()) {
                true => Ok(v),
                false => Err(Error::IllegalValue((VAL_ERR_ILLEGAL_HEX_DIGIT.to_string()))),
            })?;
        Ok(value)
    }
}
```

Invoke validators as follows:
```rust
let hex_value = "123456";
let base_64_value = "123456";

// Method syntax
let hex_result1 = hex_value.validate::<HexByteString>();
let base_64_result1 = base_64_value.validate::<Base64ByteString>();

// Function syntax
let hex_result2 = Validate::<HexByteString>(hex_value);
let base_64_result2 = Validate::<Base64ByteString>(base_64_value);
```
