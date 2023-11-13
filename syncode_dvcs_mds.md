**Group 2 - SynCode:** Demin, Suumil, Helia, Pranay, Shunzhi

# <p align="center">DVCS Module Specifications</p>
# <p align="center">Part A: DVCS Acceptance Tests</p>

## Acceptance Test ID 1: File System Compatibility

**Description:** Test the DVCS system's ability to handle file system compatibility.
- Checks if the system can recognize and handle line endings, file system case sensitivity, path length limitations, and path separators on different operating systems.
- Verifies that the Machine Hiding module appropriately abstracts file system operations.
- Ensures cross-platform data consistency and compatibility.

**Steps:**
1. Initialize the DVCS system and create a new repository.
2. Perform file system operations on different operating systems.
3. Commit changes and validate cross-platform data consistency.
4. Inspect repository to ensure compatibility handling.
5. Verify Machine Hiding abstraction for file system operations.

## Acceptance Test ID 2: Repository Inspection

**Description:** Test the DVCS system's repository inspection capabilities.
- Checks if the Inspection Module provides accurate information about repository status.
- Validates the ability to view heads, differences between revisions (diff), file contents (cat), and checkout historical states.
- Ensures that users can effectively inspect and navigate the repository's revision history.

**Steps:**
1. Initialize the DVCS system and create a new repository.
2. Perform changes, commits, and revisions.
3. Inspect repository heads, differences between revisions, and file contents.
4. Check out historical states and validate inspection capabilities.

## Acceptance Test ID 3: User Authentication

**Description:** Test the DVCS system's user authentication mechanisms.
- Validates the Authentication Manager Module's ability to check user authorization.
- Ensures proper reporting to users who are not authorized to access specific repositories.
- Verifies the correct management of user permissions within the DVCS system.

**Steps:**
1. Initialize the DVCS system and create repositories with different access levels.
2. Attempt DVCS commands with authorized and unauthorized users.
3. Validate correct reporting for unauthorized users.
4. Check user permissions and ensure proper management.

## Acceptance Test ID 4: Commit Changes

**Description:** Test the DVCS system's ability to commit changes.
- Validates the Staging Module's ability to stage changes for commit.
- Ensures proper recording of changes, creating updated revisions in the repository.
- Verifies that the Commit facility functions as expected.

**Steps:**
1. Initialize the DVCS system and create a new repository.
2. Make changes to tracked and untracked files.
3. Stage changes for commit and verify staging status.
4. Perform a commit and ensure proper recording of changes.

## Acceptance Test ID 5: Pull Changes

**Description:** Test the DVCS system's ability to pull changes from remote repositories.
- Validates the Synchronization Module's ability to retrieve and integrate updates.
- Ensures that local repositories are updated with the latest revisions from remote repositories.

**Steps:**
1. Initialize the DVCS system and create a local repository.
2. Set up a remote repository and push changes to it.
3. Create a new local repository and pull changes from the remote.
4. Validate integration of updates in the local repository.

## Acceptance Test ID 6: Push Changes

**Description:** Test the DVCS system's ability to push changes to remote repositories.
- Validates the Synchronization Module's ability to upload local repository changes.
- Ensures that remote repositories are updated with the latest revisions from local repositories.

**Steps:**
1. Initialize the DVCS system and create a local repository.
2. Make changes and commit them in the local repository.
3. Set up a remote repository and push changes to it.
4. Validate that the remote repository is updated with the latest revisions.


# <p align="center">Part B: Module Guide</p>

# Module A: Machine Hiding
**Designers:** Demin, Suumil | **Reviewers:** Suumil, Demin

## Role
Machine hiding is categorized to handle the necessary changes in commands used by the other two modules for different machine operating systems (more specifically file systems). Provide a consistent interface for interacting with the underlying operating system and hardware. Abstract file system operations, network communication, environment variable management.

## Secret
- *Primary:* This module will cover the difference of file systems in different machines. The primary secret will involve methods that recognize file system differences and keep cross-platform data consistency.
- *Secondary:* Will be listing potential file system conflict and implementation to work around potential conflicts while keeping repository data consistency between targeted systems.

Some examples are listed as follows:
- *Line Endings:* The handling of line endings can vary between operating systems. Unix-based systems (Linux, macOS) use LF (Line Feed) characters, while Windows uses CRLF (Carriage Return and Line Feed) characters.
- *Filesystem Case Sensitivity:* Some operating systems, like Linux and macOS, have case-sensitive file systems, while Windows has a case-insensitive file system.
- *File Path Length:* Windows has a maximum path length limitation (260 characters) that may lead to issues with long file paths.
- *Path Separators:* The character used to separate directory paths in file paths can vary between operating systems. On Unix-based systems, it's "/", while on Windows, it's "".

## Notes on Decomposition
**Reason(s)**
The Machine Hiding module is designed to collect and handle cross-platform conflicts in different file system I/O settings. Most common differences are handled by Rust inner design.

If time permits, we are interested in distributed features above basic multi-repository single machine systems.

**Rule(s)**
- Unexpected file system I/O conflict collecting module will collect all special conflicts encountered in the other two modules that are not solved by Rust.
- File system specific I/O adaptor module will solve the conflicts collected.
- Cross-machine communication authenticator module will manage and verify remote repository communication request.
- Data encryption module will handle data encryption and key to be transferred once the communication is authenticated.
- Cross-machine data flow manager module will handle encrypted data upload and download.

**Fuzzy Area(s)**
The Machine Hiding Module is expected to interact with both Behaviour Hiding and Repository Hiding modules. Changes in machines may make a facility design decision less appropriate or deviate from expectation. This module will evolve with unexpected demands from the other two modules.

# Module A.1: File System Operations Module
**Designer:** Demin | **Reviewer:** Suumil

**Role:**
The file system module will list and handle potential conflicts that do not have a genuine solution in Rust, for different file systems of operating systems.

**Secret:**
- *Primary:* Unexpected conflicts in different file systems of operating systems. Especially the conflicts that cannot be handled by Rust in the implementation, though Rust I/O module itself is a big module hiding that helps most of the cross-platform issues.
- *Secondary:* The implementation of necessary system-specific I/O commands.

