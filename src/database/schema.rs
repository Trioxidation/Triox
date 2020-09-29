table! {
    users (ID) {
        ID -> Unsigned<Integer>,
        NAME -> Varchar,
        EMAIL -> Varchar,
        PASSWORD_HASH -> Char,
        LANGUAGE_CODE -> Nullable<Char>,
        ROLE -> Unsigned<Tinyint>,
        STATUS -> Nullable<Varchar>,
    }
}
