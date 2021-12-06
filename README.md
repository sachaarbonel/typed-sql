# typed-sql

This crate provides the sql! macro for building fully type checked SQL statements inside your Rust code using roughly SQL compatible syntax. Fails at compile time if the query 

## Quick Preview

```rust
 let value = sql!(CREATE TABLE Persons (
        PersonID int,
        LastName varchar(255),
        FirstName varchar(255),
        Address varchar(255),
        City varchar(255)
    ););
    assert_eq!(value, "CREATE TABLE Persons (PersonID INT(32), LastName VARCHAR(255), FirstName VARCHAR(255), Address VARCHAR(255), City VARCHAR(255))");
```