**Facilities:**
1. **Cross-machine filesystem I/O conflicts collecting module**
    - *Description:* Collects and identifies unique filesystem conflicts that arise due to different operating systems, which are not inherently resolved by Rust.
    - *Method:*
     ```rust
     fn identify_conflict(file_path: &str) -> Result<FileSystemConflict, IoError>;
     ```
    - *Input:*
        - `file_path: &str`: The path of the file where a potential conflict might occur.
    - *Output:*
        - `Result<FileSystemConflict, IoError>`: The type of conflict identified or an error if the process fails.

2. **File system-specific I/O adaptor module**
    - *Description:* Adapts file system operations to handle identified conflicts in a way that ensures consistency across different operating systems.
    - *Method:*
     ```rust
     fn adapt_io_operations(file_path: &str, conflict: FileSystemConflict) -> Result<(), IoError>;
     ```
    - *Input:*
        - `file_path: &str`: The path of the file to be adapted.
        - `conflict: FileSystemConflict`: The specific conflict identified that needs resolution.
    - *Output:*
        - `Result<(), IoError>`: Success if adaptation is successful, or an error if it fails.

**Tests:**

1. **Conflict Identification Success:**
    ```rust
    // Test ID: 1
    let file_system_module = FileSystemOperationsModule::new();
    let file_path = "/path/to/file";
    let result = file_system_module.identify_conflict(file_path);
    assert_eq!(result.is_ok(), true);
    ```
2. **Conflict Identification Failure:**
    ```rust
    // Test ID: 2
    let file_system_module = FileSystemOperationsModule::new();
    let invalid_file_path = "/invalid/path";
    let result = file_system_module.identify_conflict(invalid_file_path);
    assert_eq!(result.is_err(), true);
    ```

3. **I/O Operation Adaptation Success:**
    ```rust
    // Test ID: 3
    let file_system_module = FileSystemOperationsModule::new();
    let file_path = "/path/to/file";
    let conflict = FileSystemConflict::LineEndings;
    let result = file_system_module.adapt_io_operations(file_path, conflict);
    assert_eq!(result.is_ok(), true);
    ```

4. **I/O Operation Adaptation Failure:**
    ```rust
    // Test ID: 4
    let file_system_module = FileSystemOperationsModule::new();
    let file_path = "/path/to/file";
    let invalid_conflict = FileSystemConflict::Unknown;
    let result = file_system_module.adapt_io_operations(file_path, invalid_conflict);
    assert_eq!(result.is_err(), true);
    ```

# Module A.2: Network Operations Module
**Designer:** Suumil | **Reviewer:** Demin

**Role:**
Network operations module is a potential bonus feature in our project. It is responsible for cross-machine communication functioning, safety, and authentication.

**Secret:**
- *Primary:* Cross-machine repository data exchange and the authentication and safety features necessary for data integrity and safety.
- *Secondary:* The function modules that control the data flow among machines and the algorithms that manage authentications, data encryption, and key management.

**Facilities:**
1. **Cross-machine data flow manager module**
    - *Description:* This functionality manages and controls the flow of data between different machines, ensuring eﬀicient and correct data transfer.
    - *Method:*
     ```rust
     fn transfer_data(data: &str, destination: &str) -> Result<(), DataFlowError>;
     ```
    - *Input:*
        - `data: &str`: The data to be sent to another machine. 
        - `destination: &str`: The identifier or address of the destination machine.
    - *Output:*
        - `Result<(), DataFlowError>`: Success or an error if the data flow process encounters issues.
2. **Cross-machine communication authenticator module**
    - *Description:* Handles the authentication of machines involved in the commu- nication, ensuring that the data is exchanged between authenticated sources only.
    - *Method:*
     ```rust
     fn authenticate(source: &str, auth_data: &str) -> Result<(), AuthenticationError>;
     ```
    - *Input:*
        - `source`: The identifier of the source machine. 
        - `auth_data`: The authentication data or credentials.
    - *Output:*
        - `Result<(), AuthenticationError>`: Success or an error related to authentication.

3. **Data encryption module for permitted data exchange**
    - *Description:* Encrypts data using a specified key before it is sent over the network, ensuring data confidentiality and integrity.
    - *Method:*
     ```rust
     fn encrypt_data(data: &str, encryption_key: &str) -> Result<String, EncryptionError>;
     ```
    - *Input:*
        - `data`: The unencrypted data that needs to be encrypted. 
        - `encryption_key`: The key used for encryption.
    - *Output:*
        - `Result<String, EncryptionError>`: The encrypted data or an error if the encryption process fails.

**Tests:**

1. **Transfer Data Success:**
    ```rust
    // Test ID: 1
    let network_module = NetworkOperationsModule::new();
    let data = "example data";
    let destination = "10.101.202.21";
    let result = network_module.transfer_data(data, destination); assert_eq!(result.is_ok(), true);
    ```

2. **Transfer Data Failure:**
    ```rust
    // Test ID: 2
    let network_module = NetworkOperationsModule::new();
    let data = "example data";
    let unreachable_host = "10.12.12.12";
    let result = network_module.transfer_data(data, unreachable_host); assert_eq!(result.is_err(), true);
    ```

3. **Authentication Success:**
    ```rust
    // Test ID: 3
    let network_module = NetworkOperationsModule::new();
    let source = "10.12.12.12";
    let auth_data = "valid_credentials";
    let result = network_module.authenticate(source, auth_data); assert_eq!(result.is_ok(), true);
    ```

4. **Authentication Failure:**
    ```rust
    // Test ID: 4
    let network_module = NetworkOperationsModule::new();
    let source = "10.12.12.12";
    let invalid_auth_data = "invalid_credentials";
    let result = network_module.authenticate(source, invalid_auth_data); assert_eq!(result.is_err(), true);
    ```

5. **Data Encryption Success:**
    ```rust
    let network_module = NetworkOperationsModule::new();
    let data = "sensitive data";
    let encryption_key = "sha256";
    let result = network_module.encrypt_data(data, encryption_key); assert!(result.is_ok());
    assert_ne!(result.unwrap(), data);
    ```

