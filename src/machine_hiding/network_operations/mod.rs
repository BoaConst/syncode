use std::ptr::write;

fn transfer_data(data: &str, destination: &str) -> Result<(), DataFlowError> {
    todo!()
    // network data flow transfer invoked to transfer data to
    // another remote and get data ready for remote repo operation
    // 1. match authenticate for permission, AuthenticationError if failed
    // example: let auth = authenticate(destination, data).expect("authentifacation failure")
    // 2. match encrypt_data for data pack up, EncryptionError if failed
    // Or, we can write the procedure separately instead embeded in transfer_data
    // low priority for now
}

fn authenticate(source: &str, auth_data: &str) -> Result<(), AuthenticationError>{
    todo!()
    // data access authentication list update, AuthenticationError if failed
}

fn encrypt_data(data: &str, encryption_key: &str) -> Result<String, EncryptionError> {
    todo!()
    // data encrypt_data packed up, EncryptionError if failed
}
