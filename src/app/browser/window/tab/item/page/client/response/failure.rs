pub enum Failure {
    Status { message: String },
    Mime { message: String },
    Error { message: String },
}