6. **Data Encryption Failure:**
    ```rust
    let network_module = NetworkOperationsModule::new();
    let data = "sensitive data";
    let invalid_key = "";
    let result = network_module.encrypt_data(data, invalid_key); assert_eq!(result.is_err(), true);
    ```

# Module B: User Hiding
**Designers:** Shunzhi, Helia | **Reviewers:** Demin, Pranay, Suumil

## Role
This module is responsible for hiding the DVCS behaviors from users, including error handling, input/output, and user interface management.

Within the architecture of User Hiding, we have decomposed it into four leaf-modules, namely the Command Parser, Syntax Checker & Error Handling, User Interface, and Authentication Module. Each of these sub-modules specializes in distinct modes of user interactions with the DVCS system.

## Notes on Decomposition
**Reason(s)**
The User Hiding Module is decomposed into four specialized sub-modules to ensure that it effectively hides DVCS behaviors from users, providing a clean and modular interface. This decomposition allows for a clear focus on different aspects of user interaction, including parsing user commands, handling errors, managing the user interface, and implementing authentication. Specialization in each of these areas helps maintain a separation of concerns, making it easier to manage and extend the user interaction components while ensuring that the DVCS remains user-friendly and secure.


**Rule(s)**
- The Command Parser Module specializes in parsing user commands and validating command correctness. It is responsible for interpreting user input and translating it into actionable commands for the DVCS.
- The Syntax Checker & Error Handling Module is dedicated to handling errors, including syntax errors and semantic errors, as well as managing exceptions. This module ensures that users receive meaningful feedback when they make mistakes and that errors are properly logged and reported.
- The User Interface Module focuses on managing the interaction between users and the DVCS through a user-friendly interface. It handles user prompts, provides help messages, and ensures a smooth user experience.
- The Authentication Module is responsible for managing user authentication and access control. It ensures that only authorized users can perform certain DVCS operations and provides a secure access mechanism.

**Fuzzy Area(s)**
The Behavior Hiding Module is expected to interact with both Machine Hiding and Repository Hiding Modules in order to handle machine-specific and DVCS-specific operations, which can lead to various fuzzy areas.
- *Machine Hiding:* For example, the Command Parser Module and the Syntax Checking & Error Handling Module might have to work closely with Machine Hiding in order to make sure differences related to different OS or devices get processed correctly. 
- *Repository Hiding:* For instance, the User Interface Module will have to interact with the Repository Hiding module constantly in order to provide data/feedback/status back and forth between the user and the DVCS system, as well as between different modules in the DVCS system.

# Module B.1: Command Parser Module
**Designer:** Pranay | **Reviewer:** Demin

**Role:**
The Command Parser Module specializes in parsing user commands and validating command correctness. It is responsible for interpreting user input and translating it into actionable commands for the DVCS.

**Secrets:**

- *Primary:* The module primarily hides the internal data structures for the input command, command processing mechanism, and logic behind checking command’s correctness.

- *Secondary:* The module also hides the implementation details and low-level algorithms regarding command parsing and correctness checking.

**Facilities:**

1. **Parsing**
   - *Description:* This functionality reads the user input commands.
   - *Method:*
     ```rust
     fn parse_command(user_input: &str) -> Result<Command, ParseError>;
     ```
   - *Input:*
     - `user_input`: User-provided command as a string.
   - *Output:*
     - `Result<Command, ParseError>`: Parsed command or an error if the parsing process encounters issues.

2. **Validating the Command’s Correctness**
   - *Description:* This functionality validates the user command's correctness.
   - *Method:*
     ```rust
     fn validate_command(command: &Command) -> Result<(), ValidationError>;
     ```
   - *Input:*
     - `command`: Parsed command to be validated.
   - *Output:*
     - `Result<(), ValidationError>`: Success or an error if the command validation process encounters issues.

**Tests:**

1. **Command Parsing Test - Valid Command**
   ```rust
   // Test ID: 1
   let user_input = "git commit -m 'Fix issue #123'";
   let result = command_parser_module.parse_command(user_input);
   assert_eq!(result.is_ok(), true);
   ```

2. **Command Parsing Test - Invalid Command**
   ```rust
   // Test ID: 2
   let user_input = "git invalid-command";
   let result = command_parser_module.parse_command(user_input);
   assert_eq!(result.is_err(), true);
   ```

3. **Command Validation Test - Valid Command**
   ```rust
   // Test ID: 3
   let valid_command = create_valid_command();
   let result = command_parser_module.validate_command(&valid_command);
   assert_eq!(result.is_ok(), true);
   ```

4. **Command Validation Test - Invalid Command**
   ```rust
   // Test ID: 4
   let invalid_command = create_invalid_command();
   let result = command_parser_module.validate_command(&invalid_command);
   assert_eq!(result.is_err(), true);
   ```

5. **Command Parsing and Validation Test - Complex Command**
   ```rust
   // Test ID: 5
   let user_input = "git merge -b feature-branch";
   let parsed_command = command_parser_module.parse_command(user_input).unwrap();
   let result = command_parser_module.validate_command(&parsed_command);
   assert_eq!(result.is_ok(), true);
   ```

6. **Command Parsing and Validation Test - Empty Command**
   ```rust
   // Test ID: 6
   let user_input = "";
   let result = command_parser_module.parse_command(user_input);
   assert_eq!(result.is_err(), true);
   ```

These tests cover various scenarios, including parsing valid and invalid commands, validating commands, and handling complex or empty commands. The reviewer, Demin, should focus on ensuring that the parsing and validation methods behave correctly and handle different input scenarios appropriately.

# Module B.2: Syntax Checker & Error Handling Module

**Designer:** Helia | **Reviewer:** Pranay

**Role:**
This module is responsible for handling various types of errors that may occur in user input (besides command, which has been cleared by Module 2.1). These errors could include syntax errors, semantic errors, and exceptions in the file name(s), directory path(s), etc., that the user types next to their command.

**Secret:**
- *Primary:* The module primarily hides the internal error-checking and handling mechanisms for user input.
- *Secondary:* The module also hides the implementation details and low-level algorithms regarding error checking and handling of user input.

**Facilities:**

