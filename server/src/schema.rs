table! {
    users (username) {
        username -> Varchar,
        double_hashed_passwd -> Varchar,
        hash_salt -> Varchar,
        data -> Nullable<Varchar>,
        vault_salt -> Varchar,
    }
}
