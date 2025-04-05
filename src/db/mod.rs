pub mod job_applications;
// pub use job_applications::insert_job_application;

pub mod connect;

pub use connect::connect_to_db; // 👈 this line exposes the function