1. **Error Detection:**
   - *Description:* This functionality detects all types of errors possibly in the user’s input.
   - *Method:* 
     ```rust
     fn detect_errors(user_input: &str) -> Result<Option<UserInputError>, ErrorDetectionError>;
     ```

   - *Input:* `'user_input: &str'` - the user’s input to be checked for errors.
   - *Output:*
     - `Ok(None)`: No errors detected.
     - `Ok(Some(UserInputError)`: Errors detected with details in the `UserInputError` type.
     - `Err(ErrorDetectionError)`: An error if the error detection process encounters issues.

2. **Error Message:**
   - *Description:* This functionality informs the user of potential errors detected from their input.
   - *Method:*
     ```rust
     fn show_error_message(error: &UserInputError) -> String;
     ```
   - *Input:* `'Error: &UserInputError'` - the detected user input error.
   - *Output:* `'String'` - A formatted error message to be displayed to the user.

**Tests:**

1. **Error Detection Test - No error**
    ```rust
    // Test ID: 1
    let user_input = "some user command";
    let result = error_handling_module::detect_errors(user_input);
    assert_eq!(result, Ok(None));
    ```
2. **Error Detection Test - Syntax error**
    ```rust
    // Test ID: 2
    let user_input = "invalid syntax";
    let result = error_handling_module::detect_errors(user_input);
    assert_eq!(result, Ok(Some(UserInputError::SyntaxError)));
    ```
3. **Error Detection Test - Semantic error**
    ```rust
    // Test ID: 3
    let user_input = "semantic error";
    let result = error_handling_module::detect_errors(user_input);
    assert_eq!(result, Ok(Some(UserInputError::SemanticError)));
    ```
4. **Error Detection Test - Unexpected error**
    ```rust
    // Test ID: 4
    let user_input = "unexpected/unknown error";
    let result = error_handling_module::detect_errors(user_input);
    assert_eq!(result, Err(ErrorDetectionError));
    ```
5. **`SyntaxError` Message Test**
    ```rust
    // Test ID: 5
    let error = UserInputError::SyntaxError;
    let result = error_handling_module::show_error_message(&error);
    assert_eq!(result, "Syntax Error ABC found! Suggestion: XYZ.");
    ```
6. **`SemanticError` Message Test**
    ```rust
    // Test ID: 6
    let error = UserInputError::SemanticError;
    let result = error_handling_module::show_error_message(&error);
    assert_eq!(result, "Semantic Error ABC found! Suggestion: XYZ.");
    ```

# Module B.3: User Interface Module
**Designer:** Pranay | **Reviewer:** Suumil

**Role:**
The User Interface Module focuses on managing interactions between users and the DVCS through a user-friendly interface. It handles user prompts, provides help messages, and ensures a smooth user experience.

**Secrets:**

- *Primary:* This module primarily hides the internal data/info and task exchanging mechanism between the DVCS system and the user, as well as between different modules inside the DVCS.

- *Secondary:* This module also hides the implementation details and low-level algorithms regarding intermodule communication and user-DVCS communication.

**Facilities:**

1. **User Input**
    - *Description:* This module provides a simple interface for the user to input their command and necessary info.
    - *Method:* 
     ```rust
     fn get_user_input() -> String;
     ```
    - *Output:*
        - `String`: User-provided input.

2. **Helping Message**
    - *Description:* This functionality allows the user to learn the necessary syntax or actions they need to provide/do for the DVCS system to work.
    - *Method:* 
     ```rust
     fn display_help_message() -> String;
     ```
    - *Output:*
        - `String`: Help message providing necessary syntax or actions for the DVCS system.

3. **User Prompt**
    - *Description:* This functionality prompts the user to the necessary steps through the DVCS functionalities.
    - *Method:*
     ```rust
     fn prompt_user(message: &str);
     ```
    - *Input:*
        - `message`: Message to prompt the user.

4. **Staging Change**
    - *Description:* This module also informs users of the current status of their command request if applicable.
    - *Method:*
     ```rust
     fn inform_staging_change(status: &str);
     ```
    - *Input:*
        - `status`: Current status of the command request.

**Tests:**

1. **User Input Test - Valid Input**
   ```rust
   // Test ID: 1
   let result = user_interface_module.get_user_input();
   assert_eq!(result.is_empty(), false);
   ```

2. **User Input Test - Empty Input**
   ```rust
   // Test ID: 2
   let result = user_interface_module.get_user_input();
   assert_eq!(result.is_empty(), true);
   ```

3. **Helping Message Test**
   ```rust
   // Test ID: 3
   let help_message = user_interface_module.display_help_message();
   assert_eq!(help_message.is_empty(), false);
   ```

4. **User Prompt Test**
   ```rust
   // Test ID: 4
   user_interface_module.prompt_user("Please enter your command:");
   // Manual check for user prompt in the console.
   ```

5. **Staging Change Test - Valid Status**
   ```rust
   // Test ID: 5
   user_interface_module.inform_staging_change("Command in progress");
   // Manual check for staging change information in the console.
   ```

6. **Staging Change Test - Empty Status**
   ```rust
   // Test ID: 6
   user_interface_module.inform_staging_change("");
   // Manual check for no staging change information in the console.
   ```

# Module B.4: Authentication Manager Module
**Designer:** Pranay | **Reviewer:** Helia

**Role:**
The Authentication Module is responsible for checking whether the user requesting DVCS command is a valid user. It ensures that only authorized users can perform certain DVCS operations and provides a secure access mechanism.

**Secrets:**
- *Primary:* This module primarily hides the internal user authentication mechanisms for the DVCS system.
- *Secondary:* This module also hides the implementation details and low-level algorithms regarding user authentication for the DVCS system.

**Facilities:**

1. **Check**
    - *Description:* This functionality will check the users if they are authorized to access or commit the certain repository.
    - *Method:*
     ```rust
     fn check_user_authorization(username: &str, repository: &str, action: &str) -> Result<(), AuthorizationError>;
     ```
    - *Input:*
        - `username`: User's username.
        - `repository`: Repository for which authorization is checked.
        - `action`: Specific action for which authorization is checked.
    - *Output:*
        - `Result<(), AuthorizationError>`: Success or an error if the user is not authorized.

