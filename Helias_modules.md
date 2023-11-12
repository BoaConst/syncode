# Module 2.2: Syntax Checker & Error Handling Module

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

- **Test ID 1:** No error detected by `detect_errors()`
    ```rust
    let user_input = "some user command";
    let result = error_handling_module::detect_errors(user_input);
    assert_eq!(result, Ok(None));
    ```
- **Test ID 2:** Syntax error detected by `detect_errors()`
    ```rust
    let user_input = "invalid syntax";
    let result = error_handling_module::detect_errors(user_input);
    assert_eq!(result, Ok(Some(UserInputError::SyntaxError)));
    ```
- **Test ID 3:** Semantic error detected by `detect_errors()`
    ```rust
    let user_input = "semantic error";
    let result = error_handling_module::detect_errors(user_input);
    assert_eq!(result, Ok(Some(UserInputError::SemanticError)));
    ```
- **Test ID 4:** Unexpected/unknown error in `detect_errors()`
    ```rust
    let user_input = "unexpected/unknown error";
    let result = error_handling_module::detect_errors(user_input);
    assert_eq!(result, Err(ErrorDetectionError));
    ```
- **Test ID 5:** Testing error message format for `SyntaxError` in user input
    ```rust
    let error = UserInputError::SyntaxError;
    let result = error_handling_module::show_error_message(&error);
    assert_eq!(result, "Syntax Error ABC found! Suggestion: XYZ.");
    ```
- **Test ID 6:** Testing error message format for `SemanticError` in user input
    ```rust
    let error = UserInputError::SemanticError;
    let result = error_handling_module::show_error_message(&error);
    assert_eq!(result, "Semantic Error ABC found! Suggestion: XYZ.");
    ```

# Module 3.1 : Initialization Module

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

- **Test ID 1:** Successful initialization
    ```rust
    let repository_name = "test_repo_1";
    assert!(init_module::init(repository_name).is_ok());
    ```
- **Test ID 2:** Failed Initialization due to duplicate repository name
    ```rust
    let repository_name = "test_repo_1";
    assert!(init_module::init(repository_name).is_ok());

    let existing_repo_name = "test_repo_1";
    assert!(init_module::init(existing_repo_name).is_err());
    ```
- **Test ID 3:** Failed Initialization due to an invalid repository name
    ```rust
    let invalid_repo_name = "-392';.3d";
    assert!(init_module::init(invalid_repo_name).is_err());
    ```
- **Test ID 4:** Successful Cloning
    ```rust
    let repo_url = "https://example.com/repo.dvcs";
    assert!(init_module::clone(repo_url).is_ok());
    ```
- **Test ID 5:** Failed Cloning due to an Invalid URL
    ```rust
    let invalid_url = "invalid_url";
    let result = init_module::clone(invalid_url);
    assert_eq!(result, "Invalid URL Error found! Suggestion: XYZ.");
    ```
- **Test ID 6:** Failed Cloning due to a network error
    ```rust
    let repo_url = "https://somerepo.com/repo.git";
    let result = init_module::clone(invalid_url);
    assert_eq!(result, "Network Error found! Suggestion: XYZ.");
    ```
