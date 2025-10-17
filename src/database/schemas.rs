diesel::table! {
    uploads (uuid) {
        uuid -> Uuid,
        expiration -> BigInt,
        getted -> Bool,
    }
}
