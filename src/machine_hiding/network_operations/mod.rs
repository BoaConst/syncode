/**
 * Module A.2 : Network Operations Module
 * Designer : Suumil 
 * Reviewer : Demin
 */
mod network_operations {
    #[derive(Debug, Clone)]
    pub enum RetryPolicy {
        None,
        ExponentialBackoff,
    }

    #[derive(Debug, Clone)]
    pub enum EncryptionStandard {
        None,
        AES,
        DES,
    }

    #[derive(Debug, PartialEq)]
    pub enum DataFlowError {
        UnreachableHost,
        NetworkIssue,
        DataCorruption,
        Timeout,
    }

    #[derive(Debug, PartialEq)]
    pub enum AuthenticationError {
        InvalidCredentials,
        ConnectionError,
    }

    #[derive(Debug, PartialEq)]
    pub enum EncryptionError {
        InvalidKey,
        EncryptionFailed,
    }

    pub struct NetworkOperationsModule {
        timeout: Option<u32>,
        retry_policy: Option<RetryPolicy>,
        encryption_standard: Option<EncryptionStandard>,
    }

    impl NetworkOperationsModule {
        fn new(timeout: Option<u32>, retry_policy: Option<RetryPolicy>, encryption_standard: Option<EncryptionStandard>) -> Self {
            NetworkOperationsModule {
                timeout,
                retry_policy,
                encryption_standard,
            }
        }

        // Transfers data to another machine.
        pub fn transfer_data(&self, data: &str, destination: &str) -> Result<(), DataFlowError> {
            // TODO: Implement data transfer logic
            Ok(())
        }

        // Authenticates the source machine.
        pub fn authenticate(&self, source: &str, auth_data: &str) -> Result<(), AuthenticationError> {
            // TODO: Implement authentication logic
            Ok(())
        }

        // Encrypts data for transmission.
        pub fn encrypt_data(&self, data: &str, encryption_key: &str) -> Result<String, EncryptionError> {
            // TODO: Implement encryption logic
            if encryption_key.is_empty() {
                return Err(EncryptionError::InvalidKey);
            }
            // Mock encryption, replace with real encryption logic
            let encrypted_data = format!("encrypted({})", data);
            Ok(encrypted_data)
        }
    }

    pub struct NetworkOperationsBuilder {
        timeout: Option<u32>,
        retry_policy: Option<RetryPolicy>,
        encryption_standard: Option<EncryptionStandard>,
    }

    impl NetworkOperationsBuilder {
        pub fn new() -> Self {
            NetworkOperationsBuilder {
                timeout: None,
                retry_policy: None,
                encryption_standard: None,
            }
        }

        pub fn timeout(mut self, timeout: u32) -> Self {
            self.timeout = Some(timeout);
            self
        }

        pub fn retry_policy(mut self, policy: RetryPolicy) -> Self {
            self.retry_policy = Some(policy);
            self
        }

        pub fn encryption_standard(mut self, standard: EncryptionStandard) -> Self {
            self.encryption_standard = Some(standard);
            self
        }

        pub fn build(self) -> NetworkOperationsModule {
            NetworkOperationsModule::new(self.timeout, self.retry_policy, self.encryption_standard)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::network_operations::*;
    use super::*;

    #[test]
    fn transfer_data_success() {
        let network_module = network_operations::NetworkOperationsBuilder::new()
                                .timeout(30)
                                .retry_policy(network_operations::RetryPolicy::ExponentialBackoff)
                                .encryption_standard(network_operations::EncryptionStandard::AES)
                                .build();
        let data = "example data";
        let destination = "10.101.202.21";
        let result = network_module.transfer_data(data, destination);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn transfer_data_failure() {
        let network_module = network_operations::NetworkOperationsBuilder::new()
                                .timeout(30)
                                .retry_policy(network_operations::RetryPolicy::ExponentialBackoff)
                                .encryption_standard(network_operations::EncryptionStandard::AES)
                                .build();
        let data = "example data";
        let unreachable_host = "10.12.12.12";
        let result = network_module.transfer_data(data, unreachable_host);
        assert_eq!(result, Err(DataFlowError::UnreachableHost));
    }

    #[test]
    fn authentication_success() {
        let network_module = network_operations::NetworkOperationsBuilder::new()
                                .timeout(30)
                                .retry_policy(network_operations::RetryPolicy::ExponentialBackoff)
                                .encryption_standard(network_operations::EncryptionStandard::AES)
                                .build();
        let source = "10.12.12.12";
        let auth_data = "valid_credentials";
        let result = network_module.authenticate(source, auth_data);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn authentication_failure() {
        let network_module = network_operations::NetworkOperationsBuilder::new()
                                .timeout(30)
                                .retry_policy(network_operations::RetryPolicy::ExponentialBackoff)
                                .encryption_standard(network_operations::EncryptionStandard::AES)
                                .build();
        let source = "10.12.12.12";
        let invalid_auth_data = "invalid_credentials";
        let result = network_module.authenticate(source, invalid_auth_data);
        assert_eq!(result, Err(AuthenticationError::InvalidCredentials));
    }

    #[test]
    fn data_encryption_success() {
        let network_module = network_operations::NetworkOperationsBuilder::new()
                                .timeout(30)
                                .retry_policy(network_operations::RetryPolicy::ExponentialBackoff)
                                .encryption_standard(network_operations::EncryptionStandard::AES)
                                .build();
        let data = "sensitive data";
        let encryption_key = "sha256";
        let result = network_module.encrypt_data(data, encryption_key);
        assert!(result.is_ok());
        assert_ne!(result.unwrap(), data);
    }

    #[test]
    fn data_encryption_failure() {
        let network_module = network_operations::NetworkOperationsBuilder::new()
                                .timeout(30)
                                .retry_policy(network_operations::RetryPolicy::ExponentialBackoff)
                                .encryption_standard(network_operations::EncryptionStandard::AES)
                                .build();
        let data = "sensitive data";
        let invalid_key = "";
        let result = network_module.encrypt_data(data, invalid_key);
        assert_eq!(result, Err(EncryptionError::InvalidKey));
    }
}
