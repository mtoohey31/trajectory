table! {
    users (username) {
        username -> Varchar,
        double_hashed_passwd -> Varchar,
        hash_salt -> Binary,
        data -> Nullable<Binary>,
        vault_salt -> Binary,
    }
}
