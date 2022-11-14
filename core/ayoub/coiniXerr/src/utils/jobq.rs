




// a job or async task queue like mpsc and the one inside the rabbitmq from scrach using the scheduler threadpools
// every task or batch of tasks from the queue must be solved inside a thread when the receiver is receiving them by awaiting on each iteration and also we have to avoid race condition 
// ...

// https://dev.to/zeroassumptions/build-a-job-queue-with-rust-using-aide-de-camp-part-1-4g5m
// https://poor.dev/blog/what-job-queue/
// https://rodent.club/queue-manager/
// https://cetra3.github.io/blog/implementing-a-jobq-with-tokio/



use crate::*;







pub type Task = Job; //// the type of the Task is of type Job structure


pub struct Job; // the job that must be received by the receiver

pub struct Queue; // a queue which contains all the incoming jobs from the sender 

pub struct JobHandler; // a threadpool structure to handle the poped-out job from the queue