2. **Report**
    - *Description:* This functionality is major for reporting to the users who are not authorized to the certain repository.
    - *Method:* 
     ```rust
     fn report_unauthorized_access(username: &str, repository: &str);
     ```
    - *Input:*
        - `username`: User's username.
        - `repository`: Repository for which unauthorized access is reported.

3. **Permissions**
    - *Description:* This functionality will mark users’ current permissions to a certain repository.
    - *Method:*
     ```rust
     fn get_user_permissions(username: &str, repository: &str) -> Vec<String>;
     ```
    - *Input:*
        - `username`: User's username.
        - `repository`: Repository for which permissions are queried.
    - *Output:*
        - `Vec<String>`: List of user's permissions for the specified repository.

**Tests:**

1. **Check Authorization Test - Authorized User**
   ```rust
   // Test ID: 1
   let result = auth_manager_module.check_user_authorization("user1", "repo1", "commit");
   assert_eq!(result.is_ok(), true);
   ```

2. **Check Authorization Test - Unauthorized User**
   ```rust
   // Test ID: 2
   let result = auth_manager_module.check_user_authorization("user2", "repo1", "commit");
   assert_eq

3. **Check Authorization Test - Invalid Repository**
   ```rust
   // Test ID: 3
   let result = auth_manager_module.check_user_authorization("user1", "invalid_repo", "commit");
   assert_eq!(result.is_err(), true);
   ```

4. **Check Authorization Test - Invalid Action**
   ```rust
   // Test ID: 4
   let result = auth_manager_module.check_user_authorization("user1", "repo1", "invalid_action");
   assert_eq!(result.is_err(), true);
   ```

5. **Report Unauthorized Access Test**
   ```rust
   // Test ID: 5
   auth_manager_module.report_unauthorized_access("unauthorized_user", "repo1");
   // Manual check for unauthorized access report in the console.
   ```

6. **Get User Permissions Test - Valid Request**
   ```rust
   // Test ID: 6
   let permissions = auth_manager_module.get_user_permissions("user1", "repo1");
   assert_eq!(permissions.is_empty(), false);
   ```

7. **Get User Permissions Test - Invalid User**
   ```rust
   // Test ID: 7
   let permissions = auth_manager_module.get_user_permissions("invalid_user", "repo1");
   assert_eq!(permissions.is_empty(), true);
   ```

8. **Get User Permissions Test - Invalid Repository**
   ```rust
   // Test ID: 8
   let permissions = auth_manager_module.get_user_permissions("user1", "invalid_repo");
   assert_eq!(permissions.is_empty(), true);
   ```

# Module C: Repository Hiding
**Designers:** Pranay, Denim, Suumil | **Reviewers:** Helia, Shunzhi

## Role
The Repository Hiding module is responsible for hiding the intricacies involved in the processes of managing, accessing, and manipulating repositories in the DVCS. This module ensures that the users and other modules interact with the repositories through well-defined interfaces, abstracting the underlying complexities.

Within the architecture of Repository Hiding, we have decomposed it into four leaf modules, namely the Initialization, Staging, Inspection Module and the Synchronization Module. Each of these leaf modules specializes in distinct modes of repository interaction.

## Notes on Decomposition
**Reason(s)**
The Repository module is decomposed into specialized modules to ensure a clear focus on different aspects of repository management, aligning with the responsibility hierarchy.
This decomposition allows for specialization in specific functionalities, making it easier to manage and extend the system while keeping a clean separation of concerns.
It promotes modularity and code reusability by organizing related operations into distinct modules.

**Rule(s)**
- The Initialization Module is designed to handle repository creation (init) and repository cloning (clone), which are fundamental steps in the DVCS setup process.
- The Inspection Module specializes in providing tools for revision inspection, such as listing heads, comparing revisions (diff), displaying file contents at specific points (cat), and checking out historical states.
- The Synchronization Module focuses on managing synchronization operations between repositories, including merging changes, pulling updates, and pushing commits to remote repositories.
- The Staging Module is dedicated to staging and managing changes, preparing them for commits, including facilities for committing, adding changes, removing staged files, and checking the status of changes in the working directory.

**Fuzzy Area(s)**
It is expected to interact with both Machine Hiding and Behaviour hiding.
- *Machine Hiding:*
The module might interact with "Machine Hiding" to ensure that the necessary environmental configurations and resources are available for the inspection facilities to function effectively.
- *User Hiding:*
This module might interact with “User Hiding” to get the current status of the repository to help with the heads, and the diff. Complicated workflows like conflict resolution require interfaces and requirements for the same might change to suit user requirements. Further it will also require us to ensure that we have the correct authentication rights and the files and directories for the repository. Thus there would be an intersection with Behavior Hiding. 

# Module C.1 : Initialization Module

**Designer:** Helia | **Reviewer:** Shunzhi

**Role:**
The Initialization Module is responsible for initializing new repositories and cloning existing ones within the DVCS system. Its primary role is to provide users with essential tools for setting up and configuring repositories effectively.

**Secret:**
- *Primary:* The core algorithms and implementation details for repository initialization (init) and cloning (clone). This involves the low-level processes required to create and configure a new repository or clone an existing one, including handling the creation of repository data structures, managing user authentication, and access control.
- *Secondary:* Optimization techniques to enhance the performance of initialization and cloning processes, including strategies for efficient data caching and handling edge cases.

**Facilities:**

1. **Init:**
   - *Description:* This functionality creates a new repository with capabilities to handle cross-file-system differences. It can be used to convert an existing, unversioned project to a repository or initialize a new, empty repository.
   - *Method:*
     ```rust
     fn init(repository_name: &str) -> Result<(), InitError>;
     ```

   - *Input:* `'repository_name: &str'` - The user-defined name of the new repository
   - *Output:*
     - `Ok()`: The new repository is created and nothing has to be returned.
     - `Err(InitError)`: Initialization failed.

2. **Clone:**
   - *Description:* This functionality obtains a copy of a repository, covering cross-platform conflict handling. Internally, Clone would call Init to first initialize an empty repository and then clone the existing repository.
   - *Method:*
     ```rust
     fn clone(url: &str) -> Result<(), CloneError>;
     ```

   - *Input:* `'url: &str'` - the URL of the repository to be cloned.
   - *Output:*
     - `Ok()`: The cloned repository is created and nothing has to be returned.
     - `Err(CloneError)`: Clone failed.

**Tests:**

1. **Successful initialization Test**
    ```rust
    //Test ID: 1
    let repository_name = "test_repo_1";
    assert!(init_module::init(repository_name).is_ok());
    ```
2. **Failed Initialization - Duplicate repository name**
    ```rust
    //Test ID: 2
    let repository_name = "test_repo_1";
    assert!(init_module::init(repository_name).is_ok());

    let existing_repo_name = "test_repo_1";
    assert!(init_module::init(existing_repo_name).is_err());
    ```
3. **Failed Initialization - Invalid repository name**
    ```rust
    // Test ID: 3
    let invalid_repo_name = "-392';.3d";
    assert!(init_module::init(invalid_repo_name).is_err());
    ```
4. **Successful Cloning**
    ```rust
    // Test ID: 4
    let repo_url = "https://example.com/repo.dvcs";
    assert!(init_module::clone(repo_url).is_ok());
    ```
5. **Failed Cloning - Invalid URL**
    ```rust
    // Test ID: 5
    let invalid_url = "invalid_url";
    let result = init_module::clone(invalid_url);
    assert_eq!(result, "Invalid URL Error found! Suggestion: XYZ.");
    ```
6. **Failed Cloning - Network error**
    ```rust
    let repo_url = "https://somerepo.com/repo.git";
    let result = init_module::clone(invalid_url);
    assert_eq!(result, "Network Error found! Suggestion: XYZ.");
    ```

# Module C.2 : Inspection Module

**Designer:** Demin | **Reviewer:** Helia

**Role:**
It is responsible for providing users with tools to inspect and analyze revisions within the DVCS system. Its primary role is to facilitate the examination of various aspects of revision history.

**Secret:**
- *Primary:* The core algorithms, data structures, or implementation details that are essential for the proper functioning of the facilities provided by the "Revision Inspection Module." The primary secrets involve the algorithms used to calculate and display differences between revisions for the "Diff" facility, or the data structures used to manage and access revisions.
- *Secondary:* Involve optimization techniques for improving the performance of revision inspection, data caching strategies, or handling edge cases in the revision history.

**Facilities:**

1. **Heads:**
    - *Description:* This functionality allows users to view the current head, often representing different branches, and is essential for tracking the active development paths.
    - *Method:*
     ```rust
     fn check_current_head(repo_path: &str) -> enum HeadInfo {Branch(String), Error(String)};
     ```
    - *Input:* 
        - `repo_path`: target repository to check which branch or commit is currently active
    - *Output:*
        - `HeadInfo`: return the information about the current head, panic if not possible.

2. **Diff:**
    - *Description:* This functionality enables users to check the differences or changes between revisions. This helps understand the modifications that have been made to the codebase over time, aiding in the review process.
    - *Method:*
     ```rust
     fn compare_diff(file_path: &str, old_rev: &str, new_rev: &str) -> Result<String, String> {};
     ```
    - *Input:* 
        - Target file_path to compare old_rev and new_rev
    - *Output:*
        - Different content, otherwise error message.

3. **Inspect:**
    - *Description:* This functionality provides the capability to inspect a specific file within a given revision. Helps users examine the contents of a file at a particular point in the code’s history.
    - *Method:*
     ```rust
     fn inspect_file(repo_path: &str, revision: &str, file_path: &str) -> Result<String, String> {};
     ```
    - *Input:* 
        - file_path in repo_path to inspect with revision
    - *Output:*
        - It opens the repository, resolves the commit ID from the revision string, and then retrieves the content of the specified file at that commit.

4. **Checkout**
    - *Description:* This functionality allows users to check out a specific revision using its hash code. This is essential for reverting to a previous state of the codebase or for creating a new branch from a specific point in the revision history.
    - *Method:*
     ```rust
     fn checkout_revision(repo_path: &str, revision_hash: &str) -> Result<(), String> {};
     ```
    - *Input:* 
        - target `repo_path` to access file version with revision_hash
    - *Output:*
        - specific file version, otherwise panic

**Tests:**

1. **Head Success**
    ```rust
   // Test ID: 1
   let goodrepo = SuccessRepository;
    let result = check_current_head(&goodrepo);
    match result {
            HeadInfo::Error(error_message) => assert_eq!(branch,  "head_branch")=>panic("Expected HeadInfo::Branch, got something else.")}
   ```
2. **Head Failure**
    ```rust
    // Test ID: 2
    let failingrepo = FailingRepository;
    let result = check_current_head(&failingrepo);
    match result {
        HeadInfo::Error(error_message) => assert_eq!(
                error_message, "Failed to open repository"), _ => panic!("Expected
                HeadInfo::Error, got something else."), }
    ```
3. **Diff Success**
    ```rust
    // Test ID: 3
    let result = compare_diff("HEAD^", "HEAD", Some("sample.txt"));
    match result { Ok(diff_content) => assert!(diff_content.contains("Initial content")), Err(error) => panic!("Unexpected error: {}", error), }
    ```
4. **Diff Success - Same**
    ```rust
    // Test ID: 4
    let result = compare_diff("HEAD", "HEAD", Some("sample.txt"));
    match result { Ok(diff_content) => assert!(diff_content.contains("Initial content")), Err(error) => panic!("Unexpected error: {}", error), }
    ```
5. **Inspect Existing File**
    ```rust
    // Test ID: 5
    let repo_path = goodRepo;
    let result = inspect_file(repo_path, "HEAD", "sample.txt");
    match result { Ok(content) => assert_eq!(content, "Initial content\n"), Err(error) => panic!("Unexpected error:{}", error), }
    ```
6. **Inspect Non-existing File**
    ```rust
    // Test ID: 6
    let repo_path = goodRepo;
    let result = inspect_file(repo_path, "HEAD", "non_exist.txt");
    match result { Ok(content) => assert_eq!(content, "Initial content\n"), Err(error) => panic!("Unexpected error:{}", error), }
    ```
7. **Checkout Valid Hash**
    ```rust
    // Test ID: 7
    let repo_path = goodRepo;
    valid_hash = "0123456789abcdef0123456789abcdef01234567";
    let result = checkout_revision(repo_path, valid_hash);
    assert!(result.is_ok());
    ```
8. **Checkout Invalid Hash**
    ```rust
    // Test ID: 8
    let repo_path = goodRepo;
    invalid_hash = "0123456789";
    let result = checkout_revision(repo_path, invalid_hash);
    assert!(result.is_ok());
    ```

# Module C.3: Synchronization Module
**Designer:** Pranay | **Reviewer:** Helia

**Role:**
The Synchronization Module is responsible for executing and managing updates within the DVCS. It focuses on applying commits, merging various versions, and connecting with remote repositories to maintain an updated and consistent code base across all platforms.

**Secrets:**
- *Primary:* The processes involved in committing changes, merging versions, and synchronizing with remote repositories are abstracted. This module encapsulates algorithms and logic that execute these tasks.

- *Secondary:* Conceals methodologies and interface implementations concerning complicated operations like conflict resolution, maintaining code integrity after a pull or push, etc.

**Facilities:**
1. **Merge**
   - *Description:* Combines two different code revisions, resolving any conflicts to establish a unified code base.
   - *Method:*
     ```rust
     fn merge(revision_a: &Revision, revision_b: &Revision) -> Result<UnifiedRevision, MergeError>;
     ```
   - *Input:*
     - `revision_a`: The first code revision.
     - `revision_b`: The second code revision.
   - *Output:*
     - `Result<UnifiedRevision, MergeError>`: A unified code revision or an error if the merge process encounters issues.

2. **Pull**
   - *Description:* Retrieves and integrates updates from a remote repository, ensuring local repositories are current.
   - *Method:*
     ```rust
     fn pull(remote_repository: &RemoteRepository) -> Result<(), PullError>;
     ```
   - *Input:*
     - `remote_repository`: The remote repository to pull updates from.
   - *Output:*
     - `Result<(), PullError>`: Success or an error if the pull process encounters issues.

3. **Push**
   - *Description:* Uploads local repository changes to a remote repository, ensuring it is updated with the latest revisions.
   - *Method:*
     ```rust
     fn push(local_repository: &LocalRepository, remote_repository: &RemoteRepository) -> Result<(), PushError>;
     ```
   - *Input:*
     - `local_repository`: The local repository with changes to push.
     - `remote_repository`: The remote repository to push changes to.
   - *Output:*
     - `Result<(), PushError>`: Success or an error if the push process encounters issues.

**Tests:**

1. **Merge Test**
   ```rust
   // Test ID: 1
   let revision_a = create_revision("Code Revision A");
   let revision_b = create_revision("Code Revision B");
   let result = synchronization_module.merge(&revision_a, &revision_b);
   assert_eq!(result.is_ok(), true);
   ```

2. **Pull Test**
   ```rust
   // Test ID: 2
   let remote_repo = create_remote_repo("https://example.com/repo.git");
   let result = synchronization_module.pull(&remote_repo);
   assert_eq!(result.is_ok(), true);
   ```

3. **Push Test**
   ```rust
   // Test ID: 3
   let local_repo = create_local_repo("path/to/local/repo");
   let remote_repo = create_remote_repo("https://example.com/repo.git");
   let result = synchronization_module.push(&local_repo, &remote_repo);
   assert_eq!(result.is_ok(), true);
   ```

4. **Merge Conflict Test**
   ```rust
   // Test ID: 4
   let revision_a = create_revision("Code Revision A");
   let conflicting_revision_b = create_revision_with_conflict("Code Revision B");
   let result = synchronization_module.merge(&revision_a, &conflicting_revision_b);
   assert_eq!(result.is_err(), true);
   ```

5. **Pull Error Test**
   ```rust
   // Test ID: 5
   let invalid_remote_repo = create_remote_repo("https://invalid.example.com/repo.git");
   let result = synchronization_module.pull(&invalid_remote_repo);
   assert_eq!(result.is_err(), true);
   ```

6. **Push Error Test**
   ```rust
   // Test ID: 6
   let invalid_remote_repo = create_remote_repo("https://invalid.example.com/repo.git");
   let local_repo = create_local_repo("path/to/local/repo");
   let result = synchronization_module.push(&local_repo, &invalid_remote_repo);
   assert_eq!(result.is_err(), true);
   ```

The Synchronization Module's methods and tests are designed to ensure efficient management of synchronization operations while handling various scenarios, including merges, pulls, pushes, conflicts, and error cases. The reviewer, Helia, should focus on validating the correctness and effectiveness of these functionalities.


# Module C.4: Staging Module
**Designer:** Suumil | **Reviewer:** Shunzhi

**Role:**
The Staging Module is responsible for the management and update of the user’s current revision. It maintains a list of all tracked/untracked files in the user’s current revision, allowing the user to add a specific file they want to store/track to the list and remove a particular file they no longer want to store/track from the list. Additionally, it enables the user to check the current status of the tracking list to see what has been tracked or untracked. The module is also involved in committing changes.

**Secrets:**
- *Primary:* The Staging Module conceals the processes involved in committing changes and hides the internal storage and data structures, updating mechanisms, and file tracking. This module encapsulates algorithms and logic that execute these tasks.

- *Secondary:* The module conceals methodologies and interface implementations concerning operations like opening an editor for commit and low-level algorithms regarding updating and maintaining the tracking list.

**Facilities:**

1. **Commit:**
    - *Description:* Records changes made to the code, creating an updated revision in the repository.
    - *Method:*
     ```rust
     fn commit(message: &str) -> Result<(), CommitError>;
     ```
    - *Input:* 
        - `message`: A message describing the changes made in the commit.
    - *Output:*
        - `Result<(), CommitError>`: Success or an error if the commit pro- cess encounters issues.

2. **Add/Remove**
    - *Description:* Allows a user to add/remove a specific local file to/from the tracking list.
    - *Method:*
     ```rust
     fn add(file_path: &str) -> Result<(), TrackingError>;
     fn remove(file_path: &str) -> Result<(), TrackingError>;
     ```
    - *Input:* 
        - `file_path`: The path of the file to add/remove to/from the tracking list
    - *Output:*
        - `Result<(), TrackingError>`: Success or an error if the add/remove process encounters issues.

3. **Status**
    - *Description:* Displays the content of the present tracking list of the current revision/repository to the user as requested, containing all the IDs of the tracked and untracked files.
    - *Method:*
     ```rust
     fn status() -> Vec<FileStatus>;
     ```
    - *Input:* 
        - None
    - *Output:*
        - `Vec<FileStatus>`: A list containing the status of all tracked and untracked files in the current revision/repository.

**Tests:**

1. **Commit Success**
    ```rust
   // Test ID: 1
    let staging_module = StagingModule::new(); 
    let message = "Commit message";
    let result = staging_module.commit(message); assert_eq!(result.is_ok(), true);
    ```
2. **Commit Failure**
    ```rust
    // Test ID: 2
    let staging_module = StagingModule::new(); 
    let message = "Invalid message";
    let result = staging_module.commit(message); assert_eq!(result.is_err(), true);
    ```
3. **Add Success**
    ```rust
    // Test ID: 3
    let staging_module = StagingModule::new(); 
    let file_path = "path/to/file.txt";
    let result = staging_module.add(file_path); assert_eq!(result.is_ok(), true);
    ```
4. **Add Failure**
    ```rust
    // Test ID: 4
    let staging_module = StagingModule::new();
    let invalid_file_path = "non_existent_file.txt"; 
    let result = staging_module.add(invalid_file_path); 
    assert_eq!(result.is_err(), true);
    ```
5. **Remove Success**
    ```rust
    // Test ID: 5
    let staging_module = StagingModule::new();
    let file_path = "path/to/file.txt";
    let result = staging_module.remove(file_path); assert_eq!(result.is_ok(), true);
    ```
6. **Remove Failure**
    ```rust
    // Test ID: 6
    let staging_module = StagingModule::new();
    let invalid_file_path = "non_existent_file.txt";
    let result = staging_module.remove(invalid_file_path); 
    assert_eq!(result.is_err(), true);
    ```
7. **Status**
    ```rust
    // Test ID: 7
    let staging_module = StagingModule::new(); 
    let status = staging_module.status(); 
    assert_eq!(status.len(), 3);
    ```
#    
# <p align="center">Part D</p>
# 1. Uses Relation

## Module A: Machine Hiding

### A.1: File System Operations Module
**Uses:**
- Possibly interacts with the Network Operations Module (A.2) for handling file operations that involve network communication.

### A.2: Network Operations Module
**Uses:**
- File System Operations Module (A.1) for any file system-related operations during network communication.

## Module B: User Hiding

### B.1: Command Parser Module
**Uses:**
- May use error handling utilities from the Syntax Checker & Error Handling Module (B.2) for parsing errors.
- Interacts with User Interface Module (B.3) to receive input and display parsing results.

### B.2: Syntax Checker & Error Handling Module
**Uses:**
- Command Parser Module (B.1) to check the syntax of parsed commands.
- User Interface Module (B.3) to display error messages.

### B.3: User Interface Module
**Uses:**
- Command Parser Module (B.1) and Syntax Checker & Error Handling Module (B.2) to get command inputs and error information.
- Authentication Manager Module (B.4) for displaying authentication-related prompts or messages.

### B.4: Authentication Manager Module
**Uses:**
- Could interact with the User Interface Module (B.3) for prompting user authentication.
- Might use data from the Repository Hiding modules (Module C) for verifying user permissions on specific repositories.

## Module C: Repository Hiding

### C.1: Initialization Module
**Uses:**
- Possibly interacts with Machine Hiding modules (Module A) for file system operations during initialization and cloning.
- Might use User Interface Module (B.3) for displaying initialization or cloning results.

### C.2: Inspection Module
**Uses:**
- Depends on the Initialization Module (C.1) for accessing repository data.
- May use the User Interface Module (B.3) for outputting inspection results.

### C.3: Synchronization Module
**Uses:**
- Network Operations Module (A.2) for pull and push operations involving remote repositories.
- Staging Module (C.4) for preparing and applying changes during merge operations.

### C.4: Staging Module
**Uses:**
- Interacts with the User Interface Module (B.3) for user inputs and status updates.
- Might depend on the Initialization Module (C.1) for repository-specific operations.

# 2. Minimal Prototype System

The prototype focuses on basic functionalities that form the foundation of the DVCS. These include the following:

## Module A (Machine Hiding):
- **File System Operations Module (A.1):** Basic operations like handling cross-platform file system conflicts.
- **Network Operations Module (A.2):** Essential network operations for communication between machines.

## Module B (User Hiding):
- **Command Parser Module (B.1):** Basic command parsing for user inputs.
- **User Interface Module (B.3):** Minimal UI for user interaction.

## Module C (Repository Hiding):
- **Initialization Module (C.1):** Basic repository initialization and cloning functionalities.

These core functionalities ensure that the system can initialize, manage basic user commands, and handle machine-specific operations.

# 3. Functionality Increment 1

The first increment adds more sophisticated features for handling errors, user authentication, and initial repository operations:

## Module B (User Hiding):
- **Syntax Checker & Error Handling Module (B.2):** Enhanced error detection and handling.
- **Authentication Manager Module (B.4):** Basic user authentication for DVCS operations.

## Module C (Repository Hiding):
- **Inspection Module (C.2):** Basic functionalities for inspecting and analyzing revisions.

# 4. Functionality Increment 2

The second increment introduces advanced synchronization and staging features, along with enhanced machine-specific functionalities:

## Module A (Machine Hiding):
- **Enhancements to the Network Operations Module (A.2):** Advanced features for cross-machine data flow management and encryption.
## Module B (User Hiding):
- **User Interface Module (B.3):** Complex UI for merge conflicts.
## Module C (Repository Hiding):
- **Synchronization Module (C.3):** Advanced synchronization operations like merge, pull, and push.
- **Staging Module (C.4):** Managing changes and updates in user’s current revision.

# 5. Subsequent Increments

Further increments will focus on expanding the capabilities of the existing modules, enhancing user experience, security,providing more robust error handling and network operation features. 
