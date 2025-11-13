use serde::Serialize;

pub enum ClientError {
    AuthError(String),
    InitError(String),
    UsageError(String),
    InvalidRequestError,
}

#[derive(Serialize, Debug)]
struct NonEncryptedErrorMessage {
    response_type: String,
    error_type: String,
    error_message: String,
}

pub fn construct_client_error_message(err: ClientError) -> String {
    match err {
        ClientError::AuthError(message) => {
            println!("Auth error: {}", message);
            let error_message = NonEncryptedErrorMessage {
                response_type: "Error".to_string(),
                error_type: "Auth".to_string(),
                error_message: "Something went wrong during the authentication process, please try again later.".to_string()
            };

            serde_json::to_string(&error_message)
                .unwrap_or("Cyborg Agent ecountered an unrecoverable error, please try again later.".to_string())
        }
        ClientError::UsageError(message) => {
            println!("Usage error: {}", message);
            let error_message = NonEncryptedErrorMessage {
                response_type: "Error".to_string(),
                error_type: "Usage".to_string(),
                error_message: "Something went wrong while streaming the usage, please try again later.".to_string()
            };

            serde_json::to_string(&error_message)
                .unwrap_or("Cyborg Agent ecountered an unrecoverable error, please try again later.".to_string())
        }
        ClientError::InitError(message) => {
            println!("Init error: {}", message);
            let error_message = NonEncryptedErrorMessage {
                response_type: "Error".to_string(),
                error_type: "Init".to_string(),
                error_message: "Something went wrong fetching the nodes specs, please try again later.".to_string()
            };

            serde_json::to_string(&error_message)
                .unwrap_or("Cyborg Agent ecountered an unrecoverable error, please try again later.".to_string())
        }
        ClientError::InvalidRequestError => {
            println!("Invalid request error");
            let error_message = NonEncryptedErrorMessage {
                response_type: "Error".to_string(),
                error_type: "InvalidRequest".to_string(),
                error_message: "Cyborg Agent is not able to process requests of this format.".to_string()
            };

            serde_json::to_string(&error_message)
                .unwrap_or("Cyborg Agent ecountered an unrecoverable error, please try again later.".to_string())
        },
    }
}