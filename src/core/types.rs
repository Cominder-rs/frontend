use tonic_web_wasm_client::Client;

use users_proto::users_client::UsersClient as _UsersClient;

pub type UsersClient = _UsersClient<Client>;

pub type Response<T> = Result<tonic::Response<T>, tonic::Status>;