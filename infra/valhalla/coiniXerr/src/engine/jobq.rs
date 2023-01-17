






/////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
///////        a job or async task queue like tokio channels and the one inside the rabbitmq actors from scrach using the worker threadpools  
/////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈


// ➔ vector of || async move{} of events for an event manager struct like riker scheduling logic and vector clock schemas and call new event every 5 seconds from vector of event of closures
// ➔ every task or batch of tasks from the queue must be solved inside a thread when the receiver is receiving them by awaiting on each iteration and also we have to avoid race condition using Mutex
// ➔ tasks can be scheduled at a specific time also they can be broadcasted to a channel
// ➔ rabbitmq will be used between two services like vps-es in a pub/sub manner and is based on actor design pattern which uses async task or jobq channel algos like celery algos to share and solve tasks between its actors 
// ➔ task queue will be used to manage tasks from the queue inside a free thread selected from the worker threadpool and also it can be used to broadcast and schedule tasks in a pub/sub manner using tokio mpsc channels
// ➔ actors use task or job queue channels under the hood like celery which is based on a prod/cons or pub/sub manner to prod tasks and cons tasks from the queue to solve them or schedule them to be executed later by sharing them between threads of the worker threadpool using mpsc channel
// ➔ celery will be used for producing and consuming async tasks with a distributed message queues (the one that being used inside the rabbitmq)
// ...

// https://github.com/codepr/tasq
// https://dev.to/zeroassumptions/build-a-job-queue-with-rust-using-aide-de-camp-part-1-4g5m
// https://poor.dev/blog/what-job-queue/
// https://cetra3.github.io/blog/implementing-a-jobq/
// https://rodent.club/queue-manager/
// https://cetra3.github.io/blog/implementing-a-jobq-with-tokio/
// https://tokio.rs/tokio/tutorial/channels




/* 
    
        TOKIO JOBQ CHANNLE ALGORITHMS

    mpsc: multi-producer, single-consumer channel. Many values can be sent.
    oneshot: single-producer, single consumer channel. A single value can be sent.
    broadcast: multi-producer, multi-consumer. Many values can be sent. Each receiver sees every value.
    watch: single-producer, multi-consumer. Many values can be sent, but no history is kept. Receivers only see the most recent value.

*/


use crate::*;







pub type Task = Job; //// the type of the Task is of type Job structure


pub struct Job{ // the job that must be received by the receiver
    pub id: Uuid,
    pub task: Box<dyn FnOnce() + Send + Sync + 'static>, //// the task that can be shared between worker threadpool for solving
} 

pub struct Queue{ // a queue which contains all the incoming jobs from the sender 
    pub tasks: Vec<Task>,   
}

pub struct JobHandler; // a threadpool structure to handle the poped-out job from the queue
