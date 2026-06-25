#[cfg(test)]
mod tests {
    use gupt_common::types::UserId;
    use gupt_identity::manager::IdentityManager;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_full_identity_creation_flow() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("gupt.db");

        // 1. Create a database
        let db = gupt_storage::Database::open(db_path.to_str().unwrap(), "my_super_secret_pin").unwrap();
        db.run_migrations().unwrap();

        // 2. Create an identity
        let mut identity_manager = IdentityManager::default();
        let identity = identity_manager.create_identity("alice", "123456").unwrap();

        assert_eq!(identity.user_id.as_ref().starts_with("user_"), true);
        assert_eq!(identity.signing_public_key.len(), 32);
        
        // Ensure identity is unlocked
        assert!(identity_manager.is_unlocked());

        // Lock it
        identity_manager.lock();
        assert!(!identity_manager.is_unlocked());
    }
}
