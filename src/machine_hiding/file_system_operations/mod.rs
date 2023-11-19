
fn identify_conflict(file_path: &str) -> Result<FileSystemConflict, IoError> {
    todo!()
    // suppose the target local file is incompatible with remote repo, then IoError
    // too abstract and least pritority for now
}

fn adapt_io_operations(file_path: &str, conflict: FileSystemConflict) -> Result<(), IoError>{
    todo!()
    // given incompatible file in local system, adaptor involved to handle conflict with remote
    // too abstract and least pritority for now
